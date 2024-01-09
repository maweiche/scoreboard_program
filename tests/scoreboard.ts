import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Scoreboard } from "../target/types/scoreboard";

describe("scoreboard", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Scoreboard as Program<Scoreboard>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
