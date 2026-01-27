use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let addr = SocketAddr::from(([0, 0, 0, 0], 6003));

    let http_service = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, HTTP" }))
        .into_make_service();

    axum_server::bind(addr).serve(http_service)
        .await.unwrap()
}
