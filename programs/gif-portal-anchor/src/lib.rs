use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod gif_portal_anchor {
    use anchor_lang::solana_program::entrypoint::ProgramResult;
    use super::*;

   pub fn start_stuff_off(ctx: Context<StartStuffOff>)->ProgramResult {
        Ok(())
   }
}

#[derive(Accounts)]
pub struct StartStuffOff{}
