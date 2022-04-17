import * as anchor from "@project-serum/anchor";
import { AnchorProvider } from "@project-serum/anchor";
import { makeSaberProvider } from "@saberhq/anchor-contrib";
import { chaiSolana, expectTX } from "@saberhq/chai-solana";
import type { Provider } from "@saberhq/solana-contrib";
import { TransactionEnvelope } from "@saberhq/solana-contrib";
import {
  getOrCreateATA,
  SPLToken,
  TOKEN_PROGRAM_ID,
  u64,
} from "@saberhq/token-utils";
import type { PublicKey, Signer } from "@solana/web3.js";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
} from "@solana/web3.js";
import chai from "chai";

import type { SnapshotsPrograms } from "../../src";
import { SnapshotsSDK } from "../../src";
import { INITIAL_MINT_AMOUNT } from "../snapshots.spec";

chai.use(chaiSolana);

export type Workspace = SnapshotsPrograms;

export const makeSDK = (): SnapshotsSDK => {
  const anchorProvider = AnchorProvider.env();
  anchor.setProvider(anchorProvider);
  const provider = makeSaberProvider(anchorProvider);
  return SnapshotsSDK.load({
    provider,
  });
};

export const DUMMY_INSTRUCTIONS = [
  Keypair.generate().publicKey,
  Keypair.generate().publicKey,
  Keypair.generate().publicKey,
].map(
  (pid) =>
    new TransactionInstruction({
      programId: pid,
      keys: [],
    })
);

export const createUser = async (
  provider: Provider,
  govTokenMint: PublicKey
): Promise<Signer> => {
  const user = Keypair.generate();

  await provider.connection.requestAirdrop(user.publicKey, LAMPORTS_PER_SOL);

  const { address, instruction } = await getOrCreateATA({
    provider,
    mint: govTokenMint,
    owner: user.publicKey,
  });
  const mintToIx = SPLToken.createMintToInstruction(
    TOKEN_PROGRAM_ID,
    govTokenMint,
    address,
    provider.wallet.publicKey,
    [],
    new u64(INITIAL_MINT_AMOUNT)
  );

  const tx = new TransactionEnvelope(
    provider,
    instruction ? [instruction, mintToIx] : [mintToIx]
  );
  await expectTX(tx, "mint gov tokens to user").to.be.fulfilled;

  return user;
};
