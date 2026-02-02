use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("BYbmC2bdPSNELqpXqCXY9iKiDMf9hUWFdcExGiHUZSRZ");

#[program]
pub mod treasury {
    use super::*;

    /// Initialize the treasury
    pub fn initialize(ctx: Context<Initialize>, fee_rate: u16) -> Result<()> {
        require!(fee_rate <= 1000, ErrorCode::FeeTooHigh); // Max 10%

        let treasury = &mut ctx.accounts.treasury;
        treasury.authority = ctx.accounts.authority.key();
        treasury.usdc_mint = ctx.accounts.usdc_mint.key();
        treasury.total_deposits = 0;
        treasury.total_payouts = 0;
        treasury.total_fees_collected = 0;
        treasury.fee_rate = fee_rate;
        treasury.bump = ctx.bumps.treasury;

        msg!("Treasury initialized with fee rate: {} bps", fee_rate);
        Ok(())
    }

    /// Deposit USDC into treasury
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        // Transfer USDC to treasury vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.depositor_token_account.to_account_info(),
                to: ctx.accounts.treasury_vault.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        let treasury = &mut ctx.accounts.treasury;
        treasury.total_deposits = treasury.total_deposits.checked_add(amount).unwrap();

        emit!(TreasuryDeposit {
            depositor: ctx.accounts.depositor.key(),
            amount,
            total_deposits: treasury.total_deposits,
        });

