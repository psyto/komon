use anchor_lang::prelude::*;

pub mod sovereign;
pub use sovereign::*;

declare_id!("AWDeGkLSX3HcU2s8vhYThxLeDQ4N9DqhnREuZU36vuJU");

#[program]
pub mod reputation {
    use super::*;

    /// Initialize the reputation system config
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.total_users = 0;
        config.bump = ctx.bumps.config;

        msg!("Reputation system initialized");
        Ok(())
    }

    /// Initialize a user's reputation (auto-created on first action)
    pub fn initialize_reputation(ctx: Context<InitializeReputation>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let reputation = &mut ctx.accounts.reputation;

        reputation.user = ctx.accounts.user.key();
        reputation.problems_posted = 0;
        reputation.directions_proposed = 0;
        reputation.directions_won = 0;
        reputation.directions_lost = 0;
        reputation.total_earned = 0;
        reputation.total_staked = 0;
        reputation.win_rate = 0;
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
            created_at: reputation.created_at,
        });

        msg!("Reputation initialized for user {}", reputation.user);
        Ok(())
    }

    /// Record a problem posted
    pub fn record_problem_posted(ctx: Context<UpdateReputation>) -> Result<()> {
        let reputation = &mut ctx.accounts.reputation;

        reputation.problems_posted += 1;
        reputation.last_active = Clock::get()?.unix_timestamp;
        reputation.experience = reputation.experience.checked_add(10).unwrap();
        update_level(reputation);

        emit!(ActionRecorded {
            user: reputation.user,
            action: ActionType::ProblemPosted,
            new_experience: reputation.experience,
        });

        msg!("Problem posted recorded for user {}", reputation.user);
        Ok(())
    }

    /// Record a direction proposed
    pub fn record_direction_proposed(ctx: Context<UpdateReputation>) -> Result<()> {
        let reputation = &mut ctx.accounts.reputation;

        reputation.directions_proposed += 1;
        reputation.last_active = Clock::get()?.unix_timestamp;
        reputation.experience = reputation.experience.checked_add(20).unwrap();
        update_level(reputation);

        emit!(ActionRecorded {
            user: reputation.user,
            action: ActionType::DirectionProposed,
            new_experience: reputation.experience,
        });

        msg!("Direction proposed recorded for user {}", reputation.user);
        Ok(())
    }

    /// Record a stake placed
    pub fn record_stake(ctx: Context<UpdateReputation>, amount: u64) -> Result<()> {
        let reputation = &mut ctx.accounts.reputation;

        reputation.total_staked = reputation.total_staked.checked_add(amount).unwrap();
        reputation.last_active = Clock::get()?.unix_timestamp;
        reputation.experience = reputation.experience.checked_add(5).unwrap();
        update_level(reputation);

        emit!(ActionRecorded {
            user: reputation.user,
            action: ActionType::StakePlaced,
            new_experience: reputation.experience,
        });

        msg!("Stake of {} recorded for user {}", amount, reputation.user);
        Ok(())
    }

    /// Record a direction outcome (win or loss)
    pub fn record_direction_outcome(
        ctx: Context<UpdateReputation>,
        won: bool,
        earnings: u64,
    ) -> Result<()> {
        let reputation = &mut ctx.accounts.reputation;

        if won {
            reputation.directions_won += 1;
            reputation.total_earned = reputation.total_earned.checked_add(earnings).unwrap();

            // Update streak
            if reputation.current_streak >= 0 {
                reputation.current_streak += 1;
            } else {
                reputation.current_streak = 1;
            }

            // Update best streak
            if reputation.current_streak as u16 > reputation.best_streak {
                reputation.best_streak = reputation.current_streak as u16;
            }

            // Bonus XP for win
            reputation.experience = reputation.experience.checked_add(50).unwrap();
        } else {
            reputation.directions_lost += 1;

            // Update streak
            if reputation.current_streak <= 0 {
                reputation.current_streak -= 1;
            } else {
                reputation.current_streak = -1;
            }

            // Some XP even for loss (participation)
            reputation.experience = reputation.experience.checked_add(10).unwrap();
        }

        // Update win rate (basis points)
        let total_outcomes = reputation.directions_won + reputation.directions_lost;
        if total_outcomes > 0 {
            reputation.win_rate = ((reputation.directions_won as u64)
                .checked_mul(10000)
                .unwrap()
                .checked_div(total_outcomes as u64)
                .unwrap()) as u16;
        }

        reputation.last_active = Clock::get()?.unix_timestamp;
        update_level(reputation);

        emit!(OutcomeRecorded {
            user: reputation.user,
            won,
            earnings,
            new_win_rate: reputation.win_rate,
            current_streak: reputation.current_streak,
        });

        msg!(
            "Outcome recorded for user {}: won={}, earnings={}",
            reputation.user,
            won,
            earnings
        );
        Ok(())
    }

    /// Update reputation via CPI from other programs
    pub fn update_via_cpi(
        ctx: Context<UpdateReputationCpi>,
        action: ActionType,
        amount: Option<u64>,
        won: Option<bool>,
    ) -> Result<()> {
        let reputation = &mut ctx.accounts.reputation;
        reputation.last_active = Clock::get()?.unix_timestamp;

        match action {
            ActionType::ProblemPosted => {
                reputation.problems_posted += 1;
                reputation.experience = reputation.experience.checked_add(10).unwrap();
            }
            ActionType::DirectionProposed => {
                reputation.directions_proposed += 1;
                reputation.experience = reputation.experience.checked_add(20).unwrap();
            }
            ActionType::StakePlaced => {
                if let Some(amt) = amount {
                    reputation.total_staked = reputation.total_staked.checked_add(amt).unwrap();
                }
                reputation.experience = reputation.experience.checked_add(5).unwrap();
            }
            ActionType::DirectionWon => {
                reputation.directions_won += 1;
                if let Some(amt) = amount {
                    reputation.total_earned = reputation.total_earned.checked_add(amt).unwrap();
                }
                if reputation.current_streak >= 0 {
                    reputation.current_streak += 1;
                } else {
                    reputation.current_streak = 1;
                }
                if reputation.current_streak as u16 > reputation.best_streak {
                    reputation.best_streak = reputation.current_streak as u16;
                }
                reputation.experience = reputation.experience.checked_add(50).unwrap();
            }
            ActionType::DirectionLost => {
                reputation.directions_lost += 1;
                if reputation.current_streak <= 0 {
                    reputation.current_streak -= 1;
                } else {
                    reputation.current_streak = -1;
                }
                reputation.experience = reputation.experience.checked_add(10).unwrap();
            }
        }

        // Update win rate
        let total_outcomes = reputation.directions_won + reputation.directions_lost;
        if total_outcomes > 0 {
            reputation.win_rate = ((reputation.directions_won as u64)
                .checked_mul(10000)
                .unwrap()
                .checked_div(total_outcomes as u64)
                .unwrap()) as u16;
        }

        update_level(reputation);

        emit!(ActionRecorded {
            user: reputation.user,
            action,
            new_experience: reputation.experience,
        });

        Ok(())
    }

    /// Sync Komon reputation to SOVEREIGN civic score
    /// This allows the user's Komon civic participation to contribute
    /// to their universal SOVEREIGN identity
    pub fn sync_to_sovereign(ctx: Context<SyncToSovereign>) -> Result<()> {
        let reputation = &ctx.accounts.reputation;

        // Calculate civic score from Komon reputation
        let civic_score = calculate_civic_score(
            reputation.problems_posted,
            reputation.directions_proposed,
            reputation.directions_won,
            reputation.directions_lost,
            reputation.win_rate,
            reputation.current_streak,
            reputation.level,
        );

        // Build CPI to SOVEREIGN program
        let cpi_program = ctx.accounts.sovereign_program.to_account_info();

        // Create instruction data for update_civic_score
        // Anchor instruction discriminator for update_civic_score + score argument
        let mut data = Vec::with_capacity(10);
        // Discriminator for "update_civic_score" (first 8 bytes of sha256("global:update_civic_score"))
        data.extend_from_slice(&[62, 199, 170, 21, 48, 132, 219, 245]);
        // Score as u16 little-endian
        data.extend_from_slice(&civic_score.to_le_bytes());

        let account_metas = vec![
            AccountMeta::new_readonly(ctx.accounts.user.key(), true), // authority (signer)
            AccountMeta::new(ctx.accounts.sovereign_identity.key(), false), // identity
        ];

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: sovereign::SOVEREIGN_PROGRAM_ID,
            accounts: account_metas,
            data,
        };

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.sovereign_identity.to_account_info(),
            ],
        )?;

        emit!(SyncedToSovereign {
            user: reputation.user,
            civic_score,
        });

        msg!(
            "Synced Komon reputation to SOVEREIGN: civic_score={}",
            civic_score
        );
        Ok(())
    }

    /// Get a user's SOVEREIGN tier (read-only, no CPI)
    /// Returns the user's tier level (1-5) or 1 if no SOVEREIGN identity
    pub fn get_sovereign_tier(ctx: Context<GetSovereignTier>) -> Result<u8> {
        // Deserialize the SOVEREIGN identity account
        let sovereign_data = ctx.accounts.sovereign_identity.try_borrow_data()?;

        // Skip 8-byte discriminator
        if sovereign_data.len() < SovereignIdentity::SIZE {
            return Ok(1); // Default tier if account doesn't exist
        }

        // Parse tier from the account data
        // Tier is at offset: 8 (disc) + 32 (owner) + 8 (created) + 32*4 (authorities) + 2*4 (scores) + 2 (composite) = 186
        let tier_offset = 8 + 32 + 8 + (32 * 4) + (2 * 4) + 2;
        if sovereign_data.len() > tier_offset {
            Ok(sovereign_data[tier_offset])
        } else {
            Ok(1)
        }
    }
}

