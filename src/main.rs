use axum::{
    routing::{get, post, Route},
    Router,
};

//this is practiced code, saved my time to skip setup, Did take help from gpt accepting it.

mod routes;
mod types;

use routes::hello_name;
#[tokio::main]
async fn main() {
    let app = Router::new()
                      .route("/", get( hello))
                      .route("/hello/:name", get(hello_name))
                      .route("/getBalance/:pubkey", get(routes::check_balance))
                      .route("/airdrop/:airdrop", get(routes::air_drop))
                      .route("/keypair", post(routes::gen_keypair))
                      .route("/token/create", post(routes::create_token))
                      .route("/token/mint", post(routes::mint_token))
                      .route("/send/sol", post(routes::create_sol_transfer_instruction));

  
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
    
}

async fn hello() -> &'static str {
    "Hello welcome to sol system.. go onto /hello/Your_Name"
}

