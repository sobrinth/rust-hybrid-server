use std::net::SocketAddr;
use tonic::{Request, Response, Status};
use proto::proto_files::greeter_server::{Greeter, GreeterServer};
use proto::proto_files::{HelloReply, HelloRequest};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let addr = SocketAddr::from(([0, 0, 0, 0], 6003));

    let http_service = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, HTTP" }))
        .into_make_service();

    let grpc_service = tonic::service::Routes::builder()
        .add_service(GreeterServer::new(MyGreeter::default()))
        .routes();

    let hybrid = hybrid(http_service, grpc_service);

    axum_server::bind(addr).serve(hybrid).await.unwrap()
}

fn hybrid<MakeWeb, Grpc>(make_web: MakeWeb, grpc: Grpc) -> HybridMakeService<MakeWeb, Grpc> {
    HybridMakeService { make_web, grpc }
}

struct HybridMakeService<MakeWeb, Grpc> {
    make_web: MakeWeb,
    grpc: Grpc,
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        todo!()
    }
}
