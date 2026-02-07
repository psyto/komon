use anchor_lang::prelude::*;
use subject_registry::cpi::accounts::RegisterSubject;
use subject_registry::cpi::register_subject;
use subject_registry::program::SubjectRegistry;
use subject_registry::{Subject, SubjectMetadata};
use market_engine::cpi::accounts::{CreateMarket, SettleMarket};
use market_engine::cpi::{create_market, settle_market};
use market_engine::program::MarketEngine;
use market_engine::{Market, Outcome};

declare_id!("8iPZJoBCPGEALzyiUwMYc4FyKW8QbNnBNuhAjBSJvUno");

// =============================================================================
// CREATOR FRAMING LAYER
// =============================================================================
//
// Thin wrapper implementing Vitalik's "How I would do creator coins" model:
// https://vitalik.eth.limo/general/2025/01/23/creatorcoins.html
//
// Vocabulary mapping:
// - Subject → Creator (seeking DAO admission)
// - Market → Admission Prediction
// - Outcome::Yes → DAO accepts creator
// - Outcome::No → DAO rejects creator
//
// This layer handles:
// 1. Type aliases for creator terminology
// 2. Content type enums (writing, music, video...)
// 3. DAO vote-based resolution (not authority)
// 4. Burn mechanism on claim (via market-engine)
// 5. Scout reputation tracking
// =============================================================================

#[program]
pub mod creator {
    use super::*;

    // =========================================================================
    // Creator Registration
    // =========================================================================
    // Vitalik: "anyone can become a creator and create a creator coin"

    /// Register a creator seeking DAO admission (CPI to subject_registry)
    pub fn register_creator(
        ctx: Context<RegisterCreator>,
        name: String,
        bio: String,
        content_type: ContentType,
        style_tag: String,
        target_dao: Pubkey,
        deadline: i64,
    ) -> Result<()> {
        // Build metadata for creator context
        // context_a/b store DAO reference parts, category = content_type
        let metadata = SubjectMetadata {
            context_a: 0, // Reserved
            context_b: 0, // Reserved
            category: content_type as u8,
            flags: 0,
            reference: Some(target_dao),
        };

        msg!(
            "Registering creator '{}' seeking admission to DAO (content: {:?})",
            name,
            content_type
        );

        // CPI to subject_registry::register_subject
        let cpi_program = ctx.accounts.subject_registry_program.to_account_info();
        let cpi_accounts = RegisterSubject {
            config: ctx.accounts.subject_config.to_account_info(),
            subject: ctx.accounts.creator_subject.to_account_info(),
            registrar: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        register_subject(cpi_ctx, name.clone(), bio, metadata, deadline)?;

        emit!(CreatorRegistered {
            name,
            content_type,
            target_dao,
            deadline,
        });

        Ok(())
    }

    // =========================================================================
    // Admission Predictions
    // =========================================================================
    // Vitalik: "speculators are specifically being predictors of what new
    // creators the high-value creator DAOs will be willing to accept"

    /// Create an admission prediction market (CPI to market_engine)
    pub fn create_prediction(
        ctx: Context<CreatePrediction>,
        thesis: String,
        analysis: String,
    ) -> Result<()> {
        let creator_subject = ctx.accounts.creator_subject.key();

        msg!(
            "Creating admission prediction for creator {}",
            creator_subject
        );

        // CPI to market_engine::create_market
        let cpi_program = ctx.accounts.market_engine_program.to_account_info();
        let cpi_accounts = CreateMarket {
            config: ctx.accounts.market_config.to_account_info(),
            subject: ctx.accounts.creator_subject.to_account_info(),
            market: ctx.accounts.prediction_market.to_account_info(),
            yes_mint: ctx.accounts.yes_mint.to_account_info(),
            no_mint: ctx.accounts.no_mint.to_account_info(),
            creator: ctx.accounts.scout.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        create_market(cpi_ctx, thesis.clone(), analysis)?;

        emit!(PredictionCreated {
            creator: creator_subject,
            scout: ctx.accounts.scout.key(),
            thesis,
        });

        Ok(())
    }

    // =========================================================================
    // DAO Management
    // =========================================================================
    // Vitalik: "there are N members, and they can (anonymously) vote new
    // members in and out"

    /// Create a DAO for creator curation
    /// Vitalik: "Hand-pick the initial membership set"
    pub fn create_dao(
        ctx: Context<CreateDAO>,
        name: String,
        content_type: ContentType,
        style_tag: String,
        admission_threshold: u8,
        quorum: u8,
    ) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        let founder_membership = &mut ctx.accounts.founder_membership;
        let now = Clock::get()?.unix_timestamp;

        dao.name = string_to_bytes32(&name);
        dao.content_type = content_type;
        dao.style_tag = string_to_bytes32(&style_tag);
        dao.founder = ctx.accounts.founder.key();
        dao.member_count = 1; // Founder is first member
        dao.admission_threshold = admission_threshold;
        dao.quorum = quorum;
        dao.created_at = now;
        dao.bump = ctx.bumps.dao;

        // Initialize founder's membership
        founder_membership.dao = dao.key();
        founder_membership.member = ctx.accounts.founder.key();
        founder_membership.added_by = ctx.accounts.founder.key();
        founder_membership.joined_at = now;
        founder_membership.is_active = true;
        founder_membership.bump = ctx.bumps.founder_membership;

        emit!(DAOCreated {
            name: name.clone(),
            content_type,
            founder: dao.founder,
        });

        // Vitalik: "Do NOT try to make the DAO universal... embrace the opinionatedness"
        msg!(
            "Created opinionated DAO '{}' for {:?} content",
            name,
            content_type
        );

        Ok(())
    }

    /// Vitalik: "If N gets above ~200, consider auto-splitting it"
    pub const MAX_DAO_MEMBERS: u16 = 200;

    /// Add a member to the DAO
    /// During initial phase, only founder can add; later, members vote in new members
    pub fn add_member(
        ctx: Context<AddMember>,
        new_member: Pubkey,
    ) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        let membership = &mut ctx.accounts.membership;

        // Verify caller is a member
        require!(
            ctx.accounts.caller_membership.dao == dao.key() && ctx.accounts.caller_membership.is_active,
            ErrorCode::NotAMember
        );

        require!(
            dao.member_count < MAX_DAO_MEMBERS,
            ErrorCode::DAOFull
        );

        // Initialize new membership
        membership.dao = dao.key();
        membership.member = new_member;
        membership.added_by = ctx.accounts.caller.key();
        membership.joined_at = Clock::get()?.unix_timestamp;
        membership.is_active = true;
        membership.bump = ctx.bumps.membership;

        dao.member_count += 1;

        emit!(MemberAdded {
            dao: dao.key(),
            member: new_member,
            added_by: ctx.accounts.caller.key(),
        });

        if dao.member_count >= MAX_DAO_MEMBERS {
            msg!("WARNING: DAO at max capacity. Consider splitting.");
        }

        Ok(())
    }

