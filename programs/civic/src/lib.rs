use anchor_lang::prelude::*;

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

    // This program is mostly type definitions and helpers.
    // Actual logic lives in the core programs.
    // CPIs are made to core programs with civic-specific parameters.

    /// Helper to create a civic problem (wraps subject_registry::register_subject)
    pub fn create_problem(
        ctx: Context<CivicAction>,
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

        emit!(ProblemCreated {
            title: title.clone(),
            category,
            location_lat,
            location_lng,
            deadline,
        });

        // Note: In practice, this would CPI to subject_registry::register_subject
        // with mode = ProtocolMode::Civic

        Ok(())
    }

    /// Helper to propose a direction (wraps market_engine::create_market)
    pub fn propose_direction(
        ctx: Context<CivicAction>,
        description: String,
        ai_analysis: String,
    ) -> Result<()> {
        msg!("Proposing civic direction: {}", description);

        emit!(DirectionProposed {
            description: description.clone(),
        });

        // Note: In practice, CPIs to market_engine::create_market

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

/// Civic reputation (alias for Reputation in core)
pub type CivicReputation = Reputation;

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

impl From<Outcome> for DirectionOutcome {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Yes => DirectionOutcome::SolutionWorks,
            Outcome::No => DirectionOutcome::SolutionFails,
        }
    }
}

impl From<DirectionOutcome> for Outcome {
    fn from(outcome: DirectionOutcome) -> Self {
        match outcome {
            DirectionOutcome::SolutionWorks => Outcome::Yes,
            DirectionOutcome::SolutionFails => Outcome::No,
        }
    }
}

// ============================================================================
// Re-exports from Core (for convenience)
// ============================================================================

// These would be imported from the core crates
// For now, stub definitions

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SubjectMetadata {
    pub context_a: i64,
    pub context_b: i64,
    pub category: u8,
    pub flags: u8,
    pub reference: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Yes,
    No,
}

// Stub types (would be imported from core)
pub struct Subject;
pub struct Market;
pub struct Reputation;

// ============================================================================
// Accounts (thin wrappers)
// ============================================================================

#[derive(Accounts)]
pub struct CivicAction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
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
    pub description: String,
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
