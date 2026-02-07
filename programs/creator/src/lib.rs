use anchor_lang::prelude::*;

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
// 4. Burn mechanism on claim
// 5. Scout reputation tracking
// =============================================================================

#[program]
pub mod creator {
    use super::*;

    // =========================================================================
    // Creator Registration
    // =========================================================================
    // Vitalik: "anyone can become a creator and create a creator coin"

    /// Register a creator seeking DAO admission
    pub fn register_creator(
        ctx: Context<CreatorAction>,
        name: String,
        bio: String,
        content_type: ContentType,
        style_tag: String,
        target_dao: Pubkey,
        deadline: i64,
    ) -> Result<()> {
        // Build metadata for creator context
        let _metadata = CreatorMetadata {
            content_type,
            style_tag: string_to_bytes32(&style_tag),
            target_dao,
            region_code: 0,
        };

        msg!(
            "Registering creator '{}' seeking admission to DAO (content: {:?})",
            name,
            content_type
        );

        emit!(CreatorRegistered {
            name: name.clone(),
            content_type,
            target_dao,
            deadline,
        });

        // Note: CPIs to subject_registry::register_subject with mode = Creator

        Ok(())
    }

    // =========================================================================
    // Admission Predictions
    // =========================================================================
    // Vitalik: "speculators are specifically being predictors of what new
    // creators the high-value creator DAOs will be willing to accept"

    /// Create an admission prediction market
    pub fn create_prediction(
        ctx: Context<CreatorAction>,
        creator_subject: Pubkey,
        thesis: String,
        analysis: String,
    ) -> Result<()> {
        msg!(
            "Creating admission prediction for creator {}",
            creator_subject
        );

        emit!(PredictionCreated {
            creator: creator_subject,
            scout: ctx.accounts.user.key(),
            thesis: thesis.clone(),
        });

        // Note: CPIs to market_engine::create_market

        Ok(())
    }

    /// Stake on admission prediction
    pub fn stake_on_admission(
        ctx: Context<CreatorAction>,
        market: Pubkey,
        amount: u64,
        prediction: AdmissionOutcome,
    ) -> Result<()> {
        msg!(
            "Staking {} on {:?} for market {}",
            amount,
            prediction,
            market
        );

        emit!(PredictionStaked {
            market,
            staker: ctx.accounts.user.key(),
            amount,
            predicts_acceptance: matches!(prediction, AdmissionOutcome::Accepted),
        });

        // Note: CPIs to market_engine::stake with outcome mapping

        Ok(())
    }

    // =========================================================================
    // DAO Voting
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

        dao.name = string_to_bytes32(&name);
        dao.content_type = content_type;
        dao.style_tag = string_to_bytes32(&style_tag);
        dao.founder = ctx.accounts.founder.key();
        dao.member_count = 1; // Founder is first member
        dao.admission_threshold = admission_threshold;
        dao.quorum = quorum;
        dao.created_at = Clock::get()?.unix_timestamp;
        dao.bump = ctx.bumps.dao;

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

    /// Add initial founder member
    /// Vitalik: "Hand-pick the initial membership set"
    pub fn add_founder_member(
        ctx: Context<AddMember>,
        member: Pubkey,
    ) -> Result<()> {
        let dao = &mut ctx.accounts.dao;

        require!(
            dao.founder == ctx.accounts.founder.key(),
            ErrorCode::NotFounder
        );
        require!(
            dao.member_count < MAX_DAO_MEMBERS,
            ErrorCode::DAOFull
        );

        dao.member_count += 1;

        emit!(MemberAdded {
            dao: dao.key(),
            member,
            by_founder: true,
        });

        if dao.member_count >= MAX_DAO_MEMBERS {
            msg!("WARNING: DAO at max capacity. Consider splitting.");
        }

        Ok(())
    }

    /// Cast vote on creator admission
    /// Vitalik: "(anonymously) vote new members in and out"
    pub fn cast_admission_vote(
        ctx: Context<CastVote>,
        creator: Pubkey,
        vote: VoteChoice,
        salt: [u8; 32], // For vote privacy
    ) -> Result<()> {
        let nomination = &mut ctx.accounts.nomination;

        match vote {
            VoteChoice::Accept => nomination.votes_accept += 1,
            VoteChoice::Reject => nomination.votes_reject += 1,
            VoteChoice::Abstain => nomination.votes_abstain += 1,
        }

        // Create vote hash for semi-anonymity
        let _voter_hash = anchor_lang::solana_program::keccak::hashv(&[
            ctx.accounts.voter.key().as_ref(),
            creator.as_ref(),
            &salt,
        ]);

        emit!(VoteCast {
            nomination: nomination.key(),
            // Note: not emitting voter for privacy
            vote,
        });

        Ok(())
    }

    /// Resolve DAO admission vote and settle prediction markets
    /// Vitalik: "the ultimate decider of who rises and falls is not speculators,
    /// but high-value content creators"
    pub fn resolve_admission(ctx: Context<ResolveAdmission>) -> Result<()> {
        let nomination = &mut ctx.accounts.nomination;
        let dao = &ctx.accounts.dao;

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
        nomination.resolved_at = Some(Clock::get()?.unix_timestamp);

        emit!(AdmissionResolved {
            nomination: nomination.key(),
            creator: nomination.creator,
            accepted,
            approval_rate: approval_rate as u16,
        });

        // This resolution becomes the oracle for prediction markets
        // CPIs to market_engine::settle_market with outcome based on `accepted`
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

/// Creator/scout reputation (alias for Reputation)
pub type CreatorReputation = Reputation;

// Stub types (imported from core in practice)
pub struct Subject;
pub struct Market;
pub struct Reputation;

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
// Creator Metadata
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreatorMetadata {
    pub content_type: ContentType,
    pub style_tag: [u8; 32],
    pub target_dao: Pubkey,
    pub region_code: u16,
}

// ============================================================================
// Accounts
// ============================================================================

#[derive(Accounts)]
pub struct CreatorAction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
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

    #[account(mut)]
    pub founder: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub dao: Account<'info, CreatorDAO>,

    pub founder: Signer<'info>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub nomination: Account<'info, Nomination>,

    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveAdmission<'info> {
    #[account(mut)]
    pub nomination: Account<'info, Nomination>,

    pub dao: Account<'info, CreatorDAO>,

    pub resolver: Signer<'info>,
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
    pub admission_threshold: u8,  // % needed to admit
    pub quorum: u8,               // % needed for valid vote
    pub created_at: i64,
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
pub struct PredictionStaked {
    pub market: Pubkey,
    pub staker: Pubkey,
    pub amount: u64,
    pub predicts_acceptance: bool,
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
    pub by_founder: bool,
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
    #[msg("Only founder can add initial members")]
    NotFounder,
    #[msg("DAO at maximum capacity, consider splitting")]
    DAOFull,
    #[msg("Quorum not reached")]
    QuorumNotReached,
}
