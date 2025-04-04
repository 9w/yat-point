import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { YatPoint } from "../target/types/yat_point";

describe("yat_point", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.yatPoint as Program<YatPoint>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
