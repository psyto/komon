use anchor_lang::prelude::*;

declare_id!("EWzQRuqkZSo3gmWnVpyob3c2bBRDpPusdL8oy3cpDHWU");

// =============================================================================
// REPUTATION CORE - Generic Reputation System
// =============================================================================
//
// Shared reputation tracking for both framings:
// - Civic: Track problem-solving and civic participation
// - Creator: Track creator quality and scout accuracy
//
// Maps to SOVEREIGN identity for portable reputation.
// =============================================================================

#[program]
pub mod reputation_core {
    use super::*;

    /// Initialize the reputation system
    pub fn initialize(ctx: Context<Initialize>, mode: ReputationMode) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.total_users = 0;
        config.mode = mode;
        config.bump = ctx.bumps.config;

        msg!("Reputation Core initialized in {:?} mode", mode);
        Ok(())
    }

    /// Initialize a user's reputation
    pub fn initialize_reputation(ctx: Context<InitializeReputation>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let reputation = &mut ctx.accounts.reputation;

        reputation.user = ctx.accounts.user.key();
        reputation.subjects_created = 0;
        reputation.markets_created = 0;
        reputation.predictions_won = 0;
        reputation.predictions_lost = 0;
        reputation.total_earned = 0;
        reputation.total_staked = 0;
        reputation.accuracy_bps = 0;
        reputation.current_streak = 0;
        reputation.best_streak = 0;
        reputation.level = 1;
        reputation.experience = 0;
        reputation.created_at = Clock::get()?.unix_timestamp;
        reputation.last_active = Clock::get()?.unix_timestamp;
        reputation.bump = ctx.bumps.reputation;

        config.total_users += 1;

        emit!(ReputationInitialized {
            user: reputation.user,
            mode: config.mode,
        });

        Ok(())
    }

    /// Record an action (generic, interpreted by framing)
    pub fn record_action(
        ctx: Context<UpdateReputation>,
        action: ActionType,
        amount: Option<u64>,
    ) -> Result<()> {
        let reputation = &mut ctx.accounts.reputation;
        reputation.last_active = Clock::get()?.unix_timestamp;

        match action {
            ActionType::SubjectCreated => {
                reputation.subjects_created += 1;
                reputation.experience = reputation.experience.checked_add(10)
                    .ok_or(error!(ReputationError::ArithmeticOverflow))?;
            }
            ActionType::MarketCreated => {
                reputation.markets_created += 1;
                reputation.experience = reputation.experience.checked_add(20)
                    .ok_or(error!(ReputationError::ArithmeticOverflow))?;
            }
            ActionType::StakePlaced => {
                if let Some(amt) = amount {
                    reputation.total_staked = reputation.total_staked.checked_add(amt)
                        .ok_or(error!(ReputationError::ArithmeticOverflow))?;
                }
                reputation.experience = reputation.experience.checked_add(5)
                    .ok_or(error!(ReputationError::ArithmeticOverflow))?;
            }
            ActionType::PredictionWon => {
                reputation.predictions_won += 1;
                if let Some(amt) = amount {
                    reputation.total_earned = reputation.total_earned.checked_add(amt)
                        .ok_or(error!(ReputationError::ArithmeticOverflow))?;
                }
                // Update streak
                if reputation.current_streak >= 0 {
                    reputation.current_streak += 1;
                } else {
                    reputation.current_streak = 1;
                }
                if reputation.current_streak as u16 > reputation.best_streak {
                    reputation.best_streak = reputation.current_streak as u16;
                }
                reputation.experience = reputation.experience.checked_add(50)
                    .ok_or(error!(ReputationError::ArithmeticOverflow))?;
            }
            ActionType::PredictionLost => {
                reputation.predictions_lost += 1;
                if reputation.current_streak <= 0 {
                    reputation.current_streak -= 1;
                } else {
                    reputation.current_streak = -1;
                }
                reputation.experience = reputation.experience.checked_add(10)
                    .ok_or(error!(ReputationError::ArithmeticOverflow))?;
            }
        }

        // Update accuracy
        let total = reputation.predictions_won + reputation.predictions_lost;
        if total > 0 {
            reputation.accuracy_bps = ((reputation.predictions_won as u64)
                .checked_mul(10000)
                .ok_or(error!(ReputationError::ArithmeticOverflow))?
                .checked_div(total as u64)
                .ok_or(error!(ReputationError::ArithmeticOverflow))?) as u16;
        }

        update_level(reputation);

        emit!(ActionRecorded {
            user: reputation.user,
            action,
            experience: reputation.experience,
        });

        Ok(())
    }

    /// Calculate score for SOVEREIGN sync
    /// Returns a score 0-10000 based on reputation mode
    pub fn calculate_score(ctx: Context<ReadReputation>) -> Result<u16> {
        let reputation = &ctx.accounts.reputation;
        let config = &ctx.accounts.config;

        let score = match config.mode {
            ReputationMode::Civic => calculate_civic_score(reputation),
            ReputationMode::Creator => calculate_creator_score(reputation),
        };

        Ok(score)
    }
}

