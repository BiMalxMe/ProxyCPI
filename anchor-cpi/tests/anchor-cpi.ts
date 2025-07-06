import * as anchor from "@project-serum/anchor";
import { assert } from "chai";

describe("CPI Wrapper -> Native Program", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  // Replace with your actual native program ID (deployed native program)
  const nativeProgramId = new anchor.web3.PublicKey("G6D4ZXSMb9WmVtVfM78hy2KwcqwG9ZUSrjVZVyc2tHwx");

  // Replace with your Anchor wrapper program ID
  const wrapperProgramId = new anchor.web3.PublicKey("3vpQib3CfZgzDZkZjjJcfW6ikMee8kp8jLDYNP9Kj83w");

  // Load your wrapper program IDL
  const wrapperIdl = require("../target/idl/cpi_wrapper.json");
  const wrapperProgram = new anchor.Program(wrapperIdl, wrapperProgramId, provider);

  // Generate a new keypair for data_account
  const dataAccount = anchor.web3.Keypair.generate();

  it("Initializes the counter via CPI wrapper", async () => {
    await wrapperProgram.methods
      .initialize()
      .accounts({
        data_account: dataAccount.publicKey,
        user_account: provider.wallet.publicKey,
        system_program: anchor.web3.SystemProgram.programId,
        native_program: nativeProgramId,  // Pass native program ID for CPI
      })
      .signers([dataAccount])  // signer if your wrapper/native expects it
      .rpc();

    const accountInfo = await provider.connection.getAccountInfo(dataAccount.publicKey);
    assert(accountInfo !== null, "data_account should be initialized and exist");
  });
});
