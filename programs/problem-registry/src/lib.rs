use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Fo1fm7iZpnpyCo8zbnnSN2g8zb6zFAips3hxk24NZETr");

#[program]
pub mod problem_registry {
    use super::*;

    /// Initialize the problem registry with a global config
    pub fn initialize(ctx: Context<Initialize>, fee_rate: u16) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.problem_count = 0;
        config.fee_rate = fee_rate;
        config.bump = ctx.bumps.config;

        msg!("Problem Registry initialized with fee rate: {} bps", fee_rate);
        Ok(())
    }

    /// Create a new civic problem
    pub fn create_problem(
        ctx: Context<CreateProblem>,
        title: String,
        description: String,
        location_lat: i64,
        location_lng: i64,
        category: ProblemCategory,
        deadline: i64,
    ) -> Result<()> {
        require!(title.len() <= 100, ErrorCode::TitleTooLong);
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        require!(deadline > Clock::get()?.unix_timestamp, ErrorCode::InvalidDeadline);

        let config = &mut ctx.accounts.config;
        let problem = &mut ctx.accounts.problem;

        problem.id = config.problem_count;
        problem.creator = ctx.accounts.creator.key();
        problem.title = title;
        problem.description = description;
        problem.location_lat = location_lat;
        problem.location_lng = location_lng;
        problem.category = category;
        problem.status = ProblemStatus::Open;
        problem.bounty_amount = 0;
        problem.deadline = deadline;
        problem.created_at = Clock::get()?.unix_timestamp;
        problem.resolved_at = None;
        problem.winning_direction = None;
        problem.direction_count = 0;
        problem.bump = ctx.bumps.problem;

        config.problem_count += 1;

        emit!(ProblemCreated {
            problem_id: problem.id,
            creator: problem.creator,
            title: problem.title.clone(),
            category: problem.category,
            deadline: problem.deadline,
        });

        msg!("Problem #{} created: {}", problem.id, problem.title);
        Ok(())
    }

    /// Fund a problem with USDC bounty
    pub fn fund_problem(ctx: Context<FundProblem>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let problem = &mut ctx.accounts.problem;
        require!(problem.status == ProblemStatus::Open, ErrorCode::ProblemNotOpen);

        // Transfer USDC from funder to problem vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.funder_token_account.to_account_info(),
                to: ctx.accounts.problem_vault.to_account_info(),
                authority: ctx.accounts.funder.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        problem.bounty_amount = problem.bounty_amount.checked_add(amount).unwrap();

        emit!(ProblemFunded {
            problem_id: problem.id,
            funder: ctx.accounts.funder.key(),
            amount,
            total_bounty: problem.bounty_amount,
        });

        msg!("Problem #{} funded with {} USDC", problem.id, amount);
        Ok(())
    }

    /// Update problem status (creator or authority only)
    pub fn update_status(ctx: Context<UpdateProblem>, new_status: ProblemStatus) -> Result<()> {
        let problem = &mut ctx.accounts.problem;

        // Validate status transitions
        match (&problem.status, &new_status) {
            (ProblemStatus::Open, ProblemStatus::InProgress) => {},
            (ProblemStatus::InProgress, ProblemStatus::Open) => {},
            (ProblemStatus::Open, ProblemStatus::Closed) => {},
            (ProblemStatus::InProgress, ProblemStatus::Closed) => {},
            _ => return Err(ErrorCode::InvalidStatusTransition.into()),
        }

        problem.status = new_status;

        emit!(ProblemStatusUpdated {
            problem_id: problem.id,
            new_status: problem.status,
        });

        msg!("Problem #{} status updated to {:?}", problem.id, problem.status);
        Ok(())
    }

    /// Resolve a problem with a winning direction (authority only)
    pub fn resolve_problem(ctx: Context<ResolveProblem>, winning_direction: Pubkey) -> Result<()> {
        let problem = &mut ctx.accounts.problem;

        require!(
            problem.status == ProblemStatus::Open || problem.status == ProblemStatus::InProgress,
            ErrorCode::ProblemNotResolvable
        );

        problem.status = ProblemStatus::Resolved;
        problem.resolved_at = Some(Clock::get()?.unix_timestamp);
        problem.winning_direction = Some(winning_direction);

        emit!(ProblemResolved {
            problem_id: problem.id,
            winning_direction,
            bounty_amount: problem.bounty_amount,
        });

        msg!("Problem #{} resolved with winning direction: {}", problem.id, winning_direction);
        Ok(())
    }

    /// Increment direction count (called by direction-market program via CPI)
    pub fn increment_direction_count(ctx: Context<IncrementDirectionCount>) -> Result<()> {
        let problem = &mut ctx.accounts.problem;
        problem.direction_count += 1;
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
pub struct CreateProblem<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = creator,
        space = 8 + Problem::INIT_SPACE,
        seeds = [b"problem", creator.key().as_ref(), config.problem_count.to_le_bytes().as_ref()],
        bump
    )]
    pub problem: Account<'info, Problem>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundProblem<'info> {
    #[account(
        mut,
        seeds = [b"problem", problem.creator.as_ref(), problem.id.to_le_bytes().as_ref()],
        bump = problem.bump
    )]
    pub problem: Account<'info, Problem>,

    #[account(
        init_if_needed,
        payer = funder,
        seeds = [b"problem_vault", problem.key().as_ref()],
        bump,
        token::mint = usdc_mint,
        token::authority = problem_vault,
    )]
    pub problem_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = funder_token_account.mint == usdc_mint.key(),
        constraint = funder_token_account.owner == funder.key()
    )]
    pub funder_token_account: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProblem<'info> {
    #[account(
        mut,
        seeds = [b"problem", problem.creator.as_ref(), problem.id.to_le_bytes().as_ref()],
        bump = problem.bump,
        constraint = problem.creator == authority.key() || config.authority == authority.key()
    )]
    pub problem: Account<'info, Problem>,

    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveProblem<'info> {
    #[account(
        mut,
        seeds = [b"problem", problem.creator.as_ref(), problem.id.to_le_bytes().as_ref()],
        bump = problem.bump
    )]
    pub problem: Account<'info, Problem>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementDirectionCount<'info> {
    #[account(
        mut,
        seeds = [b"problem", problem.creator.as_ref(), problem.id.to_le_bytes().as_ref()],
        bump = problem.bump
    )]
    pub problem: Account<'info, Problem>,
}

