import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MultisigContract } from "../target/types/multisig_contract";
import { AccountsResolver } from "@coral-xyz/anchor/dist/cjs/program/accounts-resolver";
import { expect } from "chai";

describe("multisig_contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.multisigContract as Program<MultisigContract>;
  const wallet1 = anchor.web3.Keypair.generate()
  const wallet2 = anchor.web3.Keypair.generate()
 before(async() => {
  const sig = await provider.connection.requestAirdrop(wallet1.publicKey,1000*anchor.web3.LAMPORTS_PER_SOL)
  await provider.connection.confirmTransaction({
    signature:sig,
  ...(await provider.connection.getLatestBlockhash())
  })
  const sig1 = await provider.connection.requestAirdrop(wallet2.publicKey,1000*anchor.web3.LAMPORTS_PER_SOL)
  await provider.connection.confirmTransaction({
    signature:sig1,
  ...(await provider.connection.getLatestBlockhash())
  })
 })
 const [multiSigPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("multisig"),Buffer.from("mulsig"), wallet1.publicKey.toBuffer()],
    program.programId
  );
 const [proposalPda, proposalbump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("proposal"),multiSigPda.toBuffer(),new anchor.BN(0).],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize("mulsig",[wallet1.publicKey,wallet2.publicKey],Buffer.from([1]),
      Buffer.from([2]),Buffer.from([1]),Buffer.from([4,0]),3
    ).accounts({creator:wallet1.publicKey,
    
    }).signers([wallet1]).rpc();
    console.log("Your transaction signature", tx);

      const accountinfo  = await program.account.multisigState.fetch(multiSigPda)
      expect(accountinfo.creator).to.deep.equal(wallet1.publicKey)
      expect(accountinfo.multisigName).to.equal("mulsig")
      expect(accountinfo.approveThreshold).to.equal(3)
      
  });

  it("proposal creation", async () => {
    // Add your test here.
    const tx = await program.methods.proposalCreate({transferSol:{}},
      new anchor.BN(10*anchor.web3.LAMPORTS_PER_SOL),
     new anchor.BN(1771718400),
      wallet2.publicKey
    ).accountsPartial({signer:wallet1.publicKey,
    
    }).signers([wallet1]).rpc();
    console.log("Your transaction signature", tx);

      const accountinfo  = await program.account.proposal.fetch(proposalPda)
      expect(accountinfo.multisig).to.deep.equal(multiSigPda)
      expect(accountinfo.destination).to.deep.equal(wallet2.publicKey)
      expect(accountinfo.executed).to.equal(false)
      expect(new anchor.BN(accountinfo.transferAmount)).to.equal(new anchor.BN(accountinfo.transferAmount))
      
  });




});
