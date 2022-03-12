//! Processor for [snapshots::sync].

use crate::*;
use ::u128::mul_div_u64;
use locked_voter::{Escrow, Locker};
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
        let now = unwrap_int!(Clock::get()?.unix_timestamp.to_u64());

        // The voting power at max lockup.
        // This is used as a multiplicand to determine the total voting power
        // at a given time.
        let power_if_max_lockup = unwrap_int!(self
            .escrow
            .amount
            .checked_mul(self.locker.params.max_stake_vote_multiplier.into()));
        let escrow_started_at = unwrap_int!(self.escrow.escrow_started_at.to_u64());
        let escrow_ends_at = unwrap_int!(self.escrow.escrow_ends_at.to_u64());

        // If the escrow never started, we should not be updating anything.
        if escrow_started_at == 0 {
            return Ok(());
        }

        // calculate every period
        let mut period_start_ts = start_ts;
        for period in 0..ERA_NUM_PERIODS {
            if period > 0 {
                // add the period each iteration
                period_start_ts = unwrap_int!(period_start_ts.checked_add(PERIOD_SECONDS.into()));
            }

            // skip over periods that have already passed.
            if now >= period_start_ts {
                continue;
            }

            // The previous value of this period's ve balance.
            // !WARNING!: not to be confused with the veBalance of the previous period.
            let prev_period_ve_balance = escrow_history.ve_balances[period];

            // The current value of this period's ve balance.
            let ve_balance: u64 = unwrap_int!(calculate_voter_power_simple(
                power_if_max_lockup,
                period_start_ts,
                escrow_started_at,
                escrow_ends_at,
                self.locker.params.max_stake_duration,
            ));

            // skip zero ve balance
            if ve_balance == 0 {
                // prev ve balance should have been zero
                invariant!(prev_period_ve_balance == 0);
                continue;
            }

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

/// Calculates voter power using cached calculations.
///
/// - `power_if_max_lockup`: Voting power if the lockup was at the maximum amount
fn calculate_voter_power_simple(
    power_if_max_lockup: u64,
    period_start_ts: u64,
    escrow_started_at: u64,
    escrow_ends_at: u64,
    max_stake_duration: u64,
) -> Option<u64> {
    // invalid `now` argument, should never happen.
    if period_start_ts == 0 {
        return None;
    }
    if escrow_started_at == 0 {
        return Some(0);
    }
    // Lockup had zero power before the start time.
    // at the end time, lockup also has zero power.
    if period_start_ts < escrow_started_at || period_start_ts >= escrow_ends_at {
        return Some(0);
    }

    // multiply the max lockup power by the fraction of the max stake duration
    let seconds_until_lockup_expiry = escrow_ends_at.checked_sub(period_start_ts)?.to_u64()?;
    // elapsed seconds, clamped to the maximum duration
    let relevant_seconds_until_lockup_expiry = seconds_until_lockup_expiry.min(max_stake_duration);

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
