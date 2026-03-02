import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MultisigContract } from "../target/types/multisig_contract";
import { AccountsResolver } from "@coral-xyz/anchor/dist/cjs/program/accounts-resolver";
import { expect } from "chai";
import chaiBN from "chai-bn";

describe("multisig_contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.multisigContract as Program<MultisigContract>;
  const wallet1 = anchor.web3.Keypair.generate()
  const wallet2 = anchor.web3.Keypair.generate()
  before(async () => {
    const sig = await provider.connection.requestAirdrop(wallet1.publicKey, 1000 * anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction({
      signature: sig,
      ...(await provider.connection.getLatestBlockhash())
    })
    const sig1 = await provider.connection.requestAirdrop(wallet2.publicKey, 1000 * anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction({
      signature: sig1,
      ...(await provider.connection.getLatestBlockhash())
    })
  })
  const [multiSigPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("multisig"), Buffer.from("mulsig"), wallet1.publicKey.toBuffer()],
    program.programId
  );
  const [proposalPda, proposalbump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("proposal"), multiSigPda.toBuffer(), new anchor.BN(0).toArrayLike(Buffer, "le", 8)],
    program.programId
  );
  const [vaultstatePda, vaultstatebump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_state"), multiSigPda.toBuffer(), wallet1.publicKey.toBuffer()],
    program.programId
  );
  const [vaultPda, vaultbump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultstatePda.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize("mulsig", [wallet1.publicKey, wallet2.publicKey], Buffer.from([0]),
      Buffer.from([0]), Buffer.from([0]), Buffer.from([4, 0]), 3
    ).accounts({
      creator: wallet1.publicKey,

    }).signers([wallet1]).rpc();
    console.log("Your transaction signature", tx);
    const sig = await provider.connection.requestAirdrop(vaultPda, 1000 * anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction({
      signature: sig,
      ...(await provider.connection.getLatestBlockhash())
    })

    const accountinfo = await program.account.multisigState.fetch(multiSigPda)

    expect(accountinfo.creator).to.deep.equal(wallet1.publicKey)
    expect(accountinfo.multisigName).to.equal("mulsig")
    expect(accountinfo.approveThreshold).to.equal(3)

  });

  it("proposal creation", async () => {
    // Add your test here.
    const tx = await program.methods.proposalCreate({ transferSol: {} },
      new anchor.BN(10 * anchor.web3.LAMPORTS_PER_SOL),
      new anchor.BN(1771718400),
      wallet2.publicKey
    ).accountsPartial({
      signer: wallet1.publicKey,
      multisigConfig: multiSigPda,
      proposal: proposalPda

    }).signers([wallet1]).rpc();
    console.log("Your transaction signature", tx);

    const accountinfo = await program.account.proposal.fetch(proposalPda)
    expect(accountinfo.multisig).to.deep.equal(multiSigPda)
    expect(accountinfo.destination).to.deep.equal(wallet2.publicKey)
    expect(accountinfo.executed).to.equal(false)
    expect(new anchor.BN(accountinfo.transferAmount)).to.equal(new anchor.BN(accountinfo.transferAmount))

  });
  it("proposal approval", async () => {
    // Add your test here.
    const tx = await program.methods.approvaProposal(new anchor.BN(0)).accountsPartial({
      signer: wallet1.publicKey,
      multisigConfig: multiSigPda,
      proposal: proposalPda

    }).signers([wallet1]).rpc();
    console.log("Your transaction signature", tx);

    const accountinfo = await program.account.proposal.fetch(proposalPda)
    expect(accountinfo.multisig).to.deep.equal(multiSigPda)
    expect(accountinfo.destination).to.deep.equal(wallet2.publicKey)
    expect(accountinfo.executed).to.equal(false)
    expect(new anchor.BN(accountinfo.transferAmount)).to.equal(new anchor.BN(accountinfo.transferAmount))

  });

  it("propsal execution", async () => {
    // Add your test here.
    const tx = await program.methods.executeProposal(new anchor.BN(0)).accountsPartial({
      signer: wallet1.publicKey,
      destination: wallet2.publicKey,
      multisigConfig: multiSigPda,
      proposal: proposalPda,


    }).signers([wallet1]).rpc();
    console.log("Your transaction signature", tx);

    const accountinfo = await program.account.proposal.fetch(proposalPda)
    const destinationaccount = await provider.connection.getAccountInfo(wallet2.publicKey)
    expect(accountinfo.multisig).to.deep.equal(multiSigPda)
    // expect(new anchor.BN(destinationaccount.lamports)).to.deep.equals(accountinfo.transferAmount)

    expect(accountinfo.destination).to.deep.equal(wallet2.publicKey)
    expect(accountinfo.executed).to.equal(true)
    expect(new anchor.BN(accountinfo.transferAmount)).to.equal(new anchor.BN(accountinfo.transferAmount))

  });


  it("proposal token creation", async () => {
    // Add your test here.
    const mintkey = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
    const tx = await program.methods.proposalCreate({ transferToken: {} },
      new anchor.BN(10 * anchor.web3.LAMPORTS_PER_SOL),
      new anchor.BN(1771718400),
      wallet2.publicKey
    ).accountsPartial({
      signer: wallet1.publicKey,
      multisigConfig: multiSigPda,
      proposal: proposalPda
      

    }).signers([wallet1]).rpc();
    console.log("Your transaction signature", tx);

    const accountinfo = await program.account.proposal.fetch(proposalPda)
    expect(accountinfo.multisig).to.deep.equal(multiSigPda)
    expect(accountinfo.destination).to.deep.equal(wallet2.publicKey)
    expect(accountinfo.executed).to.equal(false)
    expect(new anchor.BN(accountinfo.transferAmount)).to.equal(new anchor.BN(accountinfo.transferAmount))

    expect(accountinfo.proposalType.transferToken).to.not.be.null;

  });

  it("propsal execution token", async () => {
    // Add your test here.
    const tx = await program.methods.executeTokenProposal(new anchor.BN(1)).accountsPartial({
      signer: wallet1.publicKey,
      destination: wallet2.publicKey,
      multisigConfig: multiSigPda,
      proposal: proposalPda,
      


    }).signers([wallet1]).rpc();
    console.log("Your transaction signature", tx);

    const accountinfo = await program.account.proposal.fetch(proposalPda)
    const destinationaccount = await provider.connection.getAccountInfo(wallet2.publicKey)
    expect(accountinfo.multisig).to.deep.equal(multiSigPda)
    // expect(new anchor.BN(destinationaccount.lamports)).to.deep.equals(accountinfo.transferAmount)

    expect(accountinfo.destination).to.deep.equal(wallet2.publicKey)
    expect(accountinfo.executed).to.equal(true)
    expect(new anchor.BN(accountinfo.transferAmount)).to.equal(new anchor.BN(accountinfo.transferAmount))

  });






});
