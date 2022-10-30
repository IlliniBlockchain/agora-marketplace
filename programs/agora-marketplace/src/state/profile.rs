use anchor_lang::prelude::*;

#[account]
pub struct Profile { //reputation, buy_count, sell_count, time, username
    //seed: userKey
    pub reputation: u32,
    pub sell_count: u32,
    pub buy_count: u32,
    pub timestamp: i64,
    pub username: String,
}

impl Profile {

    pub const STATIC_SIZE: usize = 8 + 4 + 4 + 4 + 8 + 4; //missing string length
    
    pub fn initialize(&mut self, _username: String) -> Result<()> {

        let clock: Clock = Clock::get()?;

        self.reputation = 0;
        self.sell_count = 0;
        self.buy_count = 0;
        self.timestamp = clock.unix_timestamp;
        self.username = _username;

        Ok(())
    }
}