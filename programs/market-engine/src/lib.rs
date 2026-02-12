use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo, Burn};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("5tFcB5fM3f19PB2CCqjBKvaNFqXWKgNjMALWknG291Qq");

// =============================================================================
// MARKET ENGINE - Generic Core
// =============================================================================
//
// Shared prediction market engine for both framings:
// - Civic: "Directions" (proposed solutions to problems)
// - Creator: "Predictions" (will DAO accept this creator?)
//
// Core mechanism:
// 1. Create market on a subject outcome
// 2. Stake on YES/NO
// 3. Settlement based on subject resolution
// 4. Winners claim proportional payout
// 5. Optional burn (Creator mode only)
// =============================================================================

#[program]
pub mod market_engine {
    use super::*;

    /// Initialize market engine config
    pub fn initialize(
        ctx: Context<Initialize>,
        fee_rate: u16,
        burn_enabled: bool,
        burn_rate_bps: u16,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.market_count = 0;
        config.fee_rate = fee_rate;
        config.burn_enabled = burn_enabled;
        config.burn_rate_bps = burn_rate_bps;
        config.total_volume = 0;
        config.total_burned = 0;
        config.bump = ctx.bumps.config;

        msg!(
            "Market Engine initialized (burn: {}, rate: {} bps)",
            burn_enabled,
            burn_rate_bps
        );
        Ok(())
    }

    /// Create a new market on a subject
    pub fn create_market(
        ctx: Context<CreateMarket>,
        description: String,
        analysis: String,
    ) -> Result<()> {
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        require!(analysis.len() <= 1000, ErrorCode::AnalysisTooLong);

        let config = &mut ctx.accounts.config;
        let market = &mut ctx.accounts.market;

        market.id = config.market_count;
        market.subject = ctx.accounts.subject.key();
        market.creator = ctx.accounts.creator.key();
        market.description = description.clone();
        market.analysis = analysis;
        market.yes_mint = ctx.accounts.yes_mint.key();
        market.no_mint = ctx.accounts.no_mint.key();
        market.yes_supply = 0;
        market.no_supply = 0;
        market.liquidity = 0;
        market.is_settled = false;
        market.outcome = None;
        market.created_at = Clock::get()?.unix_timestamp;
        market.bump = ctx.bumps.market;

        config.market_count += 1;

        emit!(MarketCreated {
            market_id: market.id,
            subject: market.subject,
            creator: market.creator,
            description,
        });

        msg!("Market #{} created for subject", market.id);
        Ok(())
    }

    /// Stake on market outcome (buy YES or NO tokens)
    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
        outcome: Outcome,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let market = &mut ctx.accounts.market;
        require!(!market.is_settled, ErrorCode::MarketSettled);

        // Extract values for seeds before mutable operations
        let subject_key = market.subject;
        let market_id = market.id;
        let market_bump = market.bump;
        let id_bytes = market_id.to_le_bytes();

        // Transfer stake to vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.staker_tokens.to_account_info(),
                to: ctx.accounts.market_vault.to_account_info(),
                authority: ctx.accounts.staker.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        // Calculate tokens (1:1 for simplicity; can upgrade to CPMM)
        let tokens_to_mint = amount;

