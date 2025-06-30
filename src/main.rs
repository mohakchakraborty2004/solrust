use axum::{
    routing::{get, post, Route},
    Router,
};



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
                      .route("/genKey", get(routes::gen_keypair));

  
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
    
}

async fn hello() -> &'static str {
    "Hello welcome to sol system.. go onto /hello/Your_Name"
}

