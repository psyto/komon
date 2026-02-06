use anchor_lang::prelude::*;

/// SOVEREIGN Program ID
pub const SOVEREIGN_PROGRAM_ID: Pubkey = pubkey!("2UAZc1jj4QTSkgrC8U9d4a7EM9AQunxMvW5g7rX7Af9T");

/// SOVEREIGN Identity account structure (for deserialization)
/// This allows Komon to read SOVEREIGN identity accounts
#[account]
#[derive(Default)]
pub struct SovereignIdentity {
    pub owner: Pubkey,
    pub created_at: i64,
    pub trading_authority: Pubkey,
    pub civic_authority: Pubkey,
    pub developer_authority: Pubkey,
    pub infra_authority: Pubkey,
    pub trading_score: u16,
    pub civic_score: u16,
    pub developer_score: u16,
    pub infra_score: u16,
    pub composite_score: u16,
    pub tier: u8,
    pub last_updated: i64,
    pub bump: u8,
}

impl SovereignIdentity {
    pub const SIZE: usize = 8 + 32 + 8 + 32 + 32 + 32 + 32 + 2 + 2 + 2 + 2 + 2 + 1 + 8 + 1;
}

/// Derive the PDA for a user's SOVEREIGN identity
pub fn get_identity_pda(owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"identity", owner.as_ref()],
        &SOVEREIGN_PROGRAM_ID,
    )
}

/// Calculate civic score from Komon reputation data
/// Returns a score from 0-10000 (basis points)
pub fn calculate_civic_score(
    problems_posted: u32,
    directions_proposed: u32,
    directions_won: u32,
    directions_lost: u32,
    win_rate: u16,
    current_streak: i16,
    level: u8,
) -> u16 {
    // Weighted formula for civic score:
    // - Win rate: 40% (directly maps)
    // - Problems solved contribution: 25% (based on directions won)
    // - Activity level: 25% (based on level)
    // - Streak bonus: 10%

    let win_rate_component = (win_rate as u32) * 40 / 100;

    // Directions won tier (similar to SOVEREIGN's solved_tier)
    let solved_tier = match directions_won {
        0..=2 => 2000u32,
        3..=10 => 4000,
        11..=50 => 6000,
        51..=200 => 8000,
        _ => 10000,
    };
    let solved_component = solved_tier * 25 / 100;

    // Level contribution (max level ~255, scale to 10000)
    let level_score = ((level as u32) * 10000 / 50).min(10000);
    let level_component = level_score * 25 / 100;

    // Streak bonus
    let streak_tier = match current_streak.abs() as u16 {
        0..=2 => 2000u32,
        3..=5 => 4000,
        6..=10 => 6000,
        11..=20 => 8000,
        _ => 10000,
    };
    // Positive streak gets full bonus, negative streak gets reduced
    let streak_component = if current_streak >= 0 {
        streak_tier * 10 / 100
    } else {
        streak_tier * 5 / 100
    };

    (win_rate_component + solved_component + level_component + streak_component).min(10000) as u16
}

/// Tier-based access levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TierAccess {
    /// Basic access (Tier 1-2)
    Basic,
    /// Standard access (Tier 3)
    Standard,
    /// Premium access (Tier 4-5)
    Premium,
}

/// Get access level from SOVEREIGN tier
pub fn get_access_level(tier: u8) -> TierAccess {
    match tier {
        1..=2 => TierAccess::Basic,
        3 => TierAccess::Standard,
        4..=5 => TierAccess::Premium,
        _ => TierAccess::Basic,
    }
}

/// Get stake limit based on SOVEREIGN tier
pub fn get_stake_limit(tier: u8) -> u64 {
    match tier {
        1 => 100_000_000,       // 100 USDC
        2 => 500_000_000,       // 500 USDC
        3 => 2_000_000_000,     // 2,000 USDC
        4 => 10_000_000_000,    // 10,000 USDC
        5 => u64::MAX,          // Unlimited
        _ => 100_000_000,       // Default: 100 USDC
    }
}
