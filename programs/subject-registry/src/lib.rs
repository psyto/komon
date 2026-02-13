use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("DSSJdN2LTG578pEFnJPr6HHP61jKhsKsmeE6ohdTQAPv");

// =============================================================================
// SUBJECT REGISTRY - Generic Core
// =============================================================================
//
// This is the shared core for prediction markets. It can be framed as:
// - Civic: "Problems" that need solutions (KOMON)
// - Creator: "Creators" seeking DAO admission (Vitalik model)
//
// The core mechanism is identical:
// 1. Someone registers a Subject
// 2. Markets form around outcomes
// 3. Resolution determines winners
// 4. Reputation tracks accuracy
// =============================================================================

#[program]
pub mod subject_registry {
    use super::*;

    /// Initialize the registry with global config
    pub fn initialize(
        ctx: Context<Initialize>,
        fee_rate: u16,
        protocol_mode: ProtocolMode,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.subject_count = 0;
        config.fee_rate = fee_rate;
        config.protocol_mode = protocol_mode;
        config.bump = ctx.bumps.config;

        msg!("Subject Registry initialized in {:?} mode", protocol_mode);
        Ok(())
    }

    /// Register a new subject (generic: could be problem or creator)
    pub fn register_subject(
        ctx: Context<RegisterSubject>,
        title: String,
        description: String,
        metadata: SubjectMetadata,
        deadline: i64,
    ) -> Result<()> {
        require!(title.len() <= 100, ErrorCode::TitleTooLong);
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        require!(deadline > Clock::get()?.unix_timestamp, ErrorCode::InvalidDeadline);

        let config = &mut ctx.accounts.config;
        let subject = &mut ctx.accounts.subject;

        subject.id = config.subject_count;
        subject.registrar = ctx.accounts.registrar.key();
        subject.title = title.clone();
        subject.description = description;
        subject.metadata = metadata;
        subject.status = SubjectStatus::Open;
        subject.pool_amount = 0;
        subject.deadline = deadline;
        subject.created_at = Clock::get()?.unix_timestamp;
        subject.resolved_at = None;
        subject.winning_outcome = None;
        subject.market_count = 0;
        subject.bump = ctx.bumps.subject;

        config.subject_count += 1;

        emit!(SubjectRegistered {
            subject_id: subject.id,
            registrar: subject.registrar,
            title,
            mode: config.protocol_mode,
        });

        msg!("Subject #{} registered", subject.id);
        Ok(())
    }

    /// Fund a subject's pool
    pub fn fund_subject(ctx: Context<FundSubject>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let subject = &mut ctx.accounts.subject;
        require!(subject.status == SubjectStatus::Open, ErrorCode::SubjectNotOpen);

        // Transfer to vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.funder_token_account.to_account_info(),
                to: ctx.accounts.subject_vault.to_account_info(),
                authority: ctx.accounts.funder.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        subject.pool_amount = subject.pool_amount.checked_add(amount).ok_or(error!(ErrorCode::ArithmeticOverflow))?;

        emit!(SubjectFunded {
            subject_id: subject.id,
            funder: ctx.accounts.funder.key(),
            amount,
            total_pool: subject.pool_amount,
        });

        Ok(())
    }

    /// Update subject status
    pub fn update_status(ctx: Context<UpdateSubject>, new_status: SubjectStatus) -> Result<()> {
        let subject = &mut ctx.accounts.subject;

        // Validate transitions
        match (&subject.status, &new_status) {
            (SubjectStatus::Open, SubjectStatus::Active) => {},
            (SubjectStatus::Active, SubjectStatus::Open) => {},
            (SubjectStatus::Open, SubjectStatus::Closed) => {},
            (SubjectStatus::Active, SubjectStatus::Closed) => {},
            _ => return Err(ErrorCode::InvalidStatusTransition.into()),
        }

        subject.status = new_status;

        emit!(SubjectStatusUpdated {
            subject_id: subject.id,
            new_status,
        });

        Ok(())
    }

    /// Resolve a subject with outcome
    /// Resolution method depends on framing (authority for civic, DAO for creator)
    pub fn resolve_subject(
        ctx: Context<ResolveSubject>,
        winning_outcome: Pubkey,
        resolution_proof: Option<ResolutionProof>,
    ) -> Result<()> {
        let subject = &mut ctx.accounts.subject;
        let config = &ctx.accounts.config;

        require!(
            subject.status == SubjectStatus::Open || subject.status == SubjectStatus::Active,
            ErrorCode::SubjectNotResolvable
        );

        // Validate resolution based on mode
        match config.protocol_mode {
            ProtocolMode::Civic => {
                // Authority-based resolution
                require!(
                    config.authority == ctx.accounts.resolver.key(),
                    ErrorCode::Unauthorized
                );
            }
            ProtocolMode::Creator => {
                // DAO vote resolution - proof required
                let proof = resolution_proof.ok_or(error!(ErrorCode::ResolutionProofRequired))?;
                require!(
                    proof.vote_count >= proof.quorum_required,
                    ErrorCode::QuorumNotReached
                );
                require!(
                    proof.approval_rate >= proof.threshold_required,
                    ErrorCode::ThresholdNotMet
                );
            }
        }

        subject.status = SubjectStatus::Resolved;
        subject.resolved_at = Some(Clock::get()?.unix_timestamp);
        subject.winning_outcome = Some(winning_outcome);

        emit!(SubjectResolved {
            subject_id: subject.id,
            winning_outcome,
            pool_amount: subject.pool_amount,
            mode: config.protocol_mode,
        });

        Ok(())
    }

