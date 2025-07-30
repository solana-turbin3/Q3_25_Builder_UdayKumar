import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/anchor_vault";

describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vault as Program<Vault>;
  it("Is initialized!", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Vault as Program<Vault>;
    it("Initialzing the vault", async()=>{
      const vaultState = anchor.web3.Keypair.generate();
      await program.methods
      .initialize()
      .accounts({
        vaultState: vaultState.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([vaultState])
      .rpc();
      console.log("Vault initialized:", vaultState.publicKey.toBase58());
    })
  });
});
