use std::str::FromStr;

use axum::{
    response::Json,
};
use axum::extract::{Path, Query};
use serde_json::{Value, json};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::{self, Pubkey}, signature::{Keypair, Signer}, system_instruction, transaction::Transaction
};

use crate::types::{airdropRequest, BalanceResponse, KeyPairResponse};


pub async  fn hello_name(Path(name): Path<String>) -> Json<Value> {
   Json(json!({
    "name" : name,
    "generateKey" : "/genKey",
    "airdrop_1_sol" : "/airdrop/Pub_key",
    "check_balance" : "/getBalance/Pub_key"
}))
}

pub async fn gen_keypair() -> Json<KeyPairResponse> {
    let keypair = Keypair::new();
    return Json(KeyPairResponse {
        pubkey : keypair.pubkey().to_string(),
        pvtKey : keypair.secret().to_bytes().to_vec() 
    })
   }

pub async fn air_drop(Path(airdrop): Path<String>) -> String {
    let client = RpcClient::new_with_commitment("https://api.devnet.solana.com", CommitmentConfig::confirmed());

    let pubkey = match Pubkey::from_str(&airdrop) {
        Ok(p) => p,
       Err(_e) => return "Invalid pubkey".into()
    };

    match client.request_airdrop(&pubkey, 1_000_000_000) {
        Ok(sig) => format!("Airdrop requested: {}", sig),
        Err(e) => format!("Airdrop failed: {}", e)
    }

}   

pub async fn check_balance(Path(pubkey) : Path<String>) -> Json<BalanceResponse> {
     let client = RpcClient::new_with_commitment("https://api.devnet.solana.com", CommitmentConfig::confirmed());
     
      let pubkey =  Pubkey::from_str(&pubkey).unwrap_or(Pubkey::default());

      let balance = client.get_balance(&pubkey).unwrap_or(0);

      return Json(BalanceResponse { 
        amount: balance }
    )

}