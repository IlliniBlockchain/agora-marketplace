use anchor_lang::prelude::*;

#[account]
pub struct UserAccount { //userKey
    //seed: username
    user: Pubkey
}

impl UserAccount{
    pub const STATIC_SIZE: usize = 8 + 32;

    pub fn initialize(&mut self, pubkey: Pubkey) -> Result<()> {
        self.user = pubkey;
        Ok(())
    }
}

//random change