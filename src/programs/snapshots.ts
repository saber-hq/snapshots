import type { AnchorTypes } from "@saberhq/anchor-contrib";

import type { SnapshotsIDL } from "../idls/snapshots";

export * from "../idls/snapshots";

export type SnapshotsTypes = AnchorTypes<
  SnapshotsIDL,
  {
    lockerHistory: LockerHistoryData;
    escrowHistory: EscrowHistoryData;
  }
>;

type Accounts = SnapshotsTypes["Accounts"];

export type LockerHistoryData = Accounts["LockerHistory"];
export type EscrowHistoryData = Accounts["EscrowHistory"];

export type SnapshotsProgram = SnapshotsTypes["Program"];
