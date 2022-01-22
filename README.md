# snapshots

[![Crates.io](https://img.shields.io/crates/v/snapshots)](https://crates.io/crates/snapshots)
[![Docs.rs](https://img.shields.io/docsrs/snapshots)](https://docs.rs/snapshots)
[![License](https://img.shields.io/crates/l/snapshots)](https://github.com/saber-hq/snapshots/blob/master/LICENSE)
[![Build Status](https://img.shields.io/github/workflow/status/saber-hq/snapshots/E2E/master)](https://github.com/saber-hq/snapshots/actions/workflows/programs-e2e.yml?query=branch%3Amaster)
[![Contributors](https://img.shields.io/github/contributors/saber-hq/snapshots)](https://github.com/saber-hq/snapshots/graphs/contributors)
[![NPM](https://img.shields.io/npm/v/@saberhq/snapshots)](https://www.npmjs.com/package/@saberhq/snapshots)

<p align="center">
    <img src="https://raw.githubusercontent.com/saber-hq/snapshots/master/images/banner.png" />
</p>

Voting Escrow Snapshots: Historical snapshots of previous voting escrow balances.

## Motivation

There are several instances in which one may want to use an instantaneous snapshot of all vote escrow balances, for example:

- **Fee distribution.** One may want to send protocol revenue to veToken holders.
- **Airdrops.** One may want to send tokens to holders of a veToken.

## Mechanism

veToken balances are recorded for every `period`. A period is recorded for every 3 days.

There are two accounts that are used to compute historical balances:

- [LockerHistory], which stores the total number of veTokens for each period, and
- [EscrowHistory], which stores the veTokens in each Escrow per period.

Any time someone refreshes and/or modifies their vote escrow, they should refresh their [EscrowHistory] accounts.

## License

The [snapshots] program is licensed under the Affero General Public License version 3.
