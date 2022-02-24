import type { TransactionEnvelope } from "@saberhq/solana-contrib";
import type { PublicKey } from "@solana/web3.js";
import { SystemProgram } from "@solana/web3.js";
import { findEscrowAddress } from "@tribecahq/tribeca-sdk";

import type {
  EscrowHistoryData,
  LockerHistoryData,
  SnapshotsProgram,
} from "../../programs/snapshots";
import type { SnapshotsSDK } from "../../sdk";
import { findEscrowHistoryAddress, findLockerHistoryAddress } from ".";

/**
 * Handles interacting with the Snapshots program.
 */
export class SnapshotsWrapper {
  readonly program: SnapshotsProgram;

  /**
   * Constructor for a {@link SnapshotsWrapper}.
   * @param sdk
   */
  constructor(readonly sdk: SnapshotsSDK) {
    this.program = sdk.programs.Snapshots;
  }

  get provider() {
    return this.sdk.provider;
  }

  async fetchLockerHistory(key: PublicKey): Promise<LockerHistoryData | null> {
    return await this.program.account.lockerHistory.fetchNullable(key);
  }

  async fetchEscrowHistory(key: PublicKey): Promise<EscrowHistoryData | null> {
    return await this.program.account.escrowHistory.fetchNullable(key);
  }

  /**
   * Creates a Locker History.
   * @returns
   */
  async createLockerHistory({
    locker,
    era,
  }: {
    locker: PublicKey;
    era: number;
  }): Promise<{ lockerHistory: PublicKey; tx: TransactionEnvelope }> {
    const [lockerHistory] = await findLockerHistoryAddress(locker, era);
    return {
      lockerHistory,
      tx: this.provider.newTX([
        this.program.instruction.createLockerHistory(era, {
          accounts: {
            locker,
            lockerHistory,
            payer: this.provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
          },
        }),
      ]),
    };
  }

  /**
   * Creates an Escrow History.
   * @returns
   */
  async createEscrowHistory({
    escrow,
    era,
  }: {
    escrow: PublicKey;
    era: number;
  }): Promise<{ escrowHistory: PublicKey; tx: TransactionEnvelope }> {
    const [escrowHistory] = await findEscrowHistoryAddress(escrow, era);
    return {
      escrowHistory,
      tx: this.provider.newTX([
        this.program.instruction.createEscrowHistory(era, {
          accounts: {
            escrow,
            escrowHistory,
            payer: this.provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
          },
        }),
      ]),
    };
  }

  /**
   * Synchronizes an EscrowHistory.
   * @returns
   */
  async sync({
    locker,
    owner,
    era,
  }: {
    locker: PublicKey;
    owner: PublicKey;
    era: number;
  }): Promise<TransactionEnvelope> {
    const [lockerHistory] = await findLockerHistoryAddress(locker, era);
    const [escrow] = await findEscrowAddress(locker, owner);
    const [escrowHistory] = await findEscrowHistoryAddress(escrow, era);
    return this.provider.newTX([
      this.program.instruction.sync({
        accounts: {
          locker,
          escrow,
          lockerHistory,
          escrowHistory,
        },
      }),
    ]);
  }
}
