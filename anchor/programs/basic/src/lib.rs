use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

#[program]
pub mod basic {
    use super::*;

    pub fn initialize_config(_ctx: Context<Initialize>) -> Result<()> {
       
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    
    #[account(mut)]
    pub payer : Signer<'info>,

}