/// Calculate and update user level based on experience
fn update_level(reputation: &mut Reputation) {
    // Level thresholds: 100, 300, 600, 1000, 1500, 2100, 2800, 3600, 4500, 5500...
    // Formula: level N requires N*(N+1)*50 XP
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
            experience: reputation.experience,
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
    #[account(
        mut,
        seeds = [b"reputation_config"],
        bump = config.bump
    )]
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
pub struct UpdateReputationCpi<'info> {
    #[account(
        seeds = [b"reputation_config"],
        bump = config.bump
    )]
    pub config: Account<'info, ReputationConfig>,

    #[account(
        mut,
        seeds = [b"reputation", reputation.user.as_ref()],
        bump = reputation.bump
    )]
    pub reputation: Account<'info, Reputation>,

    /// The authorized caller (must be one of our programs)
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct SyncToSovereign<'info> {
    #[account(
        seeds = [b"reputation", user.key().as_ref()],
        bump = reputation.bump,
        constraint = reputation.user == user.key()
    )]
    pub reputation: Account<'info, Reputation>,

    /// The user must sign to authorize sync
    pub user: Signer<'info>,

    /// CHECK: SOVEREIGN identity account (validated by SOVEREIGN program)
    #[account(mut)]
    pub sovereign_identity: UncheckedAccount<'info>,

    /// CHECK: SOVEREIGN program for CPI
    #[account(address = sovereign::SOVEREIGN_PROGRAM_ID)]
    pub sovereign_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct GetSovereignTier<'info> {
    /// CHECK: SOVEREIGN identity account (read-only)
    pub sovereign_identity: UncheckedAccount<'info>,
}

