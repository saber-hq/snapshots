import invariant from "tiny-invariant";

import {
  COMMON_ERA_UNIX_TS,
  ERA_NUM_PERIODS,
  PERIOD_SECONDS,
} from "./constants";

/**
 * Number of seconds in an era.
 */
export const SECONDS_PER_ERA = PERIOD_SECONDS * ERA_NUM_PERIODS;

/**
 * Calculates the era that the given {@link Date} is in.
 * @param date
 * @returns
 */
export const calculateEra = (date: Date): number => {
  return Math.floor(
    (Math.floor(date.getTime() / 1_000) - COMMON_ERA_UNIX_TS) / SECONDS_PER_ERA
  );
};

/**
 * Calculates the start date of a period.
 * @param era
 * @param period
 * @returns
 */
export const calculatePeriodStart = (era: number, period: number): Date => {
  return new Date(
    (COMMON_ERA_UNIX_TS + era * SECONDS_PER_ERA + period * PERIOD_SECONDS) *
      1_000
  );
};

/**
 * Calculates the start date of an era.
 * @param era
 * @returns
 */
export const calculateEraStart = (era: number): Date =>
  calculatePeriodStart(era, 0);

/**
 * Returns the eras included in a given period.
 *
 * This is useful for figuring out what histories must be fetched.
 *
 * @param start
 * @param end
 * @returns
 */
export const calculateErasForPeriod = (
  start: Date,
  end: Date
): readonly number[] => {
  const currentEra = calculateEra(start);
  const lastEra = calculateEra(end);
  invariant(
    lastEra >= currentEra,
    "Last era must be greater than current era."
  );
  return Array(lastEra - currentEra + 1)
    .fill(null)
    .map((_, i) => currentEra + i);
};
