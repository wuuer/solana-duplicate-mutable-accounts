use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod duplicate_mutable_accounts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_player.player = ctx.accounts.payer.key();
        ctx.accounts.new_player.games_played = 0;
        Ok(())
    }

    pub fn insecure_start_game(ctx: Context<InsecureGameStart>) -> Result<()> {
        ctx.accounts.player_one.games_played =
            ctx.accounts.player_one.games_played.checked_add(1).unwrap();

        ctx.accounts.player_two.games_played =
            ctx.accounts.player_two.games_played.checked_add(1).unwrap();
        Ok(())
    }

    pub fn secure_start_game(ctx: Context<SecureGameStart>) -> Result<()> {
        ctx.accounts.player_one.games_played =
            ctx.accounts.player_one.games_played.checked_add(1).unwrap();

        ctx.accounts.player_two.games_played =
            ctx.accounts.player_two.games_played.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8
    )]
    pub new_player: Account<'info, PlayerState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InsecureGameStart<'info> {
    #[account(mut)]
    pub player_one: Account<'info, PlayerState>,
    #[account(mut)]
    pub player_two: Account<'info, PlayerState>,
}

#[derive(Accounts)]
pub struct SecureGameStart<'info> {
    #[account(
        mut,
        constraint = player_one.key() != player_two.key())]
    pub player_one: Account<'info, PlayerState>,
    #[account(mut)]
    pub player_two: Account<'info, PlayerState>,
}

#[account]
pub struct PlayerState {
    player: Pubkey,
    games_played: u64,
}
