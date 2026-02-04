import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pursh } from "../target/types/pursh";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

async function main() {
  const program = anchor.workspace.Pursh as Program<Pursh>;

  const purshAccount = anchor.web3.Keypair.generate();

  await program.methods
    .initialize()
    .accounts({
      purshData: purshAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([purshAccount])
    .rpc();

  console.log("Pursh initialized:", purshAccount.publicKey.toBase58());

  await program.methods
    .increment()
    .accounts({
      purshData: purshAccount.publicKey,
    })
    .rpc();

  const data = await program.account.purshData.fetch(
    purshAccount.publicKey
  );

  console.log("Counter value:", data.counter.toString());
}

main();
