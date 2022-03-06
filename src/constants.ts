import { buildCoderMap } from "@saberhq/anchor-contrib";
import { PublicKey } from "@solana/web3.js";

import type { SnapshotsProgram, SnapshotsTypes } from "./programs";
import { SnapshotsJSON } from "./programs";

/**
 * Snapshots program types.
 */
export interface SnapshotsPrograms {
  Snapshots: SnapshotsProgram;
}

/**
 * Snapshots addresses.
 */
export const SNAPSHOTS_ADDRESSES = {
  Snapshots: new PublicKey("StakeSSzfxn391k3LvdKbZP5WVwWd6AsY1DNiXHjQfK"),
};

/**
 * Program IDLs.
 */
export const SNAPSHOTS_IDLS = {
  Snapshots: SnapshotsJSON,
};

/**
 * Coders.
 */
export const SNAPSHOTS_CODERS = buildCoderMap<{
  Snapshots: SnapshotsTypes;
}>(SNAPSHOTS_IDLS, SNAPSHOTS_ADDRESSES);

/**
 * Number of periods in an era.
 */
export const ERA_NUM_PERIODS = 256;

/**
 * Number of seconds in a period.
 */
export const PERIOD_SECONDS = 86_400 * 3;

/**
 * The Unix timestamp of the start of the first era.
 */
export const COMMON_ERA_UNIX_TS = 1640995200;
