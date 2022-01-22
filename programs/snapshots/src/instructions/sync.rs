//! Processor for [snapshots::sync].

use anchor_lang::prelude::*;
use locked_voter::{Escrow, Locker};
use num_traits::ToPrimitive;
use vipers::{assert_keys_eq, unwrap_int};

use crate::*;

/// Accounts for [snapshots::sync].
#[derive(Accounts)]
pub struct Sync<'info> {
    /// The [Locker].
    pub locker: Account<'info, Locker>,

    /// The [Escrow].
    pub escrow: Account<'info, Escrow>,

    /// The [LockerHistory] to sync.
    #[account(mut)]
    pub locker_history: AccountLoader<'info, LockerHistory>,

    /// The [EscrowHistory] to sync.
    #[account(mut)]
    pub escrow_history: AccountLoader<'info, EscrowHistory>,
}

impl<'info> Sync<'info> {
    fn sync(&self) -> ProgramResult {
        let locker_history = &mut self.locker_history.load_mut()?;
        let escrow_history = &mut self.escrow_history.load_mut()?;

        let start_ts = unwrap_int!(calculate_era_start_ts(locker_history.era));
        let now = Clock::get()?.unix_timestamp;

        // calculate every period
        let mut period_start_ts = unwrap_int!(start_ts.to_i64());
        for period in 0..ERA_NUM_PERIODS {
            if period > 0 {
                // add the period each iteration
                period_start_ts =
                    unwrap_int!(period_start_ts.checked_add(unwrap_int!(PERIOD_SECONDS.to_i64())));
            }

            // skip over periods that have already passed.
            if now >= period_start_ts {
                continue;
            }

            let prev_period_ve_balance = escrow_history.ve_balances[period];
            let ve_balance: u64 = unwrap_int!(self
                .locker
                .params
                .calculate_voter_power(&self.escrow, period_start_ts));

            locker_history.ve_balances[period] = unwrap_int!(locker_history.ve_balances[period]
                .checked_sub(prev_period_ve_balance)
                .and_then(|v| v.checked_add(ve_balance)));
            escrow_history.ve_balances[period] = ve_balance;

            // If the previous balance was zero, this is a newly tracked.
            // This voter should be recorded in the counts.
            if prev_period_ve_balance == 0 && ve_balance != 0 {
                locker_history.ve_counts[period] =
                    unwrap_int!(locker_history.ve_counts[period].checked_add(1));
            } else if prev_period_ve_balance != 0 && ve_balance == 0 {
                // If the previous balance was non-zero but the new balance is zero,
                // locker parameters have changed.
                locker_history.ve_counts[period] =
                    unwrap_int!(locker_history.ve_counts[period].checked_sub(1));
            }
        }

        Ok(())
    }
}

pub fn handler(ctx: Context<Sync>) -> ProgramResult {
    ctx.accounts.sync()
}

impl<'info> Validate<'info> for Sync<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys_eq!(self.locker, self.escrow.locker);
        Ok(())
    }
}
