use std::str::FromStr;
use axum::http::Response;
use axum::response::IntoResponse;
use solana_sdk::instruction::Instruction;
use spl_token::ID as TOKEN_PROGRAM_ID;
use axum::{
    response::Json,
    Json as ReqJson
};
use axum::extract::{Path, Query};
use serde_json::{Value, json};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::{self, Pubkey}, signature::{Keypair, Signer}, system_instruction, transaction::Transaction
};
use spl_token::instruction::{initialize_mint, mint_to};
use base64::{engine::general_purpose, Engine as _};

use crate::types::{airdropRequest, mintInit, ApiResponse, BalanceResponse, KeyPairResponse, MintTokenRequest, SignRequest, SolRequest, TokenRequest};


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


pub async fn create_token(ReqJson(mint): ReqJson<mintInit>) -> impl IntoResponse {
  

    let mint_pubkey = Pubkey::from_str(&mint.mint).unwrap_or_default();
    let authority_pubkey = Pubkey::from_str(&mint.mintAuthority).unwrap_or_default();

   

     match initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint_pubkey,
        &authority_pubkey,
        None,
        mint.decimals,
    ) {
        Ok(ix) => {
            // took help
            let accounts = ix.accounts.iter().map(|meta| {
                format!(
                    "{{ pubkey: \"{}\", signer: {}, writable: {} }}",
                    meta.pubkey, meta.is_signer, meta.is_writable
                )
            }).collect::<Vec<_>>();

            let response = serde_json::json!({
                "success": true,
                "data": {
                    "program_id": ix.program_id.to_string(),
                    "accounts": accounts,
                    "instruction_data": general_purpose::STANDARD.encode(&ix.data)
                }
            });

            Json(ApiResponse {
                success : true,
                data : Some(response)
            })
        }

        Err(e) => Json(ApiResponse{
           success : false,
           data : None
        }),
    }
    
   
}

pub async fn mint_token(ReqJson(req): Json<MintTokenRequest>) -> impl IntoResponse {
    let mint = Pubkey::from_str(&req.mint).unwrap();
    let dest = Pubkey::from_str(&req.destination).unwrap();
    let authority = Pubkey::from_str(&req.authority).unwrap();

    match mint_to(
        &TOKEN_PROGRAM_ID,
        &mint,
        &dest,
        &authority,
        &[],
        req.amount,
    ) {
        Ok(ix) => {
            let accounts = ix.accounts.iter().map(|a| {
                serde_json::json!({
                    "pubkey": a.pubkey.to_string(),
                    "is_signer": a.is_signer,
                    "is_writable": a.is_writable
                })
            }).collect::<Vec<_>>();

            Json(serde_json::json!({
                "success": true,
                "data": {
                    "program_id": ix.program_id.to_string(),
                    "accounts": accounts,
                    "instruction_data": general_purpose::STANDARD.encode(&ix.data)
                }
            }))
        },
        Err(e) => Json(serde_json::json!({
            "success": false,
            "message": format!("Failed to create mint_to instruction: {}", e)
        }))
    }
}



pub async fn create_sol_transfer_instruction(ReqJson(payload): Json<SolRequest>) -> Json<ApiResponse<serde_json::Value>> {
    let from_pubkey = match payload.from.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Json(ApiResponse {
                success: false,
                data: Some(json!({ "error": "Invalid sender address" })),
            });
        }
    };

    let to_pubkey = match payload.to.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Json(ApiResponse {
                success: false,
                data: Some(json!({ "error": "Invalid recipient address" })),
            });
        }
    };

    let instruction: Instruction =
        system_instruction::transfer(&from_pubkey, &to_pubkey, payload.lamports);

    let instruction_json = json!({
        "program_id": instruction.program_id.to_string(),
        "accounts": instruction.accounts.iter().map(|acc| {
            json!({
                "pubkey": acc.pubkey.to_string(),
                "is_signer": acc.is_signer,
                "is_writable": acc.is_writable,
            })
        }).collect::<Vec<_>>(),
        "data": base64::encode(instruction.data),
    });

    Json(ApiResponse {
        success: true,
        data: Some(instruction_json),
    })
}