    /// Create a nomination for a creator to join the DAO
    pub fn create_nomination(
        ctx: Context<CreateNomination>,
        creator: Pubkey,
        voting_period_seconds: i64,
    ) -> Result<()> {
        let nomination = &mut ctx.accounts.nomination;
        let dao = &ctx.accounts.dao;
        let now = Clock::get()?.unix_timestamp;

        // Verify nominator is a member
        require!(
            ctx.accounts.nominator_membership.dao == dao.key() && ctx.accounts.nominator_membership.is_active,
            ErrorCode::NotAMember
        );

        nomination.dao = dao.key();
        nomination.creator = creator;
        nomination.nominator = ctx.accounts.nominator.key();
        nomination.votes_accept = 0;
        nomination.votes_reject = 0;
        nomination.votes_abstain = 0;
        nomination.created_at = now;
        nomination.voting_ends_at = now + voting_period_seconds;
        nomination.is_resolved = false;
        nomination.was_accepted = false;
        nomination.resolved_at = None;
        nomination.bump = ctx.bumps.nomination;

        emit!(NominationCreated {
            dao: dao.key(),
            creator,
            nominator: ctx.accounts.nominator.key(),
            voting_ends_at: nomination.voting_ends_at,
        });

        Ok(())
    }

    /// Cast vote on creator admission
    /// Vitalik: "(anonymously) vote new members in and out"
    pub fn cast_vote(
        ctx: Context<CastVote>,
        vote: VoteChoice,
        salt: [u8; 32], // For vote privacy
    ) -> Result<()> {
        let nomination = &mut ctx.accounts.nomination;
        let vote_record = &mut ctx.accounts.vote_record;
        let now = Clock::get()?.unix_timestamp;

        // Verify voting is still open
        require!(now < nomination.voting_ends_at, ErrorCode::VotingEnded);
        require!(!nomination.is_resolved, ErrorCode::AlreadyResolved);

        // Verify voter is a member
        require!(
            ctx.accounts.voter_membership.dao == nomination.dao && ctx.accounts.voter_membership.is_active,
            ErrorCode::NotAMember
        );

        // Verify hasn't voted already
        require!(!vote_record.has_voted, ErrorCode::AlreadyVoted);

        match vote {
            VoteChoice::Accept => nomination.votes_accept += 1,
            VoteChoice::Reject => nomination.votes_reject += 1,
            VoteChoice::Abstain => nomination.votes_abstain += 1,
        }

        // Record vote (hashed for semi-anonymity)
        vote_record.nomination = nomination.key();
        vote_record.voter_hash = anchor_lang::solana_program::keccak::hashv(&[
            ctx.accounts.voter.key().as_ref(),
            nomination.creator.as_ref(),
            &salt,
        ]).0;
        vote_record.has_voted = true;
        vote_record.bump = ctx.bumps.vote_record;

        emit!(VoteCast {
            nomination: nomination.key(),
            vote,
            // Note: voter not included for privacy
        });

        Ok(())
    }

