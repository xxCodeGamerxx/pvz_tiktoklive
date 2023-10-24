use futures::future::Either;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use tokio::signal;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_string();
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let webhook_id = path.split('/').last().unwrap_or("");

    println!("Received Webhook {} Payload: {}", webhook_id, body_str);

    match webhook_id {
        "follow" => println!("User has followed the livestream"),
        "like" => println!("User has liked the livestream"),
        "share" => println!("User has shared the livestream"),
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

pub async fn run_server() -> Result<(), hyper::Error>  {
    let make_svc = make_service_fn(|_conn| {
        let func = service_fn(handle_request);
        async move { Ok::<_, Infallible>(func) }
    });

    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server started on http://{}", addr);

    // Handling graceful shutdown using signal.
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to install CTRL+C signal handler");
        tx.send(()).unwrap();
    });

    match futures::future::select(Box::pin(server), rx).await {
        Either::Left((server_result, _)) => {
            // The server completed (with an error or otherwise) before the signal was received.
            return server_result;
        },
        Either::Right((_, _server_future)) => {
            // The signal was received. You can now decide what to do next.
            // If you just want the server to shut down gracefully, you can simply return Ok(()).
            return Ok(());
        },
    }
}
