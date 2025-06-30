use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Deserialize)]
pub struct airdropRequest {
    pub pubkey : String,
}

#[derive(Debug, Serialize)]
pub struct KeyPairResponse {
   pub  pubkey : String,
   pub  pvtKey : Vec<u8>
}


#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub amount : u64
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success : bool,
    pub data : Option<T>
}

#[derive(Debug, Clone, Deserialize)]
pub struct mintInit {
    pub mintAuthority : String,
    pub mint : String,
    pub decimals : u8
}

#[derive(Debug, Clone, Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignRequest {
    pub message: String,
    pub secret: String
}

#[derive(Debug, Deserialize)]
pub struct SolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}