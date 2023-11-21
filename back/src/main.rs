use std::error::Error;

use tokio;
use dotenv;
use spotify_rs::{ClientCredsFlow, ClientCredsClient};

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

    Ok(())
}