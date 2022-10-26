use anchor_lang::prelude::*;

#[account]
pub struct User { //userKey
    //seed: username
    user: Pubkey
}

impl User {
    pub const STATIC_SIZE: usize = 8 + 32;

    pub fn initialize(&mut self, _pubkey: Pubkey) -> Result<()> {
        self.user = _pubkey;
        Ok(())
    }
}

//random change