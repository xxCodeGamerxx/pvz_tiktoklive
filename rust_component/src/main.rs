mod tiktok_connect;
mod mem_utils;
mod pvz_scripts;
mod addresses;

use mem_utils::get_pid_by_name;
// use pvz_scripts::change_sun_value;
use tiktok_connect::run_server;

#[tokio::main]
async fn main() {
    let game_name = "PlantsVsZombies.exe";

    let process_id = match get_pid_by_name(game_name) {
        Some(pid) => pid,
        None => {
            eprintln!("Failed to get PID for {}", game_name);
            return;
        }
    };

    if let Err(e) = run_server(process_id).await {
        eprintln!("Server error: {}", e);
    }
}
