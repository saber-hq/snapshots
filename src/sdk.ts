import { newProgramMap } from "@saberhq/anchor-contrib";
import type { AugmentedProvider, Provider } from "@saberhq/solana-contrib";
import { SolanaAugmentedProvider } from "@saberhq/solana-contrib";
import type { Signer } from "@solana/web3.js";

import type { SnapshotsPrograms } from ".";
import { SNAPSHOTS_ADDRESSES, SNAPSHOTS_IDLS } from "./constants";
import { SnapshotsWrapper } from "./wrappers";

/**
 * Snapshots SDK.
 */
export class SnapshotsSDK {
  constructor(
    readonly provider: AugmentedProvider,
    readonly programs: SnapshotsPrograms
  ) {}

  /**
   * Creates a new instance of the SDK with the given keypair.
   */
  withSigner(signer: Signer): SnapshotsSDK {
    return SnapshotsSDK.load({
      provider: this.provider.withSigner(signer),
    });
  }

  /**
   * Loads the SDK.
   * @returns
   */
  static load({ provider }: { provider: Provider }): SnapshotsSDK {
    const programs: SnapshotsPrograms = newProgramMap<SnapshotsPrograms>(
      provider,
      SNAPSHOTS_IDLS,
      SNAPSHOTS_ADDRESSES
    );
    return new SnapshotsSDK(new SolanaAugmentedProvider(provider), programs);
  }

  /**
   * Snapshots program helpers.
   */
  get snapshots(): SnapshotsWrapper {
    return new SnapshotsWrapper(this);
  }
}
