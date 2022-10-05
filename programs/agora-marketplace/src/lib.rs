use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod agora_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct ProfileAccount { //reputation, buy_count, sell_count, time, username
    //seed: userKey
}

#[account]
pub struct UserAccount { //userKey
    //seed: username
}

#[derive(Accounts)]
pub struct Initialize {

}
