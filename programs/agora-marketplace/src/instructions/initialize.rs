use anchor_lang::prelude::*;
use crate::state::profile::*;
use crate::state::username::*;
use crate::error::InputError;

pub fn initialize(ctx: Context<Initialize>, _uname: String) -> Result<()> {
    if _uname.len() > 20 {
        return err!(InputError::LongUserName);
    }

    ctx.accounts.username.initialize(ctx.accounts.user.key())?;
    ctx.accounts.profile.initialize(_uname)?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(_uname: String)]
pub struct Initialize <'info>{
    #[account{
        init,
        payer = user,
        space = User::STATIC_SIZE,
        seeds = ["username".as_bytes(), _uname.as_bytes()], bump
    }]
    pub username: Account<'info, User>,
    #[account(
        init,
        payer = user,
        space = Profile::STATIC_SIZE + _uname.len(),
        seeds = ["profile".as_bytes(), user.key().as_ref()], bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}