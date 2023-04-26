import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SoltrekProgram } from "../target/types/soltrek_program";


describe("soltrek-program", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SoltrekProgram as Program<SoltrekProgram>;
  const provider = anchor.AnchorProvider.local();
  const wallet = provider.wallet as anchor.Wallet;

  let [user_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      wallet.publicKey.toBuffer()
    ],
    program.programId
  )


  it("User initialized!", async () => {
    // Add your test here.

    const ix = await program.methods.createUser().accounts({
      userAccount: user_account
    }).instruction();

    const tx = new anchor.web3.Transaction().add(ix)
    await program.provider.sendAndConfirm(tx)
    console.log("Your transaction signature", tx);
  });

  it("Playground initialized!", async () => {
    // Add your test here.

    let details = await program.account.user.fetch(user_account)
    let [playground_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        wallet.publicKey.toBuffer(),
        Buffer.from(String(details.playgroundCount.toNumber() + 1))
      ],
      program.programId

    )
    const ix = await program.methods.createPlayground(String(details.playgroundCount.toNumber() + 1), "hi").accounts({
      userAccount: user_account,
      playgroundAccount: playground_account,
    }).instruction();

    const tx = new anchor.web3.Transaction().add(ix)
    await program.provider.sendAndConfirm(tx)
    console.log("Your transaction signature", tx);
  });
});
