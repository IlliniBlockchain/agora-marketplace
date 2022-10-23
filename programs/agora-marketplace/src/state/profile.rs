use anchor_lang::prelude::*;

use super::username;

#[account]
pub struct ProfileAccount { //reputation, buy_count, sell_count, time, username
    //seed: userKey
    pub authority: Pubkey,
    pub reputation: u32,
    pub sell_count: u32,
    pub buy_count: u32,
    pub timestamp: i64,
    pub username: String
}

impl ProfileAccount{
    
    pub fn initialize(&mut username, pubkey: Pubkey) -> Result<()> {
        let username = &mut ctx.accounts.username;
        
        username.reputation = 0;
        username.sell_count = 0;
        username.buy_count = 0;
        username.timestamp = 0;

        username.authority = pubkey;
        Ok(())
    }
}