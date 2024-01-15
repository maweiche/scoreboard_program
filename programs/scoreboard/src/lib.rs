use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("3uj1ZssrUsXA5rMJRj9M5tq4smLdyjNFAvTknvGKBX21");

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

    // Function to add a new score to the scoreboard
    pub fn add_score(ctx: Context<AddScoreContext>, score: u64, timestamp: i64) -> Result<()> {
        let scoreboard = &mut ctx.accounts.scoreboard;
        let player = ctx.accounts.signer.key();
        let new_score = Score { player, score, timestamp };

        // Add the new score to the scoreboard
        scoreboard.scores.push(new_score);

        // Sort the scoreboard
        scoreboard.scores.sort_by(|a, b| b.score.cmp(&a.score));

        Ok(())
    }
    
    // Function to reset scoreboard
    pub fn reset_scoreboard(ctx: Context<ResetScoreboardContext>) -> Result<()> {
        let scoreboard = &mut ctx.accounts.scoreboard;
        scoreboard.scores = Vec::new(); // Reset the scores vector
        Ok(())
    }
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

#[derive(Accounts)]
pub struct AddScoreContext<'info> {
    #[account(mut)]
    pub scoreboard: Account<'info, Scoreboard>,
    /// CHECK: The signer is the player who's score is being added
    #[account(signer)]
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ResetScoreboardContext<'info> {
    #[account(mut)]
    pub scoreboard: Account<'info, Scoreboard>,
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
