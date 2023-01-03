use tide::sse;
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::{Response, StatusCode};

use std::time::Duration;

use async_std::task;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTION".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    //femme::start();
    let mut app = tide::new();
    app.with(cors);
    app.at("/sse").get(sse::endpoint(|_req, sender| async move {
        loop {
            sender.send("fruit", "banana", None).await?;
            task::sleep(Duration::from_secs(5)).await;
            sender.send("fruit", "apple", None).await?;
        }
        Response::new(StatusCode::Ok);
    }));
    app.listen("localhost:8080").await?;
    Ok(())
}