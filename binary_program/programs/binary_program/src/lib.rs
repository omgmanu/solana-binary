use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;

declare_id!("HQ1GUunXVdBBCc7iFdS8y2Q423JD8sMXuBxmGT7n2LzX");

#[program]
pub mod btc_binary_options {
    use super::*;

    pub fn start_game(ctx: Context<StartGame>, timeframe: u64, bet_amount: u64, prediction: bool) -> Result<()> {
        let game: &mut Account<Game> = &mut ctx.accounts.game;
        let clock: Clock = Clock::get()?;

        game.player = ctx.accounts.player.key();
        game.start_time = clock.unix_timestamp as u64;
        game.end_time = game.start_time + timeframe;
        game.bet_amount = bet_amount;
        game.prediction = prediction;
        game.is_settled = false;

        // Transfer bet amount from player to game account
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.player.to_account_info(),
                to: ctx.accounts.game.to_account_info(),
            },
        );
        
        anchor_lang::system_program::transfer(cpi_context, bet_amount)?;

        Ok(())
    }

    pub fn settle_game(ctx: Context<SettleGame>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let clock = Clock::get()?;

        require!(!game.is_settled, ErrorCode::GameAlreadySettled);
        require!(clock.unix_timestamp as u64 >= game.end_time, ErrorCode::GameNotFinished);

        let price_feed = load_price_feed_from_account_info(&ctx.accounts.pyth_price_feed).unwrap();
        let current_price = price_feed.get_price_unchecked().price;
        let start_price = price_feed.get_price_no_older_than(game.start_time as i64, 60).unwrap().price;

        let price_increased = current_price > start_price;
        let player_won = game.prediction == price_increased;

        if player_won {
            // Transfer double the bet amount to the player
            let winner_amount = game.bet_amount * 2;
            **game.to_account_info().try_borrow_mut_lamports()? -= winner_amount;
            **ctx.accounts.player.try_borrow_mut_lamports()? += winner_amount;
        } else {
            // Transfer the bet amount to the house (you can modify this as needed)
            let house_amount = game.bet_amount;
            **game.to_account_info().try_borrow_mut_lamports()? -= house_amount;
            **ctx.accounts.house.try_borrow_mut_lamports()? += house_amount;
        }

        game.is_settled = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartGame<'info> {
    #[account(init, payer = player, space = 8 + 32 + 8 + 8 + 8 + 1 + 1)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettleGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    /// CHECK: This account is not written to or read from in this instruction
    #[account(mut)]
    pub player: AccountInfo<'info>,
    /// CHECK: This account is not written to or read from in this instruction
    #[account(mut)]
    pub house: AccountInfo<'info>,
    /// CHECK: This account is verified in the instruction
    pub pyth_price_feed: AccountInfo<'info>,
}

#[account]
pub struct Game {
    pub player: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    pub bet_amount: u64,
    pub prediction: bool,
    pub is_settled: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The game has already been settled")]
    GameAlreadySettled,
    #[msg("The game has not finished yet")]
    GameNotFinished,
}