use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use tokio::sync::Mutex;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::pvz_scripts::change_sun_value;

// Shared state to manage the accumulation of changes and command execution state
struct SharedState {
    command_state: Mutex<CommandState>,
}

struct CommandState {
    is_running: bool,
    accumulated_change: i32,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            command_state: Mutex::new(CommandState {
                is_running: false,
                accumulated_change: 0,
            }),
        }
    }

    // Function to add change to the accumulator
    pub async fn add_to_accumulator(&self, amount: i32) {
        let mut state = self.command_state.lock().await;
        state.accumulated_change += amount;
    }

    // Function to potentially run the `change_sun_value` command if it's not already running
    pub async fn run_command_if_needed(&self, process_id: u32) {
        loop {
            let (should_run, accumulated_change) = {
                let mut state = self.command_state.lock().await;
                if state.is_running {
                    // If the command is already running, don't start another one.
                    break;
                }
    
                state.is_running = true;
                let accumulated_change = state.accumulated_change;
                state.accumulated_change = 0;
    
                // Indicate that we should run the command and with what value
                (true, accumulated_change)
            };
    
            if should_run {
                // Run the command in a blocking fashion
                tokio::task::spawn_blocking(move || {
                    change_sun_value(process_id, accumulated_change);
                }).await.expect("Failed to run change_sun_value");
    
                // Once the task is done, check if we need to run it again
                let mut state = self.command_state.lock().await;
                state.is_running = false;
                if state.accumulated_change == 0 {
                    // If no changes were accumulated during the command execution, we're done
                    break;
                }
                // If changes were accumulated, the loop will continue and run the command again
            } else {
                // If we determined not to run the command, exit the loop
                break;
            }
        }
    }
    
}

async fn handle_request(req: Request<Body>, shared_state: Arc<SharedState>, process_id: u32) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_string();
    let parts = path.split('/').collect::<Vec<&str>>();
    let webhook_id = parts.get(parts.len() - 2).unwrap_or(&"");
    let count: i32 = parts.last().unwrap_or(&"0").parse().unwrap_or(0);

    match *webhook_id {
        "like" => {
            let change_value = -(count / 3);
            shared_state.add_to_accumulator(change_value).await;
        shared_state.run_command_if_needed(process_id).await;
        },
        "follow" => {
            let change_value = -25;
            shared_state.add_to_accumulator(change_value).await;
            shared_state.run_command_if_needed(process_id).await;
        },
        "gift" => {
            let change_value = -(count * 5);
            shared_state.add_to_accumulator(change_value).await;
            shared_state.run_command_if_needed(process_id).await;
        },
        "reset" => {
            let change_value = -9990;
            shared_state.add_to_accumulator(change_value).await;
            shared_state.run_command_if_needed(process_id).await;
        },
        _ => {
            println!("Unknown webhook ID: {}", webhook_id);
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Bad Request"))
                .unwrap());
        }
    }
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Command processed"))
        .unwrap())
}


pub async fn run_server(process_id: u32) -> Result<(), hyper::Error>  {
    let shared_state = Arc::new(SharedState::new());

    let make_svc = make_service_fn(move |_conn| {
        let shared_state = shared_state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, shared_state.clone(), process_id)
            }))
        }
    });

    let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running at http://{}", addr);

    server.await
}