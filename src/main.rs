use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let router: Router = Router::new()
        .route("/", get(root_get));

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let address = server.local_addr();
    println!("Listening on {address}");

    server.await.unwrap();

    println!("Hello, world!");
}

async fn root_get() -> &'static str {
    "Hi from Axum!"
}