    /// Increment market count (called by market-engine via CPI)
    pub fn increment_market_count(ctx: Context<IncrementMarketCount>) -> Result<()> {
        let subject = &mut ctx.accounts.subject;
        subject.market_count += 1;
        Ok(())
    }
}

// ============================================================================
// Accounts
// ============================================================================

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct RegisterSubject<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = registrar,
        space = 8 + Subject::INIT_SPACE,
        seeds = [b"subject", registrar.key().as_ref(), config.subject_count.to_le_bytes().as_ref()],
        bump
    )]
    pub subject: Account<'info, Subject>,

    #[account(mut)]
    pub registrar: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundSubject<'info> {
    #[account(
        mut,
        seeds = [b"subject", subject.registrar.as_ref(), subject.id.to_le_bytes().as_ref()],
        bump = subject.bump
    )]
    pub subject: Account<'info, Subject>,

    #[account(
        init_if_needed,
        payer = funder,
        seeds = [b"subject_vault", subject.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = subject_vault,
    )]
    pub subject_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = funder_token_account.mint == token_mint.key(),
        constraint = funder_token_account.owner == funder.key()
    )]
    pub funder_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSubject<'info> {
    #[account(
        mut,
        seeds = [b"subject", subject.registrar.as_ref(), subject.id.to_le_bytes().as_ref()],
        bump = subject.bump,
        constraint = subject.registrar == authority.key() || config.authority == authority.key()
    )]
    pub subject: Account<'info, Subject>,

    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveSubject<'info> {
    #[account(
        mut,
        seeds = [b"subject", subject.registrar.as_ref(), subject.id.to_le_bytes().as_ref()],
        bump = subject.bump
    )]
    pub subject: Account<'info, Subject>,

    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,

    pub resolver: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementMarketCount<'info> {
    #[account(
        mut,
        seeds = [b"subject", subject.registrar.as_ref(), subject.id.to_le_bytes().as_ref()],
        bump = subject.bump
    )]
    pub subject: Account<'info, Subject>,
}

// ============================================================================
// State
// ============================================================================

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub subject_count: u64,
    pub fee_rate: u16,
    pub protocol_mode: ProtocolMode,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Subject {
    pub id: u64,
    pub registrar: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(500)]
    pub description: String,
    pub metadata: SubjectMetadata,
    pub status: SubjectStatus,
    pub pool_amount: u64,
    pub deadline: i64,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
    pub winning_outcome: Option<Pubkey>,
    pub market_count: u64,
    pub bump: u8,
}

// ============================================================================
// Enums & Structs
// ============================================================================

/// Protocol mode determines resolution mechanism and features
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum ProtocolMode {
    /// Civic mode: Authority resolves, no burn
    /// Framing: Problems → Directions → Solutions
    Civic,
    /// Creator mode: DAO vote resolves, burn enabled
    /// Framing: Creators → Predictions → Admission
    Creator,
}

/// Generic subject status (works for both framings)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum SubjectStatus {
    /// Open for markets/predictions
    Open,
    /// Being actively worked on
    Active,
    /// Outcome determined
    Resolved,
    /// Cancelled or expired
    Closed,
}

/// Flexible metadata that can hold framing-specific data
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct SubjectMetadata {
    /// For Civic: location coordinates; For Creator: DAO reference
    pub context_a: i64,
    pub context_b: i64,
    /// Category code (interpreted by framing layer)
    pub category: u8,
    /// Additional flags
    pub flags: u8,
    /// Optional reference (DAO pubkey for creator, region for civic)
    pub reference: Option<Pubkey>,
}

/// Proof of DAO resolution (for Creator mode)
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResolutionProof {
    pub dao: Pubkey,
    pub vote_count: u16,
    pub quorum_required: u16,
    pub approval_rate: u16,      // basis points
    pub threshold_required: u16, // basis points
    pub vote_signature: [u8; 64],
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct SubjectRegistered {
    pub subject_id: u64,
    pub registrar: Pubkey,
    pub title: String,
    pub mode: ProtocolMode,
}

#[event]
pub struct SubjectFunded {
    pub subject_id: u64,
    pub funder: Pubkey,
    pub amount: u64,
    pub total_pool: u64,
}

#[event]
pub struct SubjectStatusUpdated {
    pub subject_id: u64,
    pub new_status: SubjectStatus,
}

#[event]
pub struct SubjectResolved {
    pub subject_id: u64,
    pub winning_outcome: Pubkey,
    pub pool_amount: u64,
    pub mode: ProtocolMode,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Title exceeds maximum length")]
    TitleTooLong,
    #[msg("Description exceeds maximum length")]
    DescriptionTooLong,
    #[msg("Deadline must be in the future")]
    InvalidDeadline,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Subject is not open")]
    SubjectNotOpen,
    #[msg("Subject cannot be resolved in current state")]
    SubjectNotResolvable,
    #[msg("Invalid status transition")]
    InvalidStatusTransition,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Resolution proof required for Creator mode")]
    ResolutionProofRequired,
    #[msg("Quorum not reached")]
    QuorumNotReached,
    #[msg("Approval threshold not met")]
    ThresholdNotMet,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
