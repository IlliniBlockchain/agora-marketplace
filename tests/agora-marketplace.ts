import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AgoraMarketplace } from "../target/types/agora_marketplace";

describe("agora-marketplace", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AgoraMarketplace as Program<AgoraMarketplace>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
