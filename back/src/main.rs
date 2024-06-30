use std::error::Error;

use axum::{routing::get, Router};
use dotenv;
use spotify_rs::{ClientCredsClient, ClientCredsFlow};
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv::dotenv()?;

    let auth_flow = ClientCredsFlow::new(
        dotenv::var("SPOTIFY_CLIENT_ID")?,
        dotenv::var("SPOTIFY_CLIENT_SECRET")?,
    );

    // Create and authenticate the client
    let mut spotify = ClientCredsClient::authenticate(auth_flow).await?;

    println!("{:?}", spotify.access_token());
    println!("{:#?}", spotify);

    let track = spotify.track("0vSzHgep92QhPeDHrUZand").get().await?;

    println!("{:?}", track);

    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|err| err.into())
}

async fn root() -> &'static str {
    include_str!("../../front/index.html")
}