// ============================================================================
// State
// ============================================================================

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub problem_count: u64,
    pub fee_rate: u16,      // Basis points (e.g., 250 = 2.5%)
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Problem {
    pub id: u64,
    pub creator: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(500)]
    pub description: String,
    pub location_lat: i64,   // Scaled by 1,000,000
    pub location_lng: i64,   // Scaled by 1,000,000
    pub category: ProblemCategory,
    pub status: ProblemStatus,
    pub bounty_amount: u64,  // USDC lamports
    pub deadline: i64,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
    pub winning_direction: Option<Pubkey>,
    pub direction_count: u64,
    pub bump: u8,
}

// ============================================================================
// Enums
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum ProblemCategory {
    Infrastructure,     // Roads, bridges, utilities
    Environment,        // Pollution, parks, wildlife
    Safety,            // Crime, traffic, emergency
    Health,            // Healthcare access, sanitation
    Education,         // Schools, libraries, programs
    Transportation,    // Public transit, bike lanes
    Housing,           // Affordability, homelessness
    Community,         // Social services, recreation
    Economic,          // Jobs, business development
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum ProblemStatus {
    Open,              // Accepting directions
    InProgress,        // Being worked on
    Resolved,          // Solution verified
    Closed,            // Cancelled or expired
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct ProblemCreated {
    pub problem_id: u64,
    pub creator: Pubkey,
    pub title: String,
    pub category: ProblemCategory,
    pub deadline: i64,
}

#[event]
pub struct ProblemFunded {
    pub problem_id: u64,
    pub funder: Pubkey,
    pub amount: u64,
    pub total_bounty: u64,
}

#[event]
pub struct ProblemStatusUpdated {
    pub problem_id: u64,
    pub new_status: ProblemStatus,
}

#[event]
pub struct ProblemResolved {
    pub problem_id: u64,
    pub winning_direction: Pubkey,
    pub bounty_amount: u64,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Title exceeds maximum length of 100 characters")]
    TitleTooLong,
    #[msg("Description exceeds maximum length of 500 characters")]
    DescriptionTooLong,
    #[msg("Deadline must be in the future")]
    InvalidDeadline,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Problem is not open for funding or directions")]
    ProblemNotOpen,
    #[msg("Problem cannot be resolved in current state")]
    ProblemNotResolvable,
    #[msg("Invalid status transition")]
    InvalidStatusTransition,
    #[msg("Unauthorized action")]
    Unauthorized,
}