    /// Resolve DAO admission vote and settle prediction markets
    /// Vitalik: "the ultimate decider of who rises and falls is not speculators,
    /// but high-value content creators"
    pub fn resolve_admission(ctx: Context<ResolveAdmission>) -> Result<()> {
        let nomination = &mut ctx.accounts.nomination;
        let dao = &ctx.accounts.dao;
        let now = Clock::get()?.unix_timestamp;

        require!(!nomination.is_resolved, ErrorCode::AlreadyResolved);
        require!(now >= nomination.voting_ends_at, ErrorCode::VotingNotEnded);

        // Check quorum
        let total_votes = nomination.votes_accept + nomination.votes_reject + nomination.votes_abstain;
        let quorum_needed = (dao.member_count as u32 * dao.quorum as u32) / 100;
        require!(total_votes as u32 >= quorum_needed, ErrorCode::QuorumNotReached);

        // Check threshold
        let decisive_votes = nomination.votes_accept + nomination.votes_reject;
        let approval_rate = if decisive_votes > 0 {
            (nomination.votes_accept as u32 * 10000) / decisive_votes as u32
        } else {
            0
        };

        let accepted = approval_rate >= (dao.admission_threshold as u32 * 100);

        nomination.is_resolved = true;
        nomination.was_accepted = accepted;
        nomination.resolved_at = Some(now);

        // CPI to market_engine::settle_market with outcome based on `accepted`
        let outcome = if accepted { Outcome::Yes } else { Outcome::No };

        let cpi_program = ctx.accounts.market_engine_program.to_account_info();
        let cpi_accounts = SettleMarket {
            config: ctx.accounts.market_config.to_account_info(),
            market: ctx.accounts.prediction_market.to_account_info(),
            authority: ctx.accounts.resolver.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        settle_market(cpi_ctx, outcome)?;

        emit!(AdmissionResolved {
            nomination: nomination.key(),
            creator: nomination.creator,
            accepted,
            approval_rate: approval_rate as u16,
        });

        msg!(
            "Admission resolved: {} ({}% approval)",
            if accepted { "ACCEPTED" } else { "REJECTED" },
            approval_rate / 100
        );

        Ok(())
    }
}

// ============================================================================
// Type Aliases
// ============================================================================

/// A creator seeking admission (alias for Subject)
pub type Creator = Subject;

/// An admission prediction market (alias for Market)
pub type AdmissionPrediction = Market;

// ============================================================================
// Creator-Specific Enums
// ============================================================================

/// Content types for creator DAOs
/// Vitalik: "Be okay with having a dominant type of content"
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum ContentType {
    LongFormWriting,   // Essays, newsletters (like Substack)
    ShortFormWriting,  // Tweets, threads
    Music,             // Original music
    ShortFormVideo,    // TikTok, Reels
    LongFormVideo,     // YouTube, documentaries
    Fiction,           // Novels, short stories
    Educational,       // Tutorials, courses
    Podcasts,          // Audio content
    Art,               // Visual art
    Code,              // Open source
}

/// Prediction outcome in creator terms
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdmissionOutcome {
    /// DAO accepted the creator
    Accepted,
    /// DAO rejected the creator
    Rejected,
}

/// Vote choice for admission
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VoteChoice {
    Accept,
    Reject,
    Abstain,
}

// ============================================================================
// Accounts
// ============================================================================

#[derive(Accounts)]
#[instruction(name: String, bio: String)]
pub struct RegisterCreator<'info> {
    /// Subject registry config
    /// CHECK: Validated by subject_registry program
    #[account(mut)]
    pub subject_config: UncheckedAccount<'info>,

