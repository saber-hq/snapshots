//! Processor for [snapshots::sync].

use crate::*;
use ::u128::mul_div_u64;
use locked_voter::{Escrow, Locker, LockerParams};
use num_traits::ToPrimitive;

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
    fn sync(&self) -> Result<()> {
        let locker_history = &mut self.locker_history.load_mut()?;
        let escrow_history = &mut self.escrow_history.load_mut()?;

        assert_keys_eq!(locker_history.locker, self.locker);
        assert_keys_eq!(escrow_history.escrow, self.escrow);
        invariant!(locker_history.era == escrow_history.era, EraMismatch);

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
            let ve_balance: u64 = unwrap_int!(calculate_voter_power_v2(
                self.locker.params,
                &self.escrow,
                period_start_ts
            ));

            locker_history.ve_balances[period] = unwrap_checked!({
                locker_history.ve_balances[period]
                    .checked_sub(prev_period_ve_balance)?
                    .checked_add(ve_balance)
            });
            escrow_history.ve_balances[period] = ve_balance;

            invariant!(ve_balance >= prev_period_ve_balance, EscrowBalanceDecreased);

            // If the previous balance was zero, this is a newly tracked escrow.
            // This voter should be recorded in the counts.
            if prev_period_ve_balance == 0 && ve_balance != 0 {
                locker_history.ve_counts[period] =
                    unwrap_int!(locker_history.ve_counts[period].checked_add(1));
            }
        }

        Ok(())
    }
}

fn calculate_voter_power_v2(
    LockerParams {
        max_stake_duration,
        max_stake_vote_multiplier,
        ..
    }: LockerParams,
    escrow: &Escrow,
    now: i64,
) -> Option<u64> {
    // invalid `now` argument, should never happen.
    if now == 0 {
        return None;
    }
    if escrow.escrow_started_at == 0 {
        return Some(0);
    }
    // Lockup had zero power before the start time.
    // at the end time, lockup also has zero power.
    if now < escrow.escrow_started_at || now >= escrow.escrow_ends_at {
        return Some(0);
    }

    let seconds_until_lockup_expiry = escrow.escrow_ends_at.checked_sub(now)?;
    // elapsed seconds, clamped to the maximum duration
    let relevant_seconds_until_lockup_expiry = seconds_until_lockup_expiry
        .to_u64()?
        .min(max_stake_duration);

    // voting power at max lockup
    let power_if_max_lockup = escrow
        .amount
        .checked_mul((max_stake_vote_multiplier).into())?;

    // multiply the max lockup power by the fraction of the max stake duration
    let power = mul_div_u64(
        power_if_max_lockup,
        relevant_seconds_until_lockup_expiry,
        max_stake_duration,
    )?;

    Some(power)
}

pub fn handler(ctx: Context<Sync>) -> Result<()> {
    ctx.accounts.sync()
}

impl<'info> Validate<'info> for Sync<'info> {
    fn validate(&self) -> Result<()> {
        assert_keys_eq!(self.locker, self.escrow.locker);
        Ok(())
    }
}
