//! Voting Escrow Snapshots: Historical snapshots of previous voting escrow balances.
//!
//! # Motivation
//!
//! There are several instances in which one may want to use an instantaneous snapshot of all vote escrow balances, for example:
//!
//! - **Fee distribution.** One may want to send protocol revenue to veToken holders.
//! - **Airdrops.** One may want to send tokens to holders of a veToken.
//!
//! # Mechanism
//!
//! veToken balances are recorded for every `period`. A period is recorded for every 3 days.
//!
//! There are two accounts that are used to compute historical balances:
//!
//! - [LockerHistory], which stores the total number of veTokens for each period, and
//! - [EscrowHistory], which stores the veTokens in each Escrow per period.
//!
//! Any time someone refreshes and/or modifies their vote escrow, they should refresh their [EscrowHistory] accounts.
//!
//! # Program Addresses
//!
//! - **[snapshots]:** [StakeSSzfxn391k3LvdKbZP5WVwWd6AsY1DNiXHjQfK](https://anchor.so/programs/StakeSSzfxn391k3LvdKbZP5WVwWd6AsY1DNiXHjQfK)
//!
//! # License
//!
//! The [snapshots] program is licensed under the Affero General Public License version 3.

#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![deny(clippy::unwrap_used)]

use anchor_lang::prelude::*;
use vipers::prelude::*;

mod instructions;
mod state;

pub use snapshots_math::*;
pub use state::*;

use instructions::*;

declare_id!("StakeSSzfxn391k3LvdKbZP5WVwWd6AsY1DNiXHjQfK");

/// The [snapshots] program.
#[program]
pub mod snapshots {
    use super::*;

    /// Creates a [EscrowHistory].
    #[access_control(ctx.accounts.validate())]
    pub fn create_escrow_history(ctx: Context<CreateEscrowHistory>, era: u16) -> Result<()> {
        create_escrow_history::handler(ctx, era)
    }

    /// Creates a [LockerHistory].
    #[access_control(ctx.accounts.validate())]
    pub fn create_locker_history(ctx: Context<CreateLockerHistory>, era: u16) -> Result<()> {
        create_locker_history::handler(ctx, era)
    }

    /// Synchronize an [locked_voter::Escrow] with the [LockerHistory]/[EscrowHistory].
    #[access_control(ctx.accounts.validate())]
    pub fn sync(ctx: Context<Sync>) -> Result<()> {
        sync::handler(ctx)
    }

    /// Synchronize an [locked_voter::Escrow] with the [LockerHistory]/[EscrowHistory].
    #[access_control(ctx.accounts.validate())]
    pub fn sync_range(ctx: Context<Sync>) -> Result<()> {
        sync::handler(ctx)
    }
}

/// Errors.
#[error_code]
pub enum ErrorCode {
    #[msg("Locker/escrow mismatch.")]
    LockerEscrowMismatch,
    #[msg("Era mismatch.")]
    EraMismatch,
    #[msg("Escrow balances cannot decrease.")]
    EscrowBalanceDecreased,
}
