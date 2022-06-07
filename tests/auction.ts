import * as anchor from "@project-serum/anchor";
import { Program, validateAccounts } from "@project-serum/anchor";
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { Auction } from "../target/types/auction";
import * as web3 from "@solana/web3.js";
import BN from "bn.js"
import { program } from "@project-serum/anchor/dist/cjs/spl/token";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("auction", () => {

  const commitment: web3.Commitment = 'processed';
  const connection = new web3.Connection('https://api.devnet.solana.com ', { commitment, wsEndpoint: 'wss://api.devnet.solana.com/' });
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  const program = anchor.workspace.Auction as Program<Auction>;

  it("Is initialized!", async () => {
    
    const programId = new anchor.web3.PublicKey("3fsYeopwx1UBtTD34PEdKVo5yvCw7jK7mKxsYWjxE76a");
    const AUCTION_SIGNER_SEEDS = "yaxche";

    // const vault = web3.Keypair.generate();
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
    const seller_basis_point:number = 10;
    const royalty = new BN(seller_basis_point)
    const tx = await program.methods.initialize(price, bump, seller_basis_point).accounts({
      auctionAccount: auctionAccount,
      seller: seller.publicKey,
      mint: mint,
      // systemProgram: web3.SystemProgram.programId,
    }).signers([seller]).rpc();

    console.log("Your transaction signature", program);

    const txInit = await program.rpc.initialize(price, bump, royalty, {
      accounts: {
        auctionAccount: auctionAccount,
        seller: wallet.publicKey,
        mint: mint,
        systemProgram: web3.SystemProgram.programId,
      },
    });

     const txEndEnglishAuction = await program.rpc.endEnglishAuction(bump, {
        accounts: {
          auctionAccount: auctionAccount,
          mint: mintKey,
          toAccount: wallet.publicKey,
          fromTokenAccount: fromTokenAccount,
          toTokenAccount: toTokenAccount,
          vault: vault.publicKey,
          seller: account.seller,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId
        },
      });
  });
});
