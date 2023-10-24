mod tiktok_connect;

use tiktok_connect::run_server;

#[tokio::main]
async fn main() {
    if let Err(e) = run_server().await {
        eprintln!("Server error: {}", e);
    }
}

