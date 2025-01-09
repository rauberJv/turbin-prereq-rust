fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use solana_client::rpc_client::RpcClient;
    use solana_program::pubkey::Pubkey;
    use solana_program::system_instruction::transfer;
    use solana_sdk::{
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
        message::Message,
    };
    use std::str::FromStr;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("Your new keypair: {}", kp.pubkey().to_string());
        println!("Save this in a wallet.json file: {:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("./dev-wallet.json").expect("Could not find the wallet, make sure you generated the dev-wallet.json file with correct wallet");

        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success, check your TX here: {}", s.to_string());
            }
            Err(e) => {
                println!("Oops, something went wrong: {}", e.to_string());
            }
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Error if not find");

        let to_pubkey = Pubkey::from_str("CJM2LnrH6byLP2ExkwvZRgTyhAM1xaJffHRS5Q3A68S").unwrap();
        
        const RPC_URL: &str = "https://api.devnet.solana.com";

        let rpc_client = RpcClient::new(RPC_URL);

        let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance.");
        
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );

        let fee = rpc_client.get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        println!("Success, check your tx here: {}", signature.to_string());
        //Todo: Finish transfer sol and add verification of amount to empty local wallet
    }
}
