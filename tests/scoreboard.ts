import * as anchor from '@project-serum/anchor';
import { Program, Provider, web3 } from '@project-serum/anchor';
import { PublicKey, SystemProgram, Keypair } from '@solana/web3.js';
import { Scoreboard } from '../target/types/scoreboard';
import assert from 'assert';

describe('scoreboard', () => {
   
// Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local();
  

    const program = anchor.workspace.Scoreboard as Program<Scoreboard>;

    it('Initializes the scoreboard', async () => {
        const testSigner = Keypair.generate();

        const [scoreboardPda, _] = await PublicKey.findProgramAddressSync(
            [Buffer.from("scoreboard"), testSigner.publicKey.toBuffer()],
            program.programId,
        );

        const tx = await program.rpc.initializeScoreboard({
            accounts: {
                scoreboard: scoreboardPda,
                signer: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [testSigner],
        });

        await provider.connection.confirmTransaction(tx);

        const scoreboardAccount = await program.account.scoreboard.fetch(scoreboardPda);
        assert.equal(scoreboardAccount.authority.toBase58(), provider.wallet.publicKey.toBase58());
        // Assuming 'scores' is a valid field
        assert.deepEqual(scoreboardAccount.scores, []);
    });
});


