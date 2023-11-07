mod tiktok_connect;
mod mem_utils;
mod pvz_scripts;
mod addresses;

use mem_utils::get_pid_by_name;
// use pvz_scripts::change_sun_value;
use tiktok_connect::run_server;

use tokio::signal;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[tokio::main]
async fn main() {
    let game_name = "PlantsVsZombies.exe";
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();

    // Spawn a task to listen for ctrl+c
    tokio::spawn(async move {
        if signal::ctrl_c().await.is_ok() {
            r.store(false, Ordering::SeqCst);
        }
    });

    let process_id = match get_pid_by_name(game_name) {
        Some(pid) => pid,
        None => {
            eprintln!("Failed to get PID for {}", game_name);
            return;
        }
    };

    // Run the server and periodically check if it should be shut down
    tokio::select! {
        _ = run_server(process_id) => {
            eprintln!("Server finished unexpectedly");
        }
        _ = tokio::task::spawn(async move {
            while running.load(Ordering::SeqCst) {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }) => {
            println!("Shutdown signal received, terminating server.");
        }
    };

    // Perform any cleanup here if necessary
    println!("Server is shutting down.");
}
