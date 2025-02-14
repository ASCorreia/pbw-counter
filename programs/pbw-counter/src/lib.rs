use anchor_lang::prelude::*;

declare_id!("5riL6yhP1mKW4yg8Uyj9qbpAGCFCCrPo5AzjGFvD3N9v");

mod state;
mod instructions;

use instructions::*;

#[program]
pub mod pbw_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps)?;

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment()?;

        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        ctx.accounts.decrement()?;

        Ok(())
    }
}
