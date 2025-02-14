use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Decrement<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
}

impl<'info> Decrement<'info> {
    pub fn decrement(&mut self) -> Result<()> {
        self.counter.count -= 1;
        Ok(())
    }
}