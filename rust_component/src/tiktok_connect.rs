use futures::future::Either;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use tokio::signal;

use tokio::sync::mpsc::UnboundedSender;
use crate::pvz_scripts::change_sun_value;

pub struct SunChangeTask {
    process_id: u32,
    change_value_amount: i32,
}

pub async fn handle_request(req: Request<Body>, tx: UnboundedSender<SunChangeTask>, process_id: u32) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_string();
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let parts = path.split('/').collect::<Vec<&str>>();
    let webhook_id = parts.get(parts.len() - 2).unwrap_or(&""); // The webhook id is now second last
    let count: i32 = parts.last().unwrap_or(&"0").parse().unwrap_or(0);

    println!("Received Webhook {} Payload: {}", webhook_id, body_str);

    match *webhook_id {
        "like" => {
            let change_value = -(count / 3);
            tx.send(SunChangeTask { process_id, change_value_amount: change_value }).unwrap();
            println!("User has liked the livestream, count: {}", count);
        },
        "follow" => {
            tx.send(SunChangeTask { process_id, change_value_amount: -25 }).unwrap();
            println!("User has followed the livestream");
        },
        "share" => {
            tx.send(SunChangeTask { process_id, change_value_amount: -10 }).unwrap();
            println!("User has shared the livestream");
        },
        "gift" => {
            let change_value = -(count * 5);
            tx.send(SunChangeTask { process_id, change_value_amount: change_value }).unwrap();
            println!("User has gifted to the stream, count: {}", count);
        },
        "reset" => {
            tx.send(SunChangeTask { process_id, change_value_amount: -9990 }).unwrap();
            println!("User has RESET the SUN!!");
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
        .body(Body::from(""))
        .unwrap())
}


pub async fn run_server(process_id: u32) -> Result<(), hyper::Error>  {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<SunChangeTask>();

    tokio::spawn(async move {
        while let Some(task) = rx.recv().await {
            change_sun_value(task.process_id, task.change_value_amount);
        }
    });

    let make_svc = make_service_fn(move |_conn| {
        let tx = tx.clone();
        let func = service_fn(move |req| handle_request(req, tx.clone(), process_id));
        async move { Ok::<_, Infallible>(func) }
    });

    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server started on http://{}", addr);

    let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to install CTRL+C signal handler");
        tx_shutdown.send(()).unwrap();
    });

    match futures::future::select(Box::pin(server), rx_shutdown).await {
        Either::Left((server_result, _)) => return server_result,
        Either::Right((_, _server_future)) => return Ok(()),
    }
}