        // Build seeds with extracted values
        let seeds = &[
            b"market".as_ref(),
            subject_key.as_ref(),
            id_bytes.as_ref(),
            &[market_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        match outcome {
            Outcome::Yes => {
                let mint_ctx = CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.yes_mint.to_account_info(),
                        to: ctx.accounts.staker_yes_tokens.to_account_info(),
                        authority: market.to_account_info(),
                    },
                    signer_seeds,
                );
                token::mint_to(mint_ctx, tokens_to_mint)?;
                market.yes_supply = market.yes_supply.checked_add(tokens_to_mint)
                    .ok_or(error!(ErrorCode::ArithmeticOverflow))?;
            }
            Outcome::No => {
                let mint_ctx = CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.no_mint.to_account_info(),
                        to: ctx.accounts.staker_no_tokens.to_account_info(),
                        authority: market.to_account_info(),
                    },
                    signer_seeds,
                );
                token::mint_to(mint_ctx, tokens_to_mint)?;
                market.no_supply = market.no_supply.checked_add(tokens_to_mint)
                    .ok_or(error!(ErrorCode::ArithmeticOverflow))?;
            }
        }

        market.liquidity = market.liquidity.checked_add(amount)
            .ok_or(error!(ErrorCode::ArithmeticOverflow))?;

        // Update global volume
        let config = &mut ctx.accounts.config;
        config.total_volume = config.total_volume.checked_add(amount)
            .ok_or(error!(ErrorCode::ArithmeticOverflow))?;

        emit!(StakePlaced {
            market_id,
            staker: ctx.accounts.staker.key(),
            amount,
            outcome,
            tokens_minted: tokens_to_mint,
        });

        Ok(())
    }

    /// Unstake (sell tokens back before settlement)
    pub fn unstake(
        ctx: Context<Unstake>,
        amount: u64,
        outcome: Outcome,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let market = &mut ctx.accounts.market;
        require!(!market.is_settled, ErrorCode::MarketSettled);

        let tokens_to_return = amount; // 1:1

        // Extract values for seeds before mutable operations
        let subject_key = market.subject;
        let market_id = market.id;
        let market_bump = market.bump;

        // Burn outcome tokens (user authority, no seeds needed)
        match outcome {
            Outcome::Yes => {
                let burn_ctx = CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Burn {
                        mint: ctx.accounts.yes_mint.to_account_info(),
                        from: ctx.accounts.staker_yes_tokens.to_account_info(),
                        authority: ctx.accounts.staker.to_account_info(),
                    },
                );
                token::burn(burn_ctx, amount)?;
                market.yes_supply = market.yes_supply.checked_sub(amount)
                    .ok_or(error!(ErrorCode::ArithmeticOverflow))?;
            }
            Outcome::No => {
                let burn_ctx = CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Burn {
                        mint: ctx.accounts.no_mint.to_account_info(),
                        from: ctx.accounts.staker_no_tokens.to_account_info(),
                        authority: ctx.accounts.staker.to_account_info(),
                    },
                );
                token::burn(burn_ctx, amount)?;
                market.no_supply = market.no_supply.checked_sub(amount)
                    .ok_or(error!(ErrorCode::ArithmeticOverflow))?;
            }
        }

        // Build seeds with extracted values
        let id_bytes = market_id.to_le_bytes();
        let seeds = &[
            b"market".as_ref(),
            subject_key.as_ref(),
            id_bytes.as_ref(),
            &[market_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Transfer tokens back
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.market_vault.to_account_info(),
                to: ctx.accounts.staker_tokens.to_account_info(),
                authority: market.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, tokens_to_return)?;

        market.liquidity = market.liquidity.checked_sub(tokens_to_return)
            .ok_or(error!(ErrorCode::ArithmeticOverflow))?;

        emit!(StakeWithdrawn {
            market_id,
            staker: ctx.accounts.staker.key(),
            amount,
            outcome,
        });

        Ok(())
    }

    /// Settle market after subject resolution
    pub fn settle_market(ctx: Context<SettleMarket>, outcome: Outcome) -> Result<()> {
        let market = &mut ctx.accounts.market;

        require!(!market.is_settled, ErrorCode::AlreadySettled);

        market.is_settled = true;
        market.outcome = Some(outcome);

        emit!(MarketSettled {
            market_id: market.id,
            outcome,
            total_liquidity: market.liquidity,
        });

        msg!("Market #{} settled: {:?}", market.id, outcome);
        Ok(())
    }

    /// Claim rewards after settlement
    /// Includes burn mechanism for Creator mode
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let market = &ctx.accounts.market;
        let config = &mut ctx.accounts.config;

        require!(market.is_settled, ErrorCode::MarketNotSettled);
        let outcome = market.outcome.ok_or(ErrorCode::MarketNotSettled)?;

        let user_position = &mut ctx.accounts.user_position;
        require!(!user_position.claimed, ErrorCode::AlreadyClaimed);

        // Extract values for seeds
        let subject_key = market.subject;
        let market_id = market.id;
        let market_bump = market.bump;
        let id_bytes = market_id.to_le_bytes();

        // Determine winning token balance
        let winning_balance = match outcome {
            Outcome::Yes => ctx.accounts.staker_yes_tokens.amount,
            Outcome::No => ctx.accounts.staker_no_tokens.amount,
        };

        if winning_balance == 0 {
            user_position.claimed = true;
            return Ok(());
        }

        // Calculate payout
        let total_winning_supply = match outcome {
            Outcome::Yes => market.yes_supply,
            Outcome::No => market.no_supply,
        };

        let gross_payout = if total_winning_supply > 0 {
            market.liquidity
                .checked_mul(winning_balance)
                .ok_or(error!(ErrorCode::ArithmeticOverflow))?
                .checked_div(total_winning_supply)
                .ok_or(error!(ErrorCode::ArithmeticOverflow))?
        } else {
            0
        };

        // === BURN MECHANISM (Creator mode) ===
        // Vitalik: "a portion of their proceeds from the DAO are used to burn"
        let (burn_amount, net_payout) = if config.burn_enabled && gross_payout > 0 {
            let burn = gross_payout
                .checked_mul(config.burn_rate_bps as u64)
                .ok_or(error!(ErrorCode::ArithmeticOverflow))?
                .checked_div(10000)
                .ok_or(error!(ErrorCode::ArithmeticOverflow))?;
            config.total_burned = config.total_burned.checked_add(burn)
                .ok_or(error!(ErrorCode::ArithmeticOverflow))?;
            (burn, gross_payout - burn)
        } else {
            (0, gross_payout)
        };

        // Build seeds once with extracted values
        let seeds = &[
            b"market".as_ref(),
            subject_key.as_ref(),
            id_bytes.as_ref(),
            &[market_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        if net_payout > 0 {
            let transfer_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.market_vault.to_account_info(),
                    to: ctx.accounts.staker_tokens.to_account_info(),
                    authority: ctx.accounts.market.to_account_info(),
                },
                signer_seeds,
            );
            token::transfer(transfer_ctx, net_payout)?;
        }

        // Transfer burn amount to burn treasury (if enabled)
        if burn_amount > 0 {
            let transfer_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.market_vault.to_account_info(),
                    to: ctx.accounts.burn_treasury.to_account_info(),
                    authority: ctx.accounts.market.to_account_info(),
                },
                signer_seeds,
            );
            token::transfer(transfer_ctx, burn_amount)?;
        }

        user_position.claimed = true;

        emit!(RewardsClaimed {
            market_id,
            staker: ctx.accounts.staker.key(),
            gross_payout,
            burn_amount,
            net_payout,
        });

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
        space = 8 + MarketConfig::INIT_SPACE,
        seeds = [b"market_config"],
        bump
    )]
    pub config: Account<'info, MarketConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(description: String, analysis: String)]
