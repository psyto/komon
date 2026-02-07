use anchor_lang::prelude::*;
use subject_registry::cpi::accounts::RegisterSubject;
use subject_registry::cpi::register_subject;
use subject_registry::program::SubjectRegistry;
use subject_registry::{Config as SubjectConfig, Subject, SubjectMetadata, ProtocolMode};
use market_engine::cpi::accounts::CreateMarket;
use market_engine::cpi::create_market;
use market_engine::program::MarketEngine;
use market_engine::{MarketConfig, Market};

declare_id!("9kLcoQ1Kpr66pNHL3m9cfWhgXzCi5E9VzzW6TBA3fYu1");

// =============================================================================
// CIVIC FRAMING LAYER
// =============================================================================
//
// Thin wrapper over the shared core that provides civic vocabulary:
// - Subject → Problem
// - Market → Direction
// - Outcome::Yes → Solution works
// - Outcome::No → Solution fails
//
// This layer handles:
// 1. Type aliases for civic terminology
// 2. Category enums specific to civic problems
// 3. Authority-based resolution (not DAO vote)
// 4. No burn mechanism
// =============================================================================

/// Civic framing module - provides Problem/Direction vocabulary
#[program]
pub mod civic {
    use super::*;

    /// Create a civic problem (CPI to subject_registry::register_subject)
    pub fn create_problem(
        ctx: Context<CreateProblem>,
        title: String,
        description: String,
        location_lat: i64,
        location_lng: i64,
        category: ProblemCategory,
        deadline: i64,
    ) -> Result<()> {
        // Build metadata for civic context
        let metadata = SubjectMetadata {
            context_a: location_lat,
            context_b: location_lng,
            category: category as u8,
            flags: 0,
            reference: None,
        };

        // Log the civic framing
        msg!(
            "Creating civic problem: {} (category: {:?}, location: {}, {})",
            title,
            category,
            location_lat,
            location_lng
        );

        // CPI to subject_registry::register_subject
        let cpi_program = ctx.accounts.subject_registry_program.to_account_info();
        let cpi_accounts = RegisterSubject {
            config: ctx.accounts.subject_config.to_account_info(),
            subject: ctx.accounts.subject.to_account_info(),
            registrar: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        register_subject(cpi_ctx, title.clone(), description, metadata, deadline)?;

        emit!(ProblemCreated {
            title,
            category,
            location_lat,
            location_lng,
            deadline,
        });

        Ok(())
    }

    /// Propose a direction/solution (CPI to market_engine::create_market)
    pub fn propose_direction(
        ctx: Context<ProposeDirection>,
        problem: Pubkey,
        description: String,
        ai_analysis: String,
    ) -> Result<()> {
        msg!("Proposing civic direction: {}", description);

        // CPI to market_engine::create_market
        let cpi_program = ctx.accounts.market_engine_program.to_account_info();
        let cpi_accounts = CreateMarket {
            config: ctx.accounts.market_config.to_account_info(),
            subject: ctx.accounts.problem.to_account_info(),
            market: ctx.accounts.direction.to_account_info(),
            yes_mint: ctx.accounts.yes_mint.to_account_info(),
            no_mint: ctx.accounts.no_mint.to_account_info(),
            creator: ctx.accounts.user.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        create_market(cpi_ctx, description.clone(), ai_analysis)?;

        emit!(DirectionProposed {
            problem,
            description,
            proposer: ctx.accounts.user.key(),
        });

        Ok(())
    }
}

// ============================================================================
// Civic Type Aliases (for SDK/frontend clarity)
// ============================================================================

/// A civic problem (alias for Subject in core)
pub type Problem = Subject;

/// A proposed direction/solution (alias for Market in core)
pub type Direction = Market;

// ============================================================================
// Civic-Specific Enums
// ============================================================================

/// Categories for civic problems
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProblemCategory {
    Infrastructure = 0,    // Roads, bridges, utilities
    Environment = 1,       // Pollution, parks, wildlife
    Safety = 2,           // Crime, traffic, emergency
    Health = 3,           // Healthcare access, sanitation
    Education = 4,        // Schools, libraries, programs
    Transportation = 5,   // Public transit, bike lanes
    Housing = 6,          // Affordability, homelessness
    Community = 7,        // Social services, recreation
    Economic = 8,         // Jobs, business development
    Other = 9,
}

impl ProblemCategory {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Infrastructure),
            1 => Some(Self::Environment),
            2 => Some(Self::Safety),
            3 => Some(Self::Health),
            4 => Some(Self::Education),
            5 => Some(Self::Transportation),
            6 => Some(Self::Housing),
            7 => Some(Self::Community),
            8 => Some(Self::Economic),
            9 => Some(Self::Other),
            _ => None,
        }
    }
}

/// Direction outcome in civic terms
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum DirectionOutcome {
    /// The proposed solution worked
    SolutionWorks,
    /// The proposed solution failed
    SolutionFails,
}

// ============================================================================
// Accounts
// ============================================================================

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProblem<'info> {
    /// Subject registry config (for CPI)
    /// CHECK: Validated by subject_registry program
    #[account(mut)]
    pub subject_config: UncheckedAccount<'info>,

    /// The subject/problem account to create
    /// CHECK: Created by subject_registry via CPI
    #[account(mut)]
    pub subject: UncheckedAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub subject_registry_program: Program<'info, SubjectRegistry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProposeDirection<'info> {
    /// Market engine config (for CPI)
    /// CHECK: Validated by market_engine program
    #[account(mut)]
    pub market_config: UncheckedAccount<'info>,

    /// The problem this direction solves
    /// CHECK: Validated by market_engine program
    pub problem: UncheckedAccount<'info>,

    /// The direction/market account to create
    /// CHECK: Created by market_engine via CPI
    #[account(mut)]
    pub direction: UncheckedAccount<'info>,

    /// YES outcome token mint
    /// CHECK: Created by market_engine via CPI
    #[account(mut)]
    pub yes_mint: UncheckedAccount<'info>,

    /// NO outcome token mint
    /// CHECK: Created by market_engine via CPI
    #[account(mut)]
    pub no_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub market_engine_program: Program<'info, MarketEngine>,
    /// CHECK: Required for token operations
    pub token_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct ProblemCreated {
    pub title: String,
    pub category: ProblemCategory,
    pub location_lat: i64,
    pub location_lng: i64,
    pub deadline: i64,
}

#[event]
pub struct DirectionProposed {
    pub problem: Pubkey,
    pub description: String,
    pub proposer: Pubkey,
}

// ============================================================================
// Configuration Constants
// ============================================================================

/// Civic mode configuration
pub mod civic_config {
    /// No burn in civic mode
    pub const BURN_ENABLED: bool = false;
    pub const BURN_RATE_BPS: u16 = 0;

    /// Resolution is authority-based
    pub const RESOLUTION_TYPE: &str = "authority";

    /// Default fee rate (basis points)
    pub const DEFAULT_FEE_BPS: u16 = 250; // 2.5%
}
