import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

import { SNAPSHOTS_ADDRESSES } from "../../constants";

const encodeU16 = (num: number): Buffer => {
  const buf = Buffer.alloc(2);
  buf.writeUInt16LE(num);
  return buf;
};

/**
 * Finds the address of an EscrowHistory.
 */
export const findEscrowHistoryAddress = async (
  escrow: PublicKey,
  era: number
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [
      utils.bytes.utf8.encode("EscrowHistory"),
      escrow.toBuffer(),
      encodeU16(era),
    ],
    SNAPSHOTS_ADDRESSES.Snapshots
  );
};

/**
 * Finds the address of a LockerHistory.
 */
export const findLockerHistoryAddress = async (
  locker: PublicKey,
  era: number
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [
      utils.bytes.utf8.encode("LockerHistory"),
      locker.toBuffer(),
      encodeU16(era),
    ],
    SNAPSHOTS_ADDRESSES.Snapshots
  );
};
