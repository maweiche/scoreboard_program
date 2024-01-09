use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod scoreboard {
    use super::*;

    // Function - Initialize the program with the authority.
    pub fn initialize_scoreboard(ctx: Context<InitializeScoreboard>) -> Result<()> {
        ctx.accounts.scoreboard.authority = *ctx.accounts.signer.key;
        ctx.accounts.scoreboard.scores = Vec::new();
        
        Ok(())
    }

    // Function - Add a new score to the scoreboard.

    // Function - Remove a score from the scoreboard.

    // Function - Update a score on the scoreboard.

    // Function - Get the top 10 scores on the scoreboard.


}

#[derive(Accounts)]
pub struct InitializeScoreboard<'info> {
    #[account(
        init,
        seeds = [
            b"scoreboard",
            signer.key().as_ref(),
        ],
        bump,
        payer = signer,
        space = 1000
    )] //size can be refactored to enhance program size -> https://www.anchor-lang.com/docs/space
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

#[account]
pub struct Score {
    pub score: u64,
    pub name: String,
    pub timestamp: i64,
    pub nft_used: Pubkey,
    pub authority: Pubkey,
}


// Create Error Types