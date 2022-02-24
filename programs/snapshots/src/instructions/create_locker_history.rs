//! Processor for [snapshots::create_locker_history].

use crate::*;
use locked_voter::Locker;

/// Accounts for [snapshots::create_locker_history].
#[derive(Accounts)]
#[instruction(era: u16)]
pub struct CreateLockerHistory<'info> {
    /// The [Locker].
    pub locker: Account<'info, Locker>,

    /// The [LockerHistory] to be created.
    #[account(
        init,
        seeds = [
            b"LockerHistory".as_ref(),
            locker.key().as_ref(),
            era.to_le_bytes().as_ref()
        ],
        bump,
        payer = payer
    )]
    pub locker_history: AccountLoader<'info, LockerHistory>,

    /// Payer.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// [System] program.
    pub system_program: Program<'info, System>,
}

impl<'info> CreateLockerHistory<'info> {
    fn create_locker_history(&mut self, bump: u8, era: u16) -> Result<()> {
        let history = &mut self.locker_history.load_init()?;
        history.locker = self.locker.key();
        history.era = era;
        history.bump = bump;
        Ok(())
    }
}

pub fn handler(ctx: Context<CreateLockerHistory>, era: u16) -> Result<()> {
    ctx.accounts
        .create_locker_history(*unwrap_int!(ctx.bumps.get("locker_history")), era)
}

impl<'info> Validate<'info> for CreateLockerHistory<'info> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
