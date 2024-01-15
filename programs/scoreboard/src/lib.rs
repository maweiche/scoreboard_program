use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("BBYN3Ss1Kw8vTKdiWooEhRQYLmENNXtGyrYHsZumj7jh");

const MAX_SCORES: usize = 10; // Define the maximum number of scores

#[program]
pub mod scoreboard {
    use super::*;

    // Initializes the scoreboard
    pub fn initialize_scoreboard(ctx: Context<InitializeScoreboard>) -> Result<()> {
        let scoreboard = &mut ctx.accounts.scoreboard;
        scoreboard.authority = *ctx.accounts.signer.key;
        scoreboard.scores = Vec::new(); // Initialize the scores vector
        Ok(())
    }

    // Additional functions for managing the scoreboard

    // Function to add a new score to the scoreboard
    pub fn add_score(ctx: Context<AddScoreContext>, player: Pubkey, score: u64, timestamp: i64) -> Result<()> {
        let scoreboard = &mut ctx.accounts.scoreboard;
        let new_score = Score { player, score, timestamp };

        // Add the new score to the scoreboard
        scoreboard.scores.push(new_score);

        Ok(())
    }

}


#[derive(Accounts)]
pub struct AddScoreContext<'info> {
    #[account(mut)]
    pub scoreboard: Account<'info, Scoreboard>,
}

#[derive(Accounts)]
pub struct InitializeScoreboard<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + (8 + size_of::<Score>() * MAX_SCORES),
        seeds = [b"scoreboard", signer.key().as_ref()],
        bump
    )]
    pub scoreboard: Account<'info, Scoreboard>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Scoreboard {
    pub authority: Pubkey,
    pub scores: Vec<Score>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Score {
    pub player: Pubkey,
    pub score: u64,
    pub timestamp: i64,
}
