//! Struct definitions for accounts that hold state.

use crate::*;
use num_traits::cast::ToPrimitive;

/// Number of periods in an era.
pub const ERA_NUM_PERIODS: usize = (u8::MAX as usize) + 1;

/// Number of seconds in a period.
pub const PERIOD_SECONDS: u32 = 86_400 * 3;

/// Number of seconds in an era.
pub const SECONDS_PER_ERA: u64 = (ERA_NUM_PERIODS as u64) * (PERIOD_SECONDS as u64);

/// The Unix timestamp of the start of the first era.
pub const COMMON_ERA_UNIX_TS: u64 = 1640995200;

/// Calculates the start timestamp of an era.
pub fn calculate_era_start_ts(era: u16) -> Option<u64> {
    COMMON_ERA_UNIX_TS.checked_add(SECONDS_PER_ERA.checked_mul(era.into())?)
}

/// Calculates the start timestamp of a period of an era.
pub fn calculate_period_start_ts(era: u16, period: u8) -> Option<u64> {
    calculate_era_start_ts(era)?
        .checked_add(period.to_u64()?.checked_mul(PERIOD_SECONDS.to_u64()?)?)
}

/// Stores the total number of veTokens for each Solana rent epoch.
///
/// The [LockerHistory] account stores 1 year worth of epochs.
/// For a 5-year [locked_voter::Locker], there will be at least 6 of these accounts existing
/// at any given time, since the maximum lock period is 5 years.
#[account(zero_copy)]
#[derive(Debug)]
pub struct LockerHistory {
    /// The [locked_voter::Locker] being tracked.
    pub locker: Pubkey,
    /// The era. Multiplying this by [ERA_NUM_PERIODS] * [PERIOD_SECONDS];
    pub era: u16,
    /// Bump seed.
    pub bump: u8,
    /// Padding for aligning the struct to an 8-byte boundary.
    pub _padding: [u8; 5],
    /// The sum of all tracked historical vote escrow balances.
    pub ve_balances: [u64; 256],
    /// Number of voters with non-zero balances at each epoch.
    pub ve_counts: [u64; 256],
}

impl Default for LockerHistory {
    fn default() -> Self {
        Self {
            locker: Default::default(),
            era: Default::default(),
            bump: Default::default(),
            _padding: Default::default(),
            ve_balances: [0; ERA_NUM_PERIODS],
            ve_counts: [0; ERA_NUM_PERIODS],
        }
    }
}

/// Stores the total veToken balance of an [locked_voter::Escrow]
/// for the given epochs.
///
/// Any time someone refreshes and/or modifies their vote [locked_voter::Escrow], they
/// should refresh their [EscrowHistory] accounts.
#[account(zero_copy)]
#[derive(Debug)]
pub struct EscrowHistory {
    /// The [locked_voter::Escrow] being tracked.
    pub escrow: Pubkey,
    /// The era.
    pub era: u16,
    /// Bump seed.
    pub bump: u8,
    /// Padding for aligning the struct to an 8-byte boundary.
    pub _padding: [u8; 5],
    /// All tracked historical vote escrow balances for this [locked_voter::Escrow].
    pub ve_balances: [u64; 256],
}

impl Default for EscrowHistory {
    fn default() -> Self {
        Self {
            escrow: Default::default(),
            era: Default::default(),
            bump: Default::default(),
            _padding: Default::default(),
            ve_balances: [0; ERA_NUM_PERIODS],
        }
    }
}
