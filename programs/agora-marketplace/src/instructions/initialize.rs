use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub authoriy: Signer<'info>,
    #[account(init, seeds = [username, authority.key().as_ref()], bump, payer = authority, space = 8 + std::mem:size_of::<ProfileAccount>(),)]
    pub profile_account: Box<Account<'info, ProfileAccount>>,
    

}