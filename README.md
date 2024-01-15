# Scoreboard Solana Program

This is a simple Solana program that implements a scoreboard built for Elusiv's Bootcamp Project. It is intended to be used as a reference for how to write Solana programs.

## Goals

- [✅] allow users to submit their scores to a scoreboard.
- [x] allow users to retrieve their own score from the scoreboard.



## To run locally:

1. run `yarn install` to install dependencies.
2. change wallet path in `Cargo.toml` to your CLI wallet path, you can access this by running `solana config get`.
3. make sure your Solana CLI is set to local host by running `solana config set --url localhost`
4. Build the program by running `anchor build` (this will generate a `/target` folder).
5. Open second terminal and run `solana-test-validator` to start a local Solana cluster.
6. back in the first terminal window, run `anchor deploy` to deploy program to your local validator, this will produce a `Program Id` in your terminal.
7. Copy the `Program Id` and paste it into the `ProgramId` variable in `Anchor.toml`:
    ```
    [programs.localnet]
    scoreboard = "3uj1ZssrUsXA5rMJRj9M5tq4smLdyjNFAvTknvGKBX21" 
    ```
8. Run `anchor build` and `anchor deploy` again for ProgramId to take effect.
9. Run `anchor test --skip-local-validator` to run the tests.


    
