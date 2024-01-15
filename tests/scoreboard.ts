import * as anchor from '@coral-xyz/anchor';
import { Program, Provider, web3 } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram, Keypair } from '@solana/web3.js';
import { Scoreboard } from '../target/types/scoreboard';
import assert from 'assert';
import { BN } from 'bn.js';

describe('scoreboard', () => {
   
// Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local();

    // Wallet is from the wallet path in your Anchor.toml file
    const wallet = anchor.workspace.Scoreboard.provider.wallet.payer;

    const program = anchor.workspace.Scoreboard as Program<Scoreboard>;
    
    const testSigner = wallet;

    // for debugging purposes
    console.log("Seeds for PDA:", ["scoreboard", testSigner.publicKey.toBase58()]);
    console.log("testSigner's Public Key:", testSigner.publicKey.toBase58());

    const [scoreboardPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("scoreboard"), testSigner.publicKey.toBuffer()],
        program.programId,
    );

    it('Reset any existing scoreboard', async () => {
        const tx = await program.methods.resetScoreboard()
            .accounts({
                scoreboard: scoreboardPda,
            })
            .signers([testSigner])
            .rpc();
        console.log("reset transaction", tx);

        await provider.connection.confirmTransaction(tx);

        const scoreboardAccount = await program.account.scoreboard.fetch(scoreboardPda);
        assert.equal(scoreboardAccount.authority.toBase58(), testSigner.publicKey.toBase58());

        assert.deepEqual(scoreboardAccount.scores, []);
    });

    it('Initializes the scoreboard', async () => {

        if (await program.account.scoreboard.fetch(scoreboardPda) == null) {
            console.log("Scoreboard not initialized");
        

            const tx = await program.methods.initializeScoreboard()
                .accounts( 
                    {
                    scoreboard: scoreboardPda,
                    signer: testSigner.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    }
                )
                .signers([testSigner])
                .rpc();
            console.log("init transaction",tx);

            await provider.connection.confirmTransaction(tx);
        } else {
            console.log("Scoreboard already initialized");
        }
        const scoreboardAccount = await program.account.scoreboard.fetch(scoreboardPda);
        assert.equal(scoreboardAccount.authority.toBase58(), testSigner.publicKey.toBase58());
    
        // Assuming 'scores' is a valid field
        assert.deepEqual(scoreboardAccount.scores, []);
    });

    it('Player adds new score', async () => {
        const timestamp = new BN(Date.now());
        const addScoreTx = await program.methods
            .addScore(
                new BN(100),
                timestamp
            )
            .accounts({
                scoreboard: scoreboardPda,
            })
            .signers([testSigner])
            .rpc();

        console.log("addScore transaction", addScoreTx);
        
        const scoreboard = await program.provider.connection.getAccountInfo(
            scoreboardPda
        );

        const scoreboardData = scoreboard.data;
        const decodedData = program.coder.accounts.decode(
            "Scoreboard",
            scoreboardData
        );
        console.log("Decoded data", decodedData);
        console.log("player", decodedData.scores[0].player.toBase58());
        console.log("score", decodedData.scores[0].score.toString());
        console.log("timestamp", decodedData.scores[0].timestamp.toString());

        assert.equal(
            decodedData.scores[0].player.toBase58(),
            testSigner.publicKey.toBase58()
        );
        assert.equal(decodedData.scores[0].score.toString(), "100");
        assert.equal(decodedData.scores[0].timestamp.toString(), timestamp.toString());
    });

});