pub struct CreateMarket<'info> {
    #[account(
        mut,
        seeds = [b"market_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketConfig>,

    /// The subject this market is for
    /// CHECK: Validated by subject-registry
    pub subject: UncheckedAccount<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Market::INIT_SPACE,
        seeds = [b"market", subject.key().as_ref(), config.market_count.to_le_bytes().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = creator,
        seeds = [b"yes_mint", market.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = market,
    )]
    pub yes_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = creator,
        seeds = [b"no_mint", market.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = market,
    )]
    pub no_mint: Account<'info, Mint>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [b"market_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketConfig>,

    #[account(
        mut,
        seeds = [b"market", market.subject.as_ref(), market.id.to_le_bytes().as_ref()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init_if_needed,
        payer = staker,
        seeds = [b"market_vault", market.key().as_ref()],
        bump,
        token::mint = stake_mint,
        token::authority = market,
    )]
    pub market_vault: Account<'info, TokenAccount>,

    #[account(mut, constraint = market.yes_mint == yes_mint.key())]
    pub yes_mint: Account<'info, Mint>,

    #[account(mut, constraint = market.no_mint == no_mint.key())]
    pub no_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = staker_tokens.mint == stake_mint.key(),
        constraint = staker_tokens.owner == staker.key()
    )]
    pub staker_tokens: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = staker,
        associated_token::mint = yes_mint,
        associated_token::authority = staker,
    )]
    pub staker_yes_tokens: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = staker,
        associated_token::mint = no_mint,
        associated_token::authority = staker,
    )]
    pub staker_no_tokens: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = staker,
        space = 8 + UserPosition::INIT_SPACE,
        seeds = [b"user_position", market.key().as_ref(), staker.key().as_ref()],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,

    pub stake_mint: Account<'info, Mint>,

    #[account(mut)]
    pub staker: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        mut,
        seeds = [b"market", market.subject.as_ref(), market.id.to_le_bytes().as_ref()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(mut, seeds = [b"market_vault", market.key().as_ref()], bump)]
    pub market_vault: Account<'info, TokenAccount>,

    #[account(mut, constraint = market.yes_mint == yes_mint.key())]
    pub yes_mint: Account<'info, Mint>,

    #[account(mut, constraint = market.no_mint == no_mint.key())]
    pub no_mint: Account<'info, Mint>,

    #[account(mut, constraint = staker_tokens.owner == staker.key())]
    pub staker_tokens: Account<'info, TokenAccount>,

    #[account(mut, associated_token::mint = yes_mint, associated_token::authority = staker)]
    pub staker_yes_tokens: Account<'info, TokenAccount>,

    #[account(mut, associated_token::mint = no_mint, associated_token::authority = staker)]
    pub staker_no_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staker: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SettleMarket<'info> {
    #[account(
        seeds = [b"market_config"],
        bump = config.bump,
        constraint = config.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub config: Account<'info, MarketConfig>,

    #[account(
        mut,
        seeds = [b"market", market.subject.as_ref(), market.id.to_le_bytes().as_ref()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"market_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketConfig>,

    #[account(
        seeds = [b"market", market.subject.as_ref(), market.id.to_le_bytes().as_ref()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(mut, seeds = [b"market_vault", market.key().as_ref()], bump)]
    pub market_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"user_position", market.key().as_ref(), staker.key().as_ref()],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,

    #[account(mut, constraint = staker_tokens.owner == staker.key())]
    pub staker_tokens: Account<'info, TokenAccount>,

    #[account(associated_token::mint = yes_mint, associated_token::authority = staker)]
    pub staker_yes_tokens: Account<'info, TokenAccount>,

    #[account(associated_token::mint = no_mint, associated_token::authority = staker)]
    pub staker_no_tokens: Account<'info, TokenAccount>,

    #[account(constraint = market.yes_mint == yes_mint.key())]
    pub yes_mint: Account<'info, Mint>,

    #[account(constraint = market.no_mint == no_mint.key())]
    pub no_mint: Account<'info, Mint>,

    /// Burn treasury (receives burn amount in Creator mode)
    /// CHECK: Can be any account
    #[account(mut)]
    pub burn_treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub staker: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

