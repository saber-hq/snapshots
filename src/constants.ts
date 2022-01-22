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
 * Default epoch duration (seconds) -- 7 days
 */
export const DEFAULT_EPOCH_DURATION_SECONDS = 60 * 60 * 24 * 7;

export const ERA_NUM_PERIODS = 256;
export const PERIOD_SECONDS = 86_400 * 3;
export const COMMON_ERA_UNIX_TS = 1640995200;
