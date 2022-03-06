//! Struct definitions for accounts that hold state.

use crate::*;

/// Stores the total number of veTokens in circulation for each period.
///
/// The [LockerHistory] account stores 256 periods, each 3 days each.
/// For a 5-year [locked_voter::Locker], there will be at least 3 of these accounts existing
/// at any given time, since the maximum lock period is 5 years.
#[account(zero_copy)]
#[derive(Debug, PartialEq, Eq)]
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
#[derive(Debug, PartialEq, Eq)]
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
