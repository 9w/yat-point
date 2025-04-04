use anchor_lang::prelude::*;

declare_id!("Cm3ERP2QqMthfVuEfr9x1G1iEKuE8BCC5UwzfgjowvJn");

#[program]
pub mod yat_point {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