// ============================================================================
// State
// ============================================================================

#[account]
#[derive(InitSpace)]
pub struct ReputationConfig {
    pub authority: Pubkey,
    pub total_users: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Reputation {
    pub user: Pubkey,
    pub problems_posted: u32,
    pub directions_proposed: u32,
    pub directions_won: u32,
    pub directions_lost: u32,
    pub total_earned: u64,       // Total USDC earned
    pub total_staked: u64,       // Total USDC staked
    pub win_rate: u16,           // Basis points (e.g., 6500 = 65%)
    pub current_streak: i16,     // Positive = wins, negative = losses
    pub best_streak: u16,        // Best winning streak
    pub level: u8,               // 1-255
    pub experience: u64,         // XP points
    pub created_at: i64,
    pub last_active: i64,
    pub bump: u8,
}

// ============================================================================
// Enums
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    ProblemPosted,
    DirectionProposed,
    StakePlaced,
    DirectionWon,
    DirectionLost,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct ReputationInitialized {
    pub user: Pubkey,
    pub created_at: i64,
}

#[event]
pub struct ActionRecorded {
    pub user: Pubkey,
    pub action: ActionType,
    pub new_experience: u64,
}

#[event]
pub struct OutcomeRecorded {
    pub user: Pubkey,
    pub won: bool,
    pub earnings: u64,
    pub new_win_rate: u16,
    pub current_streak: i16,
}

#[event]
pub struct LevelUp {
    pub user: Pubkey,
    pub old_level: u8,
    pub new_level: u8,
    pub experience: u64,
}

#[event]
pub struct SyncedToSovereign {
    pub user: Pubkey,
    pub civic_score: u16,
}
