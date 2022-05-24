import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { Auction } from "../target/types/auction";
import * as web3 from "@solana/web3.js";
import BN from "bn.js"

describe("auction", () => {

  const commitment: web3.Commitment = 'processed';
  const connection = new web3.Connection('https://api.devnet.solana.com ', { commitment, wsEndpoint: 'wss://api.devnet.solana.com/' });
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  const program = anchor.workspace.Auction as Program<Auction>;

  it("Is initialized!", async () => {
    
    const programId = new anchor.web3.PublicKey("61reie38A5ecZQ45ebeeCcQgBQ82NtA7h59jPLCzx6mK");
    const AUCTION_SIGNER_SEEDS = "testhuehuehuetest";

    const vault = web3.Keypair.generate();
    const seller = web3.Keypair.generate();
    const mint = new web3.PublicKey("9263LwjEN9zfdpGuVvWDD8fFvabgXsVWHQKQdUVELv4W");

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(seller.publicKey, 1000000000),
      "processed"
    );

    let price = new BN(web3.LAMPORTS_PER_SOL*2);

    const [auctionAccount, bump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("auction"),
        programId.toBuffer(),
        mint.toBuffer(),
        Buffer.from(AUCTION_SIGNER_SEEDS),
      ],
      programId
    );

    const tx = await program.methods.createAuction(price, bump).accounts({
      auctionAccount: auctionAccount,
      seller: seller.publicKey,
      mint: mint,
      systemProgram: web3.SystemProgram.programId,
    }).signers([seller]).rpc();

    console.log("Your transaction signature", program);
  });
});