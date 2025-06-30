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