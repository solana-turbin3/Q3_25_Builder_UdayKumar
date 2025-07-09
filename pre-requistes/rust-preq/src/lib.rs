#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::rpc_client::RpcClient;
    // use solana_program::{pubkey::Pubkey,system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        pubkey::Pubkey,
        signature::{Keypair, Signer, read_keypair_file},
        system_instruction::transfer,
        transaction::Transaction,
        // hash::hash,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        //print the public key
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() {
        //importing keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // establishing a connection to Solana devnet using the const we defined above
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        //connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);
        let to_pubkey = Pubkey::from_str("BVsResBPa1Uhddmk82tqqQ4ZNkVXtop9hgZ95EPXYiAt").unwrap();
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent block hash");
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    // let amount = balance.saturating_sub(fee);



        // let message = Message::new_with_blockhash(
        //     &[transfer(&keypair.pubkey(), &to_pubkey, amount)],
        //     Some(&keypair.pubkey()),
        //     &recent_blockhash,
        // );

        //fetching recent blockhash

        //final transaction
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed ro send final transaction");
        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

        // //generate a signature form keypair
        // let pub_key = keypair.pubkey();
        // let message_bytes = b"I verify my Solana Keypair!";
        // let sig = keypair.sign_message(message_bytes);
        // let sig_hashed = hash(sig.as_ref());

        // //verifying the signature using public key
        // match sig.verify(&pub_key.to_bytes(), &sig_hashed.to_bytes()){
        //     true => println!("Signature Verified"),
        //     false => println!("Verification failed"),
        // }

        // //estimate transaction fee
    }
    #[test]
    fn submit_rs(){
        use solana_sdk::instruction::AccountMeta;
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbine-Wallet.json").expect("Couldnt find wallet details");
        let program_id = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").expect("Invalid program ID");
        let user_pubkey = signer.pubkey();
        let (account_pda, bump) = Pubkey::find_program_address(
            &[b"prereqs", user_pubkey.as_ref()],
            &program_id,
        );
        println!("Derived account PDA: {}", account_pda);
        println!("Bump: {}", bump);
        
        //loading keypair
        let signer = read_keypair_file("Turbine-Wallet.json").expect("Couldnt find wallet");

        let mint = Keypair::new();
        let turbin3_prereq_program =Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = solana_sdk::system_program::id();

        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds,
            &turbin3_prereq_program);
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
        let (authority, _auth_bump) = Pubkey::find_program_address(
            &[b"collection", collection.as_ref()],
            &turbin3_prereq_program,
        );
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true), // user signer
            AccountMeta::new(prereq_pda, false), // PDA account
            AccountMeta::new(mint.pubkey(), true), // mint keypair
            AccountMeta::new(collection, false), // collection
            AccountMeta::new_readonly(authority, false), // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false), // system program
        ];

        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get latest blockhash");
        use solana_sdk::instruction::Instruction;
        let instruction = Instruction{
            program_id: turbin3_prereq_program,
            accounts,
            data
        };
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer,&mint],
            blockhash
        );
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        println!("Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",signature)   
    }
}