    /// The creator subject account to create
    /// CHECK: Created by subject_registry via CPI
    #[account(mut)]
    pub creator_subject: UncheckedAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub subject_registry_program: Program<'info, SubjectRegistry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePrediction<'info> {
    /// Market engine config
    /// CHECK: Validated by market_engine program
    #[account(mut)]
    pub market_config: UncheckedAccount<'info>,

    /// The creator this prediction is about
    /// CHECK: Validated by market_engine program
    pub creator_subject: UncheckedAccount<'info>,

    /// The prediction market account
    /// CHECK: Created by market_engine via CPI
    #[account(mut)]
    pub prediction_market: UncheckedAccount<'info>,

    /// YES token mint (creator accepted)
    /// CHECK: Created by market_engine via CPI
    #[account(mut)]
    pub yes_mint: UncheckedAccount<'info>,

    /// NO token mint (creator rejected)
    /// CHECK: Created by market_engine via CPI
    #[account(mut)]
    pub no_mint: UncheckedAccount<'info>,

    /// The scout creating the prediction
    #[account(mut)]
    pub scout: Signer<'info>,

    pub market_engine_program: Program<'info, MarketEngine>,
    /// CHECK: Token program
    pub token_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateDAO<'info> {
    #[account(
        init,
        payer = founder,
        space = 8 + CreatorDAO::INIT_SPACE,
        seeds = [b"creator_dao", founder.key().as_ref()],
        bump
    )]
    pub dao: Account<'info, CreatorDAO>,

    /// Founder's membership record
    #[account(
        init,
        payer = founder,
        space = 8 + DAOMembership::INIT_SPACE,
        seeds = [b"membership", dao.key().as_ref(), founder.key().as_ref()],
        bump
    )]
    pub founder_membership: Account<'info, DAOMembership>,

    #[account(mut)]
    pub founder: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub dao: Account<'info, CreatorDAO>,

    /// Caller's membership (must be active member)
    #[account(
        seeds = [b"membership", dao.key().as_ref(), caller.key().as_ref()],
        bump = caller_membership.bump
    )]
    pub caller_membership: Account<'info, DAOMembership>,

    /// New member's membership record
    #[account(
        init,
        payer = caller,
        space = 8 + DAOMembership::INIT_SPACE,
        seeds = [b"membership", dao.key().as_ref(), new_member.key().as_ref()],
        bump
    )]
    pub membership: Account<'info, DAOMembership>,

    /// CHECK: The new member's address
    pub new_member: UncheckedAccount<'info>,

    #[account(mut)]
    pub caller: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateNomination<'info> {
    pub dao: Account<'info, CreatorDAO>,

    /// Nominator's membership (must be active member)
    #[account(
        seeds = [b"membership", dao.key().as_ref(), nominator.key().as_ref()],
        bump = nominator_membership.bump
    )]
    pub nominator_membership: Account<'info, DAOMembership>,

    #[account(
        init,
        payer = nominator,
        space = 8 + Nomination::INIT_SPACE,
        seeds = [b"nomination", dao.key().as_ref(), creator.key().as_ref()],
        bump
    )]
    pub nomination: Account<'info, Nomination>,

    /// CHECK: The creator being nominated
    pub creator: UncheckedAccount<'info>,

    #[account(mut)]
    pub nominator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub nomination: Account<'info, Nomination>,

    /// Voter's membership (must be active member)
    #[account(
        seeds = [b"membership", nomination.dao.as_ref(), voter.key().as_ref()],
        bump = voter_membership.bump
    )]
    pub voter_membership: Account<'info, DAOMembership>,

    /// Vote record to prevent double voting
    #[account(
        init,
        payer = voter,
        space = 8 + VoteRecord::INIT_SPACE,
        seeds = [b"vote", nomination.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveAdmission<'info> {
    #[account(mut)]
    pub nomination: Account<'info, Nomination>,

    pub dao: Account<'info, CreatorDAO>,

    /// Market engine config
    /// CHECK: Validated by market_engine program
    #[account(mut)]
    pub market_config: UncheckedAccount<'info>,

    /// The prediction market to settle
    /// CHECK: Validated by market_engine program
    #[account(mut)]
    pub prediction_market: UncheckedAccount<'info>,

    pub resolver: Signer<'info>,

    pub market_engine_program: Program<'info, MarketEngine>,
}

