import { GokiSDK } from "@gokiprotocol/client";
import { expectTX, expectTXTable } from "@saberhq/chai-solana";
import { createMint, sleep, u64 } from "@saberhq/token-utils";
import type { PublicKey, Signer } from "@solana/web3.js";
import type { LockerWrapper } from "@tribecahq/tribeca-sdk";
import {
  createLocker,
  DEFAULT_LOCKER_PARAMS,
  findEscrowAddress,
  TribecaSDK,
} from "@tribecahq/tribeca-sdk";
import { expect } from "chai";
import { zip } from "lodash";
import invariant from "tiny-invariant";

import {
  findEscrowHistoryAddress,
  findLockerHistoryAddress,
} from "../src/wrappers/snapshots/pda";
import { createUser, makeSDK } from "./workspace";

export const INITIAL_MINT_AMOUNT = new u64(1_000_000_000);

describe("Locked Voter", () => {
  const sdk = makeSDK();
  const gokiSDK = GokiSDK.load({ provider: sdk.provider });
  const tribecaSDK = TribecaSDK.load({ provider: sdk.provider });

  let govTokenMint: PublicKey;

  let lockerW: LockerWrapper;
  let user: Signer;

  beforeEach(async () => {
    govTokenMint = await createMint(sdk.provider);

    const owners = [sdk.provider.wallet.publicKey];

    const { createTXs, lockerWrapper } = await createLocker({
      sdk: tribecaSDK,
      gokiSDK,
      govTokenMint,
      owners,
      lockerParams: {
        proposalActivationMinVotes: INITIAL_MINT_AMOUNT,
      },
    });

    for (const { tx: createTX } of createTXs) {
      await expectTX(createTX).to.be.fulfilled;
    }

    lockerW = lockerWrapper;

    const { tx: createLockerHistoryTX } =
      await sdk.snapshots.createLockerHistory({
        locker: lockerW.locker,
        era: 0,
      });
    await expectTX(createLockerHistoryTX).to.be.fulfilled;
  });

  beforeEach("Create user and deposit tokens", async () => {
    user = await createUser(sdk.provider, govTokenMint);
    const lockTx = await lockerW.lockTokens({
      amount: INITIAL_MINT_AMOUNT,
      duration: DEFAULT_LOCKER_PARAMS.maxStakeDuration,
      authority: user.publicKey,
    });
    lockTx.addSigners(user);
    await expectTX(lockTx, "lock tokens").to.be.fulfilled;

    const [escrowKey] = await findEscrowAddress(lockerW.locker, user.publicKey);

    await Promise.all(
      Array(6)
        .fill(null)
        .map(async (_, era) => {
          const { tx: createEscrowHistoryTX } =
            await sdk.snapshots.createEscrowHistory({
              escrow: escrowKey,
              era,
            });
          await expectTX(createEscrowHistoryTX, "create escrow history").to.be
            .fulfilled;
        })
    );
  });

  it("syncs single escrow", async () => {
    const [escrowKey] = await findEscrowAddress(lockerW.locker, user.publicKey);

    const [lockerHistory] = await findLockerHistoryAddress(lockerW.locker, 0);
    const [escrowHistory] = await findEscrowHistoryAddress(escrowKey, 0);

    const syncTX = await sdk.snapshots.sync({
      locker: lockerW.locker,
      owner: user.publicKey,
      era: 0,
    });
    await expectTXTable(syncTX, "snapshots").to.be.fulfilled;

    const lockerHistoryData = await sdk.snapshots.fetchLockerHistory(
      lockerHistory
    );
    const escrowHistoryData = await sdk.snapshots.fetchEscrowHistory(
      escrowHistory
    );

    invariant(lockerHistoryData && escrowHistoryData);

    expect(lockerHistoryData.veBalances).to.deep.eq(
      escrowHistoryData.veBalances
    );
  });

  it("sync multiple times should have no effect", async () => {
    const syncTX1 = await sdk.snapshots.sync({
      locker: lockerW.locker,
      owner: user.publicKey,
      era: 0,
    });
    await expectTXTable(syncTX1, "sync").to.be.fulfilled;

    const [escrowKey] = await findEscrowAddress(lockerW.locker, user.publicKey);

    const [lockerHistory] = await findLockerHistoryAddress(lockerW.locker, 0);
    const [escrowHistory] = await findEscrowHistoryAddress(escrowKey, 0);

    const lockerHistoryData1 = await sdk.snapshots.fetchLockerHistory(
      lockerHistory
    );
    const escrowHistoryData1 = await sdk.snapshots.fetchEscrowHistory(
      escrowHistory
    );

    const syncTX2 = await sdk.snapshots.sync({
      locker: lockerW.locker,
      owner: user.publicKey,
      era: 0,
    });
    await expectTXTable(syncTX2, "sync again").to.be.fulfilled;

    const lockerHistoryData2 = await sdk.snapshots.fetchLockerHistory(
      lockerHistory
    );
    const escrowHistoryData2 = await sdk.snapshots.fetchEscrowHistory(
      escrowHistory
    );

    invariant(
      lockerHistoryData1 &&
        escrowHistoryData1 &&
        lockerHistoryData2 &&
        escrowHistoryData2
    );

    expect(lockerHistoryData1.veBalances).to.deep.eq(
      lockerHistoryData2.veBalances
    );
    expect(escrowHistoryData1.veBalances).to.deep.eq(
      escrowHistoryData2.veBalances
    );
  });

  it("syncs multiple escrows", async () => {
    const [lockerHistory] = await findLockerHistoryAddress(lockerW.locker, 0);
    const initialLockerHistoryData = await sdk.snapshots.fetchLockerHistory(
      lockerHistory
    );

    invariant(initialLockerHistoryData);
    expect(initialLockerHistoryData.veBalances.length).to.eq(256);

    Array(256)
      .fill(null)
      .map((_, i) => {
        expect(initialLockerHistoryData.veBalances[i]).to.bignumber.zero;
        expect(initialLockerHistoryData.veCounts[i]).to.bignumber.zero;
      });

    const user2 = await createUser(sdk.provider, govTokenMint);
    const lockTx = await lockerW.lockTokens({
      amount: INITIAL_MINT_AMOUNT,
      duration: DEFAULT_LOCKER_PARAMS.maxStakeDuration,
      authority: user2.publicKey,
    });
    lockTx.addSigners(user2);
    await expectTX(lockTx, "lock tokens").to.be.fulfilled;

    const [escrow2Key] = await findEscrowAddress(
      lockerW.locker,
      user2.publicKey
    );
    await Promise.all(
      Array(6)
        .fill(null)
        .map(async (_, era) => {
          const { tx: createEscrowHistoryTX } = await sdk
            .withSigner(user2)
            .snapshots.createEscrowHistory({
              escrow: escrow2Key,
              era,
            });
          await expectTX(createEscrowHistoryTX, "create escrow history").to.be
            .fulfilled;
        })
    );

    const [escrow2History] = await findEscrowHistoryAddress(escrow2Key, 0);

    const syncTX = await sdk.snapshots.sync({
      locker: lockerW.locker,
      owner: user.publicKey,
      era: 0,
    });
    await expectTXTable(syncTX, "snapshots 1").to.be.fulfilled;

    const sync2TX = await sdk.withSigner(user2).snapshots.sync({
      locker: lockerW.locker,
      owner: user2.publicKey,
      era: 0,
    });
    await expectTXTable(sync2TX, "snapshots 2", {
      verbosity: "always",
    }).to.be.fulfilled;

    const lockerHistoryData = await sdk.snapshots.fetchLockerHistory(
      lockerHistory
    );

    const [escrow1Key] = await findEscrowAddress(
      lockerW.locker,
      user.publicKey
    );
    const [escrow1History] = await findEscrowHistoryAddress(escrow1Key, 0);
    const escrow1HistoryData = await sdk.snapshots.fetchEscrowHistory(
      escrow1History
    );
    const escrow2HistoryData = await sdk.snapshots.fetchEscrowHistory(
      escrow2History
    );

    invariant(lockerHistoryData && escrow1HistoryData && escrow2HistoryData);

    zip(escrow1HistoryData.veBalances, escrow2HistoryData.veBalances).map(
      ([e1, e2], i) => {
        invariant(e1 && e2);
        const expected = e1.add(e2);
        expect(lockerHistoryData.veBalances[i], `period ${i}`).to.bignumber.eq(
          expected
        );
        if (!e1.isZero() && !e2.isZero()) {
          expect(lockerHistoryData.veCounts[i], `count ${i}`).to.bignumber.eq(
            "2"
          );
        }
      }
    );
  });

  it("changes with a refresh", async () => {
    const [escrowKey] = await findEscrowAddress(lockerW.locker, user.publicKey);

    const [lockerHistory] = await findLockerHistoryAddress(lockerW.locker, 0);
    const [escrowHistory] = await findEscrowHistoryAddress(escrowKey, 0);

    const syncTX = await sdk.snapshots.sync({
      locker: lockerW.locker,
      owner: user.publicKey,
      era: 0,
    });
    await expectTXTable(syncTX, "snapshots").to.be.fulfilled;

    const lockerHistoryData = await sdk.snapshots.fetchLockerHistory(
      lockerHistory
    );
    const escrowHistoryData = await sdk.snapshots.fetchEscrowHistory(
      escrowHistory
    );
    invariant(lockerHistoryData && escrowHistoryData);
    expect(lockerHistoryData.veBalances).to.deep.eq(
      escrowHistoryData.veBalances
    );

    // sleep so we can get more lock
    await sleep(1_000);

    const lockTx = await lockerW.lockTokens({
      amount: new u64(0),
      duration: DEFAULT_LOCKER_PARAMS.maxStakeDuration,
      authority: user.publicKey,
    });
    lockTx.addSigners(user);
    await expectTX(lockTx, "lock tokens").to.be.fulfilled;

    const sync2TX = await sdk.snapshots.sync({
      locker: lockerW.locker,
      owner: user.publicKey,
      era: 0,
    });
    await expectTXTable(sync2TX, "snapshots sync v2").to.be.fulfilled;

    const lockerHistoryData2 = await sdk.snapshots.fetchLockerHistory(
      lockerHistory
    );
    const escrowHistoryData2 = await sdk.snapshots.fetchEscrowHistory(
      escrowHistory
    );
    invariant(lockerHistoryData2 && escrowHistoryData2);
    expect(lockerHistoryData2.veBalances).to.deep.eq(
      escrowHistoryData2.veBalances
    );

    // should have changed
    expect(lockerHistoryData2.veBalances).to.not.deep.eq(
      lockerHistoryData.veBalances
    );
    expect(escrowHistoryData2.veBalances).to.not.deep.eq(
      escrowHistoryData.veBalances
    );
  });
});
