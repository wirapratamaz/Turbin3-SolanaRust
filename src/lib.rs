mod programs;

#[cfg(test)]
mod tests {
    use solana_sdk;
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file}, 
        pubkey::Pubkey,
        transaction::Transaction,
        hash::{hash},
        message::Message
    };
    use solana_client::rpc_client::RpcClient;
    use solana_program::{
        pubkey::Pubkey as ProgramPubkey,
        system_instruction::transfer,
        system_program,
    };
    use bs58;
    use std::io::{self, BufRead};
    use std::str::FromStr;
    use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs};
    
    const RPC_URL: &str = "https://api.devnet.solana.com";
    
    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }
    
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }
    
    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
    
    #[test]
    fn airdop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);
        
        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };
    }
    
    #[test]
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // With the imported Keypair, we can sign a new message.
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        
        // After that we can verify the signature, using the default implementation
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        
        // Define our Turbin3 public key
        let to_pubkey = ProgramPubkey::from_str("5QpPAVrQE5aZzd9sS5pWMoXuqngGqrWUFNLcWqDCmhzT").unwrap();
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
            
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                1_000_000
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );
        
        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
            
        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
    
    #[test]
    fn empty_wallet() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Define our Turbin3 public key
        let to_pubkey = ProgramPubkey::from_str("5QpPAVrQE5aZzd9sS5pWMoXuqngGqrWUFNLcWqDCmhzT").unwrap();
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
            
        // Get balance of dev wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
            
        println!("Current balance: {} lamports", balance);
            
        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance,
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );
        
        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
            
        println!("Transaction fee: {} lamports", fee);
        
        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance - fee,
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );
        
        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
            
        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
        
        // Verify the new balance
        let new_balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
            
        println!("New balance: {} lamports", new_balance);
    }
    
    #[test]
    fn complete_enrollment() {
        println!("=====================================================");
        println!("Submitting Turbin3 Enrollment");
        println!("=====================================================");
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Import our dev wallet which now has 2 SOL
        let signer = read_keypair_file("dev-wallet.json").expect("Couldn't find dev wallet file");
        println!("Signer Public Key: {}", signer.pubkey());
        
        // Check our balance first
        let balance = rpc_client
            .get_balance(&signer.pubkey())
            .expect("Failed to get balance");
        println!("Current balance: {} lamports", balance);
        
        // Create a PDA for our prereq account
        println!("\nCreating PDA with inputs:");
        println!("  - Seed: prereq");
        println!("  - Public Key: {}", signer.pubkey());
        
        let prereq = Turbin3PrereqProgram::derive_program_address(
            &[b"prereq", signer.pubkey().to_bytes().as_ref()]
        );
        println!("PDA derived: {}", prereq);
        
        // Print program details
        println!("\nProgram Details:");
        println!("  - Program ID: {}", Turbin3PrereqProgram::id());
        println!("  - Github Username: wirapratamaz");
        
        // Define our instruction data with your github username
        let args = CompleteArgs {
            github: b"wirapratamaz".to_vec()
        };
        
        // Get recent blockhash
        println!("\nGetting recent blockhash...");
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        println!("Building and sending transaction...");
        // Now we can invoke the "complete" function
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );
        
        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        
        // Print our transaction out
        println!("\nSuccess! 🎉");
        println!("Your enrollment is now recorded on-chain!");
        println!("Transaction: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
        println!("=====================================================");
    }
    
    #[test]
    fn fund_turbin3_wallet() {
        // Import our Turbin3 keypair
        let keypair = read_keypair_file("Turbin3_wallet.json").expect("Couldn't find wallet file");
        
        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);
        
        // We're going to claim devnet SOL tokens
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };
    }
    
    #[test]
    fn check_balance() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        println!("Dev Wallet Public Key: {}", keypair.pubkey());
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Get balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
            
        println!("Balance: {} lamports", balance);
    }

    #[test]
    fn fund_enrollment_wallet() {
        // Import our enrollment keypair
        let keypair = read_keypair_file("enrollment-wallet.json").expect("Couldn't find wallet file");
        println!("Enrollment Wallet Public Key: {}", keypair.pubkey());
        
        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);
        
        // We're going to claim devnet SOL tokens
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
                
                // Wait a moment for the transaction to be confirmed
                std::thread::sleep(std::time::Duration::from_secs(2));
                
                // Check balance
                let balance = client
                    .get_balance(&keypair.pubkey())
                    .expect("Failed to get balance");
                    
                println!("New Balance: {} lamports", balance);
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };
    }
}
