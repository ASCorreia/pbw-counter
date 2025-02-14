use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"counter", user.key().as_ref()],
        bump,
        space = 8 + Counter::INIT_SPACE,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {
        self.counter.set_inner(Counter {
            count: 0,
            bump: bumps.counter,
        });

        Ok(())
    }
}