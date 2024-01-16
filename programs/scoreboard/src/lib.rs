use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("2qjPLYfvkTNB95mDgcnq4s8myTig2g1ya7HQfXwWJK5A");

const MAX_SCORES: usize = 20; // Define the maximum number of scores

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
        
        // CHECK: The signer is the player who's score is being added
        if ctx.accounts.signer.key() != new_score.player {
            return Err(ErrorCode::WrongSigner.into());
        }

        // CHECK: The scoreboard is not full
        if scoreboard.scores.len() == MAX_SCORES {
            return Err(ErrorCode::ScoreboardFull.into());
        }

        // Find the position to insert the new score
        let position = scoreboard.scores.iter()
                            .position(|x| x.score <= new_score.score)
                            .unwrap_or(scoreboard.scores.len());
    
        // Insert the new score at the found position
        scoreboard.scores.insert(position, new_score);
    
        Ok(())
    }
    
    // Function to reset scoreboard
    pub fn reset_scoreboard(ctx: Context<ResetScoreboardContext>) -> Result<()> {
        if ctx.accounts.signer.key() != ctx.accounts.scoreboard.authority {
            return Err(ErrorCode::Unauthorized.into());
        }
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
    /// CHECK: This is not dangerous because the signer is checked in the program
    #[account(signer)]
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ResetScoreboardContext<'info> {
    #[account(mut)]
    pub scoreboard: Account<'info, Scoreboard>,
    /// CHECK: This is not dangerous because the signer is checked in the program
    #[account(signer)]
    pub signer: AccountInfo<'info>,
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

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,

    #[msg("Signer does not match player.")]
    WrongSigner,

    #[msg("Scoreboard is full.")]
    ScoreboardFull,
}



