mod hybrid;

use proto::proto_files::greeter_server::{Greeter, GreeterServer};
use proto::proto_files::{HelloReply, HelloRequest};
use std::net::SocketAddr;
use std::time::Duration;
use tonic::{Request, Response, Status};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 6003));
    println!("Listening on http://{}", addr);

    let http_service = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                println!("Handling HTTP request");
                "Hello, HTTP"
            }),
        )
        .into_make_service(); // TODO: Investigate `into_service()`

    let mut grpc_builder = tonic::service::Routes::builder();
    grpc_builder.add_service(GreeterServer::new(MyGreeter::default()));
    let grpc_service = grpc_builder.routes();

    let hybrid_server = hybrid::combine(http_service, grpc_service);

    let configured_service = ServiceBuilder::new()
        .rate_limit(5, Duration::from_secs(5))
        .buffer(5)
        .concurrency_limit(5)
        .timeout(Duration::from_secs(10))
        .service(hybrid_server);

    axum_server::bind(addr)
        .serve(configured_service)
        .await
        .unwrap()
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Received request: {:?}", request);
        let name = request.into_inner().name;
        let reply = HelloReply {
            message: format!("Hello, {name}!"),
        };
        Ok(Response::new(reply))
    }
}