// ============================================================================
// State
// ============================================================================

/// Creator DAO
/// Vitalik: "Create a DAO that is NOT token-based... N members vote"
#[account]
#[derive(InitSpace)]
pub struct CreatorDAO {
    pub name: [u8; 32],
    pub content_type: ContentType,
    pub style_tag: [u8; 32],
    pub founder: Pubkey,
    pub member_count: u16,
    pub admission_threshold: u8,  // % needed to admit (e.g., 66 = 66%)
    pub quorum: u8,               // % needed for valid vote (e.g., 50 = 50%)
    pub created_at: i64,
    pub bump: u8,
}

/// DAO Membership record
#[account]
#[derive(InitSpace)]
pub struct DAOMembership {
    pub dao: Pubkey,
    pub member: Pubkey,
    pub added_by: Pubkey,
    pub joined_at: i64,
    pub is_active: bool,
    pub bump: u8,
}

/// Nomination for DAO admission
#[account]
#[derive(InitSpace)]
pub struct Nomination {
    pub dao: Pubkey,
    pub creator: Pubkey,
    pub nominator: Pubkey,
    pub votes_accept: u16,
    pub votes_reject: u16,
    pub votes_abstain: u16,
    pub created_at: i64,
    pub voting_ends_at: i64,
    pub is_resolved: bool,
    pub was_accepted: bool,
    pub resolved_at: Option<i64>,
    pub bump: u8,
}

/// Vote record (prevents double voting, stores hash for privacy)
#[account]
#[derive(InitSpace)]
pub struct VoteRecord {
    pub nomination: Pubkey,
    pub voter_hash: [u8; 32],
    pub has_voted: bool,
    pub bump: u8,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct CreatorRegistered {
    pub name: String,
    pub content_type: ContentType,
    pub target_dao: Pubkey,
    pub deadline: i64,
}

#[event]
pub struct PredictionCreated {
    pub creator: Pubkey,
    pub scout: Pubkey,
    pub thesis: String,
}

#[event]
pub struct DAOCreated {
    pub name: String,
    pub content_type: ContentType,
    pub founder: Pubkey,
}

#[event]
pub struct MemberAdded {
    pub dao: Pubkey,
    pub member: Pubkey,
    pub added_by: Pubkey,
}

#[event]
pub struct NominationCreated {
    pub dao: Pubkey,
    pub creator: Pubkey,
    pub nominator: Pubkey,
    pub voting_ends_at: i64,
}

#[event]
pub struct VoteCast {
    pub nomination: Pubkey,
    pub vote: VoteChoice,
    // Note: voter not included for privacy
}

#[event]
pub struct AdmissionResolved {
    pub nomination: Pubkey,
    pub creator: Pubkey,
    pub accepted: bool,
    pub approval_rate: u16,
}

// ============================================================================
// Configuration Constants
// ============================================================================

/// Creator mode configuration
/// Vitalik: "a portion of their proceeds from the DAO are used to burn"
pub mod creator_config {
    /// Burn enabled in creator mode
    pub const BURN_ENABLED: bool = true;
    /// Default burn rate (5%)
    pub const BURN_RATE_BPS: u16 = 500;

    /// Resolution is DAO vote-based
    pub const RESOLUTION_TYPE: &str = "dao_vote";

    /// Default fee rate
    pub const DEFAULT_FEE_BPS: u16 = 250;

    /// Max DAO size before split
    /// Vitalik: "If N gets above ~200, consider auto-splitting it"
    pub const MAX_DAO_SIZE: u16 = 200;
}

// ============================================================================
// Helpers
// ============================================================================

fn string_to_bytes32(s: &str) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let slice = s.as_bytes();
    let len = slice.len().min(32);
    bytes[..len].copy_from_slice(&slice[..len]);
    bytes
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Not a member of the DAO")]
    NotAMember,
    #[msg("DAO at maximum capacity, consider splitting")]
    DAOFull,
    #[msg("Quorum not reached")]
    QuorumNotReached,
    #[msg("Voting period has ended")]
    VotingEnded,
    #[msg("Voting period has not ended yet")]
    VotingNotEnded,
    #[msg("Already voted on this nomination")]
    AlreadyVoted,
    #[msg("Nomination already resolved")]
    AlreadyResolved,
}
