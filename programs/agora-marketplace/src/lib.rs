use anchor_lang::prelude::*;
use instructions::*;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("HqQQTC8BDrCiJH8BEWMuHK4bVkV3H3VyYG15fLc4rXmn");

#[program]
pub mod agora_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _uname: String) -> Result<()> {
        instructions::initialize::initialize(ctx, _uname)
    }
}