/// Calculate civic score (for SOVEREIGN civic dimension)
fn calculate_civic_score(rep: &Reputation) -> u16 {
    // Weights: Accuracy 40%, Activity 25%, Trust 25%, Streak 10%
    let accuracy = rep.accuracy_bps as u32 * 40 / 100;

    let activity_tier = match rep.subjects_created + rep.markets_created {
        0..=2 => 2000u32,
        3..=10 => 4000,
        11..=50 => 6000,
        51..=200 => 8000,
        _ => 10000,
    };
    let activity = activity_tier * 25 / 100;

    let trust_tier = match rep.predictions_won {
        0..=5 => 2000u32,
        6..=20 => 4000,
        21..=50 => 6000,
        51..=100 => 8000,
        _ => 10000,
    };
    let trust = trust_tier * 25 / 100;

    let streak_tier = match rep.best_streak {
        0..=2 => 2000u32,
        3..=5 => 4000,
        6..=10 => 6000,
        11..=20 => 8000,
        _ => 10000,
    };
    let streak = streak_tier * 10 / 100;

    (accuracy + activity + trust + streak).min(10000) as u16
}

/// Calculate creator score (for SOVEREIGN creator dimension)
/// Vitalik: "good creators are also good judges of quality"
fn calculate_creator_score(rep: &Reputation) -> u16 {
    // Weights aligned with Vitalik's model:
    // - Accuracy (judgment quality): 40%
    // - Markets created (surfacing): 25%
    // - Earnings (successful predictions): 20%
    // - Streak (consistency): 15%

    let accuracy = rep.accuracy_bps as u32 * 40 / 100;

    // Surfacing contribution (markets created)
    let surfacing_tier = match rep.markets_created {
        0..=5 => 2000u32,
        6..=20 => 4000,
        21..=50 => 6000,
        51..=100 => 8000,
        _ => 10000,
    };
    let surfacing = surfacing_tier * 25 / 100;

    // Earnings tier
    let earnings_tier = match rep.total_earned {
        0..=1_000_000_000 => 2000u32,      // < 1 SOL
        1_000_000_001..=10_000_000_000 => 4000,
        10_000_000_001..=100_000_000_000 => 6000,
        100_000_000_001..=1_000_000_000_000 => 8000,
        _ => 10000,
    };
    let earnings = earnings_tier * 20 / 100;

    let streak_tier = match rep.best_streak {
        0..=2 => 2000u32,
        3..=5 => 4000,
        6..=10 => 6000,
        11..=20 => 8000,
        _ => 10000,
    };
    let streak = streak_tier * 15 / 100;

    (accuracy + surfacing + earnings + streak).min(10000) as u16
}

fn update_level(reputation: &mut Reputation) {
    let mut level = 1u8;
    let mut threshold = 100u64;

    while reputation.experience >= threshold && level < 255 {
        level += 1;
        threshold = (level as u64) * ((level as u64) + 1) * 50;
    }

    if level > reputation.level {
        emit!(LevelUp {
            user: reputation.user,
            old_level: reputation.level,
            new_level: level,
        });
    }

    reputation.level = level;
}

// ============================================================================
// Accounts
// ============================================================================

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ReputationConfig::INIT_SPACE,
        seeds = [b"reputation_config"],
        bump
    )]
    pub config: Account<'info, ReputationConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeReputation<'info> {
    #[account(mut, seeds = [b"reputation_config"], bump = config.bump)]
    pub config: Account<'info, ReputationConfig>,

    #[account(
        init,
        payer = payer,
        space = 8 + Reputation::INIT_SPACE,
        seeds = [b"reputation", user.key().as_ref()],
        bump
    )]
    pub reputation: Account<'info, Reputation>,

    /// CHECK: The user whose reputation is being initialized
    pub user: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateReputation<'info> {
    #[account(
        mut,
        seeds = [b"reputation", user.key().as_ref()],
        bump = reputation.bump,
        constraint = reputation.user == user.key()
    )]
    pub reputation: Account<'info, Reputation>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReadReputation<'info> {
    #[account(seeds = [b"reputation_config"], bump = config.bump)]
    pub config: Account<'info, ReputationConfig>,

    #[account(seeds = [b"reputation", reputation.user.as_ref()], bump = reputation.bump)]
    pub reputation: Account<'info, Reputation>,
}

// ============================================================================
// State
// ============================================================================

#[account]
#[derive(InitSpace)]
pub struct ReputationConfig {
    pub authority: Pubkey,
    pub total_users: u64,
    pub mode: ReputationMode,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Reputation {
    pub user: Pubkey,
    pub subjects_created: u32,    // Problems (civic) or Creator nominations (creator)
    pub markets_created: u32,     // Directions (civic) or Predictions (creator)
    pub predictions_won: u32,
    pub predictions_lost: u32,
    pub total_earned: u64,
    pub total_staked: u64,
    pub accuracy_bps: u16,
    pub current_streak: i16,
    pub best_streak: u16,
    pub level: u8,
    pub experience: u64,
    pub created_at: i64,
    pub last_active: i64,
    pub bump: u8,
}

// ============================================================================
// Enums
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum ReputationMode {
    Civic,
    Creator,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    SubjectCreated,
    MarketCreated,
    StakePlaced,
    PredictionWon,
    PredictionLost,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct ReputationInitialized {
    pub user: Pubkey,
    pub mode: ReputationMode,
}

#[event]
pub struct ActionRecorded {
    pub user: Pubkey,
    pub action: ActionType,
    pub experience: u64,
}

#[event]
pub struct LevelUp {
    pub user: Pubkey,
    pub old_level: u8,
    pub new_level: u8,
}

#[error_code]
pub enum ReputationError {
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
