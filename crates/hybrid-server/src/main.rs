use std::marker::PhantomPinned;
use proto::proto_files::greeter_server::{Greeter, GreeterServer};
use proto::proto_files::{HelloReply, HelloRequest};
use std::net::SocketAddr;
use std::task::{Context, Poll};
use tonic::{Request, Response, Status};
use tower::Service;

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

impl<ConnInfo, MakeWeb, Grpc> Service<ConnInfo> for HybridMakeService<MakeWeb, Grpc>
where
    MakeWeb: Service<ConnInfo>,
    Grpc: Clone,
{
    type Response = HybridService<MakeWeb::Response, Grpc>;
    type Error = MakeWeb::Error;
    type Future = HybridMakeServiceFuture<MakeWeb::Future, Grpc>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.make_web.poll_ready(cx)
    }

    fn call(&mut self, req: ConnInfo) -> Self::Future {
        HybridMakeServiceFuture {
            web_future: self.make_web.call(req),
            grpc: Some(self.grpc.clone()),
            _pin: PhantomPinned,
        }
    }
}


struct HybridService<Web, Grpc> {
    web: Web,
    grpc: Grpc,
}

struct HybridMakeServiceFuture<WebFuture, Grpc> {
    web_future: WebFuture,
    grpc: Option<Grpc>,
    _pin: PhantomPinned,
}

impl<WebFuture, Web, WebError, Grpc> Future for HybridMakeServiceFuture<WebFuture, Grpc>
where
    WebFuture: Future<Output = Result<Web, WebError>>,
{
    type Output = Result<HybridService<Web, Grpc>, WebError>;
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
