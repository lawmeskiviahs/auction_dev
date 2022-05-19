import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { Auction } from "../target/types/auction";
import * as web3 from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import BN from "bn.js"

describe("auction", () => {

  const commitment: web3.Commitment = 'processed';
  const connection = new web3.Connection('https://api.devnet.solana.com ', { commitment, wsEndpoint: 'wss://api.devnet.solana.com/' });
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  const program = anchor.workspace.Auction as Program<Auction>;

  it("Is initialized!", async () => {
    
    const programId = new anchor.web3.PublicKey("9fnWiZpicj8MNHymzYiKpA9GtdxqKeV5p7AYevp9hzWF");
    const AUCTION_SIGNER_SEEDS = "testhuehuehuetest";

    const vault = web3.Keypair.generate();
    const seller = web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(seller.publicKey, 1000000000),
      "processed"
    );
    
    let mint = await Token.createMint(
      provider.connection,
      seller,
      seller.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    let sellerTokenAccount = await mint.createAccount(seller.publicKey);
    let vaultTokenAccount = await mint.createAccount(vault.publicKey);

    let price = new BN(web3.LAMPORTS_PER_SOL*2);

    const [auctionAccount, bump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("auction"),
        programId.toBuffer(),
        mint.publicKey.toBuffer(),
        Buffer.from(AUCTION_SIGNER_SEEDS),
      ],
      programId
    );

    // const tx = await program.methods.createAuction(price, bump).accounts({
    //   auctionAccount: auctionAccount,
    //   seller: seller.publicKey,
    //   mint: mint.publicKey,
    //   systemProgram: web3.SystemProgram.programId,
    // }).signers([seller]).rpc();
    console.log("Your transaction signature", program);
  });
});