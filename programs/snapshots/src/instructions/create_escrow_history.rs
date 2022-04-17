//! Processor for [snapshots::create_escrow_history].

use crate::*;
use locked_voter::Escrow;

/// Accounts for [snapshots::create_escrow_history].
#[derive(Accounts)]
#[instruction(era: u16)]
pub struct CreateEscrowHistory<'info> {
    /// The [Escrow].
    pub escrow: Account<'info, Escrow>,

    /// The [EscrowHistory] to be created.
    #[account(
        init,
        seeds = [
            b"EscrowHistory".as_ref(),
            escrow.key().as_ref(),
            era.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + EscrowHistory::LEN,
        payer = payer
    )]
    pub escrow_history: AccountLoader<'info, EscrowHistory>,

    /// Payer.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// [System] program.
    pub system_program: Program<'info, System>,
}

impl<'info> CreateEscrowHistory<'info> {
    fn create_escrow_history(&mut self, bump: u8, era: u16) -> Result<()> {
        let history = &mut self.escrow_history.load_init()?;
        history.escrow = self.escrow.key();
        history.era = era;
        history.bump = bump;
        Ok(())
    }
}

pub fn handler(ctx: Context<CreateEscrowHistory>, era: u16) -> Result<()> {
    ctx.accounts
        .create_escrow_history(*unwrap_int!(ctx.bumps.get("escrow_history")), era)?;
    Ok(())
}

impl<'info> Validate<'info> for CreateEscrowHistory<'info> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
