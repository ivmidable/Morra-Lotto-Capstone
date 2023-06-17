use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;


#[program]
pub mod morra_lotto {
    use crate::instruction::{BuyTicket, RevealMoves};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // init GameState
        let game_state = &mut ctx.accounts.game_state;
        game_state.total_tickets = 0;
        game_state.ticket_price = 1 * LAMPORTS_PER_SOL;
        game_state.min_move = 0;
        game_state.max_move = 5;

        // init VaultState
        ctx.accounts.vault_state.owner = *ctx.accounts.buyer.key;
        ctx.accounts.vault_state.auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        ctx.accounts.vault_state.vault_bump = *ctx.bumps.get("vault").unwrap();

        // game_state.stage_link = 
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, hash: [u8;32] ) -> Result<()> {
        // pass in hash of hand and sum. (have to save in state)
// ref vault code

        Ok(())
    }

    pub fn reveal_moves(ctx: Context<RevealMoves>) -> Result<()> {

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {

        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(init, payer = buyer, space = 8 + 32 + 3)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    ///CHECK: NO NEED TO CHECK THIS
    pub vault_auth:  UncheckedAccount<'info,>,
    #[account(mut, seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        seeds = [b"lotto".as_ref(), buyer.key().as_ref()],
        bump,
        payer = buyer,
        //count new space
        space = 8 + 32 + 4 + 4 + 4 + 32 + 8 + 8 + 32,
    )]
    pub game_state: Account<'info, GameState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTicket <'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(init, payer = buyer, space = 8 + 32 + 3)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    ///CHECK: NO NEED TO CHECK THIS
    pub vault_auth:  UncheckedAccount<'info,>,
    #[account(mut, seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    owner: Pubkey,
    auth_bump: u8,
    vault_bump: u8,
}

#[account]
pub struct TicketInfo {
    player: Pubkey,
    player_move: u8,
    guess_sum: u8,
}

#[account]
pub struct GameState {
    total_tickets: u64,
    ticket_price: u64,
    min_move: u8,
    max_move: u8,
    stage_link: u64,
    // base_pot: u16
}

