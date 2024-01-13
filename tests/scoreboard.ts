import * as anchor from '@coral-xyz/anchor';
import { Program, Provider, web3 } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram, Keypair } from '@solana/web3.js';
import { Scoreboard } from '../target/types/scoreboard';
import assert from 'assert';

describe('scoreboard', () => {
   
// Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local();
  

    const program = anchor.workspace.Scoreboard as Program<Scoreboard>;
    
    const testSigner = Keypair.generate();

      // for debugging purposes
    console.log("Seeds for PDA:", ["scoreboard", testSigner.publicKey.toBase58()]);


    it('Initializes the scoreboard', async () => {
       
    

        const res = await anchor.getProvider().connection.requestAirdrop(testSigner.publicKey, 1e9);
        await anchor.getProvider().connection.confirmTransaction(res, "confirmed");


        const [scoreboardPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("scoreboard"), testSigner.publicKey.toBuffer()],
            program.programId,
        );

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

        // for debugging purposes
        console.log("testSigner's Public Key:", testSigner.publicKey.toBase58());

        const scoreboardAccount = await program.account.scoreboard.fetch(scoreboardPda);
        assert.equal(scoreboardAccount.authority.toBase58(), testSigner.publicKey.toBase58());
       
        // Assuming 'scores' is a valid field
        assert.deepEqual(scoreboardAccount.scores, []);
    });
});
