import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PbwCounter } from "../target/types/pbw_counter";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("pbw-counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PbwCounter as Program<PbwCounter>;

  const counterPDA = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("counter"), provider.publicKey.toBuffer()], program.programId);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
    .accountsPartial({
      user: provider.publicKey,
      counter: counterPDA[0],
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .rpc();

    console.log("\nYour transaction signature", tx);

    const counter = await program.account.counter.fetch(counterPDA[0]);
    console.log("\nCounter", counter.count);
  });

  it("Increment!", async () => {
    // Add your test here.
    const tx = await program.methods.increment()
    .accountsPartial({
      user: provider.publicKey,
      counter: counterPDA[0],
    })
    .rpc();

    console.log("\nYour transaction signature", tx);

    const counter = await program.account.counter.fetch(counterPDA[0]);
    console.log("\nCounter", counter.count);
  });

  it("Decrement!", async () => {
    // Add your test here.
    const tx = await program.methods.decrement()
    .accountsPartial({
      user: provider.publicKey,
      counter: counterPDA[0],
    })
    .rpc();

    console.log("\nYour transaction signature", tx);

    const counter = await program.account.counter.fetch(counterPDA[0]);
    console.log("\nCounter", counter.count);
  });
});