// ============================================================================
// State
// ============================================================================

#[account]
#[derive(InitSpace)]
pub struct MarketConfig {
    pub authority: Pubkey,
    pub market_count: u64,
    pub fee_rate: u16,
    pub burn_enabled: bool,      // true for Creator mode
    pub burn_rate_bps: u16,      // e.g., 500 = 5%
    pub total_volume: u64,
    pub total_burned: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub id: u64,
    pub subject: Pubkey,
    pub creator: Pubkey,
    #[max_len(500)]
    pub description: String,
    #[max_len(1000)]
    pub analysis: String,
    pub yes_mint: Pubkey,
    pub no_mint: Pubkey,
    pub yes_supply: u64,
    pub no_supply: u64,
    pub liquidity: u64,
    pub is_settled: bool,
    pub outcome: Option<Outcome>,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserPosition {
    pub user: Pubkey,
    pub market: Pubkey,
    pub claimed: bool,
    pub bump: u8,
}

// ============================================================================
// Enums
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum Outcome {
    Yes,
    No,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct MarketCreated {
    pub market_id: u64,
    pub subject: Pubkey,
    pub creator: Pubkey,
    pub description: String,
}

#[event]
pub struct StakePlaced {
    pub market_id: u64,
    pub staker: Pubkey,
    pub amount: u64,
    pub outcome: Outcome,
    pub tokens_minted: u64,
}

#[event]
pub struct StakeWithdrawn {
    pub market_id: u64,
    pub staker: Pubkey,
    pub amount: u64,
    pub outcome: Outcome,
}

#[event]
pub struct MarketSettled {
    pub market_id: u64,
    pub outcome: Outcome,
    pub total_liquidity: u64,
}

#[event]
pub struct RewardsClaimed {
    pub market_id: u64,
    pub staker: Pubkey,
    pub gross_payout: u64,
    pub burn_amount: u64,
    pub net_payout: u64,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Analysis too long")]
    AnalysisTooLong,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Market already settled")]
    MarketSettled,
    #[msg("Market already settled")]
    AlreadySettled,
    #[msg("Market not yet settled")]
    MarketNotSettled,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
