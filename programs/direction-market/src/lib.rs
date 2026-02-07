use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo, Burn};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("8yX75Q8Go26wGTmttuVQuzMvGFYa6mWPHtGXV8vHGZzt");

#[program]
pub mod direction_market {
    use super::*;

    /// Initialize the direction market config
    pub fn initialize(ctx: Context<Initialize>, fee_rate: u16) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.direction_count = 0;
        config.fee_rate = fee_rate;
        config.total_volume = 0;
        config.bump = ctx.bumps.config;

        msg!("Direction Market initialized with fee rate: {} bps", fee_rate);
        Ok(())
    }

    /// Propose a new direction (solution) for a problem
    pub fn propose_direction(
        ctx: Context<ProposeDirection>,
        description: String,
        ai_analysis: String,
    ) -> Result<()> {
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        require!(ai_analysis.len() <= 1000, ErrorCode::AnalysisTooLong);

        let config = &mut ctx.accounts.config;
        let direction = &mut ctx.accounts.direction;
        let problem = &ctx.accounts.problem;

        // Verify problem is open
        require!(
            problem.status == problem_registry::ProblemStatus::Open ||
            problem.status == problem_registry::ProblemStatus::InProgress,
            ErrorCode::ProblemNotAcceptingDirections
        );

        direction.id = config.direction_count;
        direction.problem = ctx.accounts.problem.key();
        direction.proposer = ctx.accounts.proposer.key();
        direction.description = description;
        direction.ai_analysis = ai_analysis;
        direction.yes_mint = ctx.accounts.yes_mint.key();
        direction.no_mint = ctx.accounts.no_mint.key();
        direction.yes_supply = 0;
        direction.no_supply = 0;
        direction.usdc_liquidity = 0;
        direction.is_settled = false;
        direction.outcome = None;
        direction.created_at = Clock::get()?.unix_timestamp;
        direction.bump = ctx.bumps.direction;

        config.direction_count += 1;

        emit!(DirectionProposed {
            direction_id: direction.id,
            problem: direction.problem,
            proposer: direction.proposer,
            description: direction.description.clone(),
        });

        msg!("Direction #{} proposed for problem", direction.id);
        Ok(())
    }

    /// Stake on a direction outcome (buy YES or NO tokens)
    pub fn stake_on_direction(
        ctx: Context<StakeOnDirection>,
        amount: u64,
        outcome: Outcome,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let direction = &mut ctx.accounts.direction;
        require!(!direction.is_settled, ErrorCode::MarketSettled);

        // Transfer USDC from staker to vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.staker_usdc.to_account_info(),
                to: ctx.accounts.direction_vault.to_account_info(),
                authority: ctx.accounts.staker.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        // Calculate tokens to mint based on CPMM formula
        // Simple linear formula for MVP: tokens = amount (1:1)
        let tokens_to_mint = amount;

        // Mint outcome tokens to staker
        // Extract values before building seeds to avoid borrow checker issues
        let problem_key = direction.problem;
        let direction_id = direction.id;
        let direction_bump = direction.bump;
        let id_bytes = direction_id.to_le_bytes();

        let seeds = &[
            b"direction".as_ref(),
            problem_key.as_ref(),
            id_bytes.as_ref(),
            &[direction_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        match outcome {
            Outcome::Yes => {
                let mint_ctx = CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.yes_mint.to_account_info(),
                        to: ctx.accounts.staker_yes_tokens.to_account_info(),
                        authority: direction.to_account_info(),
                    },
                    signer_seeds,
                );
                token::mint_to(mint_ctx, tokens_to_mint)?;
                direction.yes_supply = direction.yes_supply.checked_add(tokens_to_mint).unwrap();
            }
            Outcome::No => {
                let mint_ctx = CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.no_mint.to_account_info(),
                        to: ctx.accounts.staker_no_tokens.to_account_info(),
                        authority: direction.to_account_info(),
                    },
                    signer_seeds,
                );
                token::mint_to(mint_ctx, tokens_to_mint)?;
                direction.no_supply = direction.no_supply.checked_add(tokens_to_mint).unwrap();
            }
        }

        direction.usdc_liquidity = direction.usdc_liquidity.checked_add(amount).unwrap();

        // Update global volume
        let config = &mut ctx.accounts.config;
        config.total_volume = config.total_volume.checked_add(amount).unwrap();

        emit!(StakePlaced {
            direction_id: direction.id,
            staker: ctx.accounts.staker.key(),
            amount,
            outcome,
            tokens_minted: tokens_to_mint,
        });

        msg!("Staked {} USDC on {:?} outcome", amount, outcome);
        Ok(())
    }

    /// Unstake from a direction (sell tokens back)
    pub fn unstake(
        ctx: Context<Unstake>,
        amount: u64,
        outcome: Outcome,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let direction = &mut ctx.accounts.direction;
        require!(!direction.is_settled, ErrorCode::MarketSettled);

        // Calculate USDC to return (1:1 for MVP)
        let usdc_to_return = amount;

        // Burn outcome tokens
        // Extract values before building seeds to avoid borrow checker issues
        let problem_key = direction.problem;
        let direction_id = direction.id;
        let direction_bump = direction.bump;
        let id_bytes = direction_id.to_le_bytes();

        let seeds = &[
            b"direction".as_ref(),
            problem_key.as_ref(),
            id_bytes.as_ref(),
            &[direction_bump],
        ];
        let signer_seeds = &[&seeds[..]];

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
                direction.yes_supply = direction.yes_supply.checked_sub(amount).unwrap();
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
                direction.no_supply = direction.no_supply.checked_sub(amount).unwrap();
            }
        }

        // Transfer USDC back to staker
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.direction_vault.to_account_info(),
                to: ctx.accounts.staker_usdc.to_account_info(),
                authority: direction.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, usdc_to_return)?;

        direction.usdc_liquidity = direction.usdc_liquidity.checked_sub(usdc_to_return).unwrap();

        emit!(StakeWithdrawn {
            direction_id: direction.id,
            staker: ctx.accounts.staker.key(),
            amount,
            outcome,
            usdc_returned: usdc_to_return,
        });

        msg!("Unstaked {} tokens for {} USDC", amount, usdc_to_return);
        Ok(())
    }

    /// Settle a direction after problem resolution (authority only)
    pub fn settle_direction(ctx: Context<SettleDirection>, outcome: Outcome) -> Result<()> {
        let direction = &mut ctx.accounts.direction;

        require!(!direction.is_settled, ErrorCode::AlreadySettled);

        direction.is_settled = true;
        direction.outcome = Some(outcome);

        emit!(DirectionSettled {
            direction_id: direction.id,
            outcome,
            total_liquidity: direction.usdc_liquidity,
        });

        msg!("Direction #{} settled with outcome: {:?}", direction.id, outcome);
        Ok(())
    }

    /// Claim rewards after settlement
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let direction = &ctx.accounts.direction;

        require!(direction.is_settled, ErrorCode::MarketNotSettled);
        let outcome = direction.outcome.ok_or(ErrorCode::MarketNotSettled)?;

        let user_stake = &mut ctx.accounts.user_stake;
        require!(!user_stake.claimed, ErrorCode::AlreadyClaimed);

        // Determine winning token balance
        let winning_balance = match outcome {
            Outcome::Yes => ctx.accounts.staker_yes_tokens.amount,
            Outcome::No => ctx.accounts.staker_no_tokens.amount,
        };

        if winning_balance == 0 {
            user_stake.claimed = true;
            return Ok(());
        }

        // Calculate payout: winner takes proportional share of total pool
        let total_winning_supply = match outcome {
            Outcome::Yes => direction.yes_supply,
            Outcome::No => direction.no_supply,
        };

        let payout = if total_winning_supply > 0 {
            direction.usdc_liquidity
                .checked_mul(winning_balance)
                .unwrap()
                .checked_div(total_winning_supply)
                .unwrap()
        } else {
            0
        };

        // Extract values before building seeds to avoid borrow checker issues
        let problem_key = direction.problem;
        let direction_id = direction.id;
        let direction_bump = direction.bump;
        let id_bytes = direction_id.to_le_bytes();

        if payout > 0 {
            // Transfer USDC to winner
            let seeds = &[
                b"direction".as_ref(),
                problem_key.as_ref(),
                id_bytes.as_ref(),
                &[direction_bump],
            ];
            let signer_seeds = &[&seeds[..]];

            let transfer_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.direction_vault.to_account_info(),
                    to: ctx.accounts.staker_usdc.to_account_info(),
                    authority: ctx.accounts.direction.to_account_info(),
                },
                signer_seeds,
            );
            token::transfer(transfer_ctx, payout)?;
        }

        user_stake.claimed = true;

        emit!(RewardsClaimed {
            direction_id: direction.id,
            staker: ctx.accounts.staker.key(),
            amount: payout,
        });

        msg!("Claimed {} USDC in rewards", payout);
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
#[instruction(description: String, ai_analysis: String)]
pub struct ProposeDirection<'info> {
    #[account(
        mut,
        seeds = [b"market_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketConfig>,

    /// The problem this direction solves
    #[account(
        constraint = problem.status == problem_registry::ProblemStatus::Open ||
                     problem.status == problem_registry::ProblemStatus::InProgress
    )]
    pub problem: Account<'info, problem_registry::Problem>,

    #[account(
        init,
        payer = proposer,
        space = 8 + Direction::INIT_SPACE,
        seeds = [b"direction", problem.key().as_ref(), config.direction_count.to_le_bytes().as_ref()],
        bump
    )]
    pub direction: Account<'info, Direction>,

    /// YES outcome token mint
    #[account(
        init,
        payer = proposer,
        seeds = [b"yes_mint", direction.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = direction,
    )]
    pub yes_mint: Account<'info, Mint>,

    /// NO outcome token mint
    #[account(
        init,
        payer = proposer,
        seeds = [b"no_mint", direction.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = direction,
    )]
    pub no_mint: Account<'info, Mint>,

    #[account(mut)]
    pub proposer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeOnDirection<'info> {
    #[account(
        mut,
        seeds = [b"market_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketConfig>,

    #[account(
        mut,
        seeds = [b"direction", direction.problem.as_ref(), direction.id.to_le_bytes().as_ref()],
        bump = direction.bump
    )]
    pub direction: Account<'info, Direction>,

    #[account(
        init_if_needed,
        payer = staker,
        seeds = [b"direction_vault", direction.key().as_ref()],
        bump,
        token::mint = usdc_mint,
        token::authority = direction,
    )]
    pub direction_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = direction.yes_mint == yes_mint.key()
    )]
    pub yes_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = direction.no_mint == no_mint.key()
    )]
    pub no_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = staker_usdc.mint == usdc_mint.key(),
        constraint = staker_usdc.owner == staker.key()
    )]
    pub staker_usdc: Account<'info, TokenAccount>,

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
        space = 8 + UserStake::INIT_SPACE,
        seeds = [b"user_stake", direction.key().as_ref(), staker.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    pub usdc_mint: Account<'info, Mint>,

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
        seeds = [b"direction", direction.problem.as_ref(), direction.id.to_le_bytes().as_ref()],
        bump = direction.bump
    )]
    pub direction: Account<'info, Direction>,

    #[account(
        mut,
        seeds = [b"direction_vault", direction.key().as_ref()],
        bump,
    )]
    pub direction_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = direction.yes_mint == yes_mint.key()
    )]
    pub yes_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = direction.no_mint == no_mint.key()
    )]
    pub no_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = staker_usdc.owner == staker.key()
    )]
    pub staker_usdc: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = yes_mint,
        associated_token::authority = staker,
    )]
    pub staker_yes_tokens: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = no_mint,
        associated_token::authority = staker,
    )]
    pub staker_no_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staker: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SettleDirection<'info> {
    #[account(
        seeds = [b"market_config"],
        bump = config.bump,
        constraint = config.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub config: Account<'info, MarketConfig>,

    #[account(
        mut,
        seeds = [b"direction", direction.problem.as_ref(), direction.id.to_le_bytes().as_ref()],
        bump = direction.bump
    )]
    pub direction: Account<'info, Direction>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        seeds = [b"direction", direction.problem.as_ref(), direction.id.to_le_bytes().as_ref()],
        bump = direction.bump
    )]
    pub direction: Account<'info, Direction>,

    #[account(
        mut,
        seeds = [b"direction_vault", direction.key().as_ref()],
        bump,
    )]
    pub direction_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"user_stake", direction.key().as_ref(), staker.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(
        mut,
        constraint = staker_usdc.owner == staker.key()
    )]
    pub staker_usdc: Account<'info, TokenAccount>,

    #[account(
        associated_token::mint = yes_mint,
        associated_token::authority = staker,
    )]
    pub staker_yes_tokens: Account<'info, TokenAccount>,

    #[account(
        associated_token::mint = no_mint,
        associated_token::authority = staker,
    )]
    pub staker_no_tokens: Account<'info, TokenAccount>,

    #[account(constraint = direction.yes_mint == yes_mint.key())]
    pub yes_mint: Account<'info, Mint>,

    #[account(constraint = direction.no_mint == no_mint.key())]
    pub no_mint: Account<'info, Mint>,

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
    pub direction_count: u64,
    pub fee_rate: u16,        // Basis points
    pub total_volume: u64,    // Total USDC traded
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Direction {
    pub id: u64,
    pub problem: Pubkey,
    pub proposer: Pubkey,
    #[max_len(500)]
    pub description: String,
    #[max_len(1000)]
    pub ai_analysis: String,
    pub yes_mint: Pubkey,
    pub no_mint: Pubkey,
    pub yes_supply: u64,
    pub no_supply: u64,
    pub usdc_liquidity: u64,
    pub is_settled: bool,
    pub outcome: Option<Outcome>,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserStake {
    pub user: Pubkey,
    pub direction: Pubkey,
    pub claimed: bool,
    pub bump: u8,
}

// ============================================================================
// Enums
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum Outcome {
    Yes,  // Solution works
    No,   // Solution fails
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct DirectionProposed {
    pub direction_id: u64,
    pub problem: Pubkey,
    pub proposer: Pubkey,
    pub description: String,
}

#[event]
pub struct StakePlaced {
    pub direction_id: u64,
    pub staker: Pubkey,
    pub amount: u64,
    pub outcome: Outcome,
    pub tokens_minted: u64,
}

#[event]
pub struct StakeWithdrawn {
    pub direction_id: u64,
    pub staker: Pubkey,
    pub amount: u64,
    pub outcome: Outcome,
    pub usdc_returned: u64,
}

#[event]
pub struct DirectionSettled {
    pub direction_id: u64,
    pub outcome: Outcome,
    pub total_liquidity: u64,
}

#[event]
pub struct RewardsClaimed {
    pub direction_id: u64,
    pub staker: Pubkey,
    pub amount: u64,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Description exceeds maximum length of 500 characters")]
    DescriptionTooLong,
    #[msg("AI analysis exceeds maximum length of 1000 characters")]
    AnalysisTooLong,
    #[msg("Problem is not accepting new directions")]
    ProblemNotAcceptingDirections,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Market has already been settled")]
    MarketSettled,
    #[msg("Market has already been settled")]
    AlreadySettled,
    #[msg("Market has not been settled yet")]
    MarketNotSettled,
    #[msg("Rewards already claimed")]
    AlreadyClaimed,
    #[msg("Unauthorized action")]
    Unauthorized,
}