        msg!("Deposited {} USDC to treasury", amount);
        Ok(())
    }

    /// Allocate funds to a problem bounty
    pub fn allocate_to_problem(
        ctx: Context<AllocateToProblem>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(
            ctx.accounts.treasury_vault.amount >= amount,
            ErrorCode::InsufficientFunds
        );

        let treasury = &ctx.accounts.treasury;
        let seeds = &[b"treasury".as_ref(), &[treasury.bump]];
        let signer_seeds = &[&seeds[..]];

        // Transfer to problem vault
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury_vault.to_account_info(),
                to: ctx.accounts.problem_vault.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, amount)?;

        emit!(FundsAllocated {
            problem: ctx.accounts.problem_vault.key(),
            amount,
        });

        msg!("Allocated {} USDC to problem", amount);
        Ok(())
    }

    /// Payout rewards to a recipient
    pub fn payout(ctx: Context<Payout>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(
            ctx.accounts.treasury_vault.amount >= amount,
            ErrorCode::InsufficientFunds
        );

        let treasury = &mut ctx.accounts.treasury;

        // Calculate fee
        let fee = amount
            .checked_mul(treasury.fee_rate as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();
        let payout_amount = amount.checked_sub(fee).unwrap();

        let seeds = &[b"treasury".as_ref(), &[treasury.bump]];
        let signer_seeds = &[&seeds[..]];

        // Transfer to recipient (minus fee)
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury_vault.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: treasury.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, payout_amount)?;

        treasury.total_payouts = treasury.total_payouts.checked_add(payout_amount).unwrap();
        treasury.total_fees_collected = treasury.total_fees_collected.checked_add(fee).unwrap();

        emit!(TreasuryPayout {
            recipient: ctx.accounts.recipient.key(),
            gross_amount: amount,
            fee,
            net_amount: payout_amount,
        });

        msg!("Paid out {} USDC (fee: {} USDC)", payout_amount, fee);
        Ok(())
    }

    /// Withdraw collected fees (authority only)
    pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let treasury = &mut ctx.accounts.treasury;
        require!(
            amount <= treasury.total_fees_collected,
            ErrorCode::InsufficientFees
        );

        let seeds = &[b"treasury".as_ref(), &[treasury.bump]];
        let signer_seeds = &[&seeds[..]];

        // Transfer fees to authority
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury_vault.to_account_info(),
                to: ctx.accounts.authority_token_account.to_account_info(),
                authority: treasury.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, amount)?;

        treasury.total_fees_collected = treasury.total_fees_collected.checked_sub(amount).unwrap();

        emit!(FeesWithdrawn {
            authority: ctx.accounts.authority.key(),
            amount,
            remaining_fees: treasury.total_fees_collected,
        });

        msg!("Withdrew {} USDC in fees", amount);
        Ok(())
    }

    /// Update fee rate (authority only)
    pub fn update_fee_rate(ctx: Context<UpdateConfig>, new_fee_rate: u16) -> Result<()> {
        require!(new_fee_rate <= 1000, ErrorCode::FeeTooHigh); // Max 10%

        let treasury = &mut ctx.accounts.treasury;
        let old_rate = treasury.fee_rate;
        treasury.fee_rate = new_fee_rate;

        emit!(FeeRateUpdated {
            old_rate,
            new_rate: new_fee_rate,
        });

        msg!("Fee rate updated from {} to {} bps", old_rate, new_fee_rate);
        Ok(())
    }

    /// Transfer authority (authority only)
    pub fn transfer_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        let old_authority = treasury.authority;
        treasury.authority = new_authority;

        emit!(AuthorityTransferred {
            old_authority,
            new_authority,
        });

        msg!("Authority transferred to {}", new_authority);
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
        space = 8 + Treasury::INIT_SPACE,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init,
        payer = authority,
        seeds = [b"treasury_vault"],
        bump,
        token::mint = usdc_mint,
        token::authority = treasury,
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [b"treasury_vault"],
        bump,
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = depositor_token_account.mint == treasury.usdc_mint,
        constraint = depositor_token_account.owner == depositor.key()
    )]
    pub depositor_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AllocateToProblem<'info> {
    #[account(
        seeds = [b"treasury"],
        bump = treasury.bump,
        constraint = treasury.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [b"treasury_vault"],
        bump,
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    /// The problem's USDC vault
    #[account(mut)]
    pub problem_vault: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
        constraint = treasury.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [b"treasury_vault"],
        bump,
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = recipient_token_account.mint == treasury.usdc_mint,
        constraint = recipient_token_account.owner == recipient.key()
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    /// CHECK: Recipient doesn't need to sign
    pub recipient: UncheckedAccount<'info>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
        constraint = treasury.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [b"treasury_vault"],
        bump,
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = authority_token_account.mint == treasury.usdc_mint,
        constraint = authority_token_account.owner == authority.key()
    )]
    pub authority_token_account: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
        constraint = treasury.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub treasury: Account<'info, Treasury>,

    pub authority: Signer<'info>,
}

// ============================================================================
// State
// ============================================================================

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub authority: Pubkey,
    pub usdc_mint: Pubkey,
    pub total_deposits: u64,
    pub total_payouts: u64,
    pub total_fees_collected: u64,
    pub fee_rate: u16,          // Basis points (e.g., 250 = 2.5%)
    pub bump: u8,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct TreasuryDeposit {
    pub depositor: Pubkey,
    pub amount: u64,
    pub total_deposits: u64,
}

#[event]
pub struct FundsAllocated {
    pub problem: Pubkey,
    pub amount: u64,
}

#[event]
pub struct TreasuryPayout {
    pub recipient: Pubkey,
    pub gross_amount: u64,
    pub fee: u64,
    pub net_amount: u64,
}

#[event]
pub struct FeesWithdrawn {
    pub authority: Pubkey,
    pub amount: u64,
    pub remaining_fees: u64,
}

#[event]
pub struct FeeRateUpdated {
    pub old_rate: u16,
    pub new_rate: u16,
}

#[event]
pub struct AuthorityTransferred {
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Fee rate cannot exceed 10% (1000 basis points)")]
    FeeTooHigh,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Insufficient funds in treasury")]
    InsufficientFunds,
    #[msg("Insufficient fees to withdraw")]
    InsufficientFees,
    #[msg("Unauthorized action")]
    Unauthorized,
}
