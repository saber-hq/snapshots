import { ACCOUNT_DISCRIMINATOR_SIZE } from "@project-serum/anchor/dist/cjs/coder";
import { PublicKeyLayout, u64 } from "@saberhq/token-utils";
import * as BufferLayout from "@solana/buffer-layout";
import { PublicKey } from "@solana/web3.js";

import { ERA_NUM_PERIODS } from "../..";
import type {
  EscrowHistoryData,
  LockerHistoryData,
} from "../../programs/snapshots";

// A tip for figuring alignment out: compile with cargo rustc -- -Zprint-type-sizes and then inspect the types for any generated padding.

const EscrowHistoryLayout = BufferLayout.struct<{
  escrow: Uint8Array;
  era: number;
  _padding: number[];
  bump: number;
  veBalances: Uint8Array[];
}>([
  PublicKeyLayout("escrow"),
  BufferLayout.u16("era"),
  BufferLayout.u8("bump"),
  BufferLayout.seq(BufferLayout.u8(), 5),
  BufferLayout.seq(BufferLayout.blob(8), ERA_NUM_PERIODS, "veBalances"),
]);

const LockerHistoryLayout = BufferLayout.struct<{
  locker: Uint8Array;
  era: number;
  bump: number;
  _padding: number[];
  veBalances: Uint8Array[];
  veCounts: Uint8Array[];
}>([
  PublicKeyLayout("locker"),
  BufferLayout.u16("era"),
  BufferLayout.u8("bump"),
  BufferLayout.seq(BufferLayout.u8(), 5),
  BufferLayout.seq(BufferLayout.blob(8), ERA_NUM_PERIODS, "veBalances"),
  BufferLayout.seq(BufferLayout.blob(8), ERA_NUM_PERIODS, "veCounts"),
]);

/**
 * Decodes a {@link LockerHistoryData}.
 * @param data
 * @returns
 */
export const decodeLockerHistory = (data: Buffer): LockerHistoryData => {
  const { locker, era, bump, veBalances, veCounts } =
    LockerHistoryLayout.decode(data.slice(ACCOUNT_DISCRIMINATOR_SIZE));
  return {
    locker: new PublicKey(locker),
    era,
    bump,
    veBalances: veBalances.map((veb) => new u64(veb, "le")),
    veCounts: veCounts.map((vec) => new u64(vec, "le")),
  };
};

/**
 * Decodes a {@link EscrowHistoryData}.
 * @param data
 * @returns
 */
export const decodeEscrowHistory = (data: Buffer): EscrowHistoryData => {
  const { escrow, era, bump, veBalances } = EscrowHistoryLayout.decode(
    data.slice(ACCOUNT_DISCRIMINATOR_SIZE)
  );
  return {
    escrow: new PublicKey(escrow),
    era,
    bump,
    veBalances: veBalances.map((veb) => new u64(veb, "le")),
  };
};
