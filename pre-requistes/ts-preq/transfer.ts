import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey  } from "@solana/web3.js";
import wallet from './dev-wallet.json';

const connection = new Connection("https://api.devnet.solana.com");
const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("BVsResBPa1Uhddmk82tqqQ4ZNkVXtop9hgZ95EPXYiAt");

const transferSol = async () =>{
    try{
        //get balance of dev wallet
        const balance = await connection.getBalance(from.publicKey);
        // adding a transaction into the transaction array;
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance // by mistake sent 0.1 sol instead of 0.01 sol shh
            })
        );
        transaction.recentBlockhash = (
            await connection.getLatestBlockhash('confirmed')
        ).blockhash;
        transaction.feePayer = from.publicKey;
        //calculate the fee rate to transfer entire SOL Amount of account minus fees.
        const fee = (await connection.getFeeForMessage(transaction.compileMessage(),'confirmed')).value || 0;
        //removing the last transaction from transaction array
        transaction.instructions.pop();

        //adding the new transaction again

        transaction.add(
            SystemProgram.transfer({
                fromPubkey:from.publicKey,
                toPubkey: to,
                lamports: balance - fee
            })
        )
        //sign transaction, broadcast and confirm
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        );
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    }catch(err){
        console.error(`Oops, something went wrong: ${err}`);
    }
}

transferSol()

