//! Calculations for voting escrow snapshots.
//!
//! These functions are split into a separate crate to ensure `anchor-lang` version mismatches
//! do not prevent building against this code.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![deny(clippy::unwrap_used, clippy::integer_arithmetic)]
#![deny(missing_docs)]

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

/// A period is `elapsed` if its start time has passed.
///
/// Elapsed periods cannot increase their locker veToken balance.
pub fn has_period_elapsed(era: u16, period: u8, now: i64) -> Option<bool> {
    let start = calculate_period_start_ts(era, period)?;
    let now = now.to_u64()?;
    // `>` instead of `>=` to prevent potential off-by-one errors
    // by programmers that are not aware of the definition of elapsed.
    // one second isn't a big deal.
    Some(now > start)
}

/// Calculates the era and period of the given Unix timestamp.
pub fn calculate_era_and_period_of_ts(now: u64) -> Option<(u16, u8)> {
    let current_era: u16 = now
        .checked_sub(COMMON_ERA_UNIX_TS)?
        .checked_div(SECONDS_PER_ERA)?
        .to_u16()?;
    let current_era_start_ts = calculate_era_start_ts(current_era)?;
    let current_period: u8 = now
        .checked_sub(current_era_start_ts)?
        .checked_div(PERIOD_SECONDS.into())?
        .to_u8()?;
    Some((current_era, current_period))
}

/// Calculates the next era and period of the given period.
pub fn calculate_next_era_and_period(era: u16, period: u8) -> Option<(u16, u8)> {
    Some(if period == u8::MAX {
        (era.checked_add(1)?, 0_u8)
    } else {
        (era, period.checked_add(1)?)
    })
}

/// Calculates the next era and period of the given Unix timestamp.
pub fn calculate_next_era_and_period_of_ts(now: u64) -> Option<(u16, u8)> {
    let (current_era, current_period) = calculate_era_and_period_of_ts(now)?;
    calculate_next_era_and_period(current_era, current_period)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::integer_arithmetic)]
mod tests {
    use super::*;

    #[test]
    fn test_has_period_elapsed() {
        // beginning of period 2: so period 2 has not elapsed yet.
        let current_time = (COMMON_ERA_UNIX_TS + (PERIOD_SECONDS as u64) * 2)
            .to_i64()
            .unwrap();

        assert!(has_period_elapsed(0, 0, current_time).unwrap());
        assert!(has_period_elapsed(0, 1, current_time).unwrap());
        assert!(!has_period_elapsed(0, 2, current_time).unwrap());

        assert!(!has_period_elapsed(1, 0, current_time).unwrap());
    }

    #[test]
    fn test_has_period_elapsed_boundary() {
        // beginning of period 2: so period 2 has not elapsed yet.
        let current_time = (COMMON_ERA_UNIX_TS + (PERIOD_SECONDS as u64) * 2 + 1)
            .to_i64()
            .unwrap();

        assert!(has_period_elapsed(0, 0, current_time).unwrap());
        assert!(has_period_elapsed(0, 1, current_time).unwrap());
        assert!(has_period_elapsed(0, 2, current_time).unwrap());
        assert!(!has_period_elapsed(0, 3, current_time).unwrap());

        assert!(!has_period_elapsed(1, 0, current_time).unwrap());
    }

    #[test]
    fn test_calculate_next_era_and_period_normal() {
        let era = 2_u16;
        let period = 4_u8;
        let start = calculate_period_start_ts(era, period).unwrap() + 40;

        let (result_era, result_period) = calculate_era_and_period_of_ts(start).unwrap();
        assert_eq!(result_era, era);
        assert_eq!(result_period, period);

        let (result_next_era, result_next_period) =
            calculate_next_era_and_period_of_ts(start).unwrap();
        assert_eq!(result_next_era, era);
        assert_eq!(result_next_period, period + 1);
    }

    #[test]
    fn test_calculate_next_era_and_period_boundary() {
        let era = 2_u16;
        let period = 255_u8;
        let start = calculate_period_start_ts(era, period).unwrap() + 40;

        let (result_era, result_period) = calculate_era_and_period_of_ts(start).unwrap();
        assert_eq!(result_era, era);
        assert_eq!(result_period, period);

        let (result_next_era, result_next_period) =
            calculate_next_era_and_period_of_ts(start).unwrap();
        assert_eq!(result_next_era, era + 1);
        assert_eq!(result_next_period, 0_u8);
    }
}
