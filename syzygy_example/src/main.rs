use std::sync::Arc;

use async_trait::async_trait;
use hyper::server::Server;

use syzygy::{Request, Response};
use syzygy::core::Dispatcher;
use syzygy::router::{ViewRouter, Router};
use syzygy::view::View;

type Root = dyn Router<()>;

struct HelloView {}

#[async_trait]
impl View<()> for HelloView {
    async fn handle(&self, _request: Request, _state: Box<()>) -> Response {
        Response::new("hello, world!".into())
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let view = HelloView {};
    let router: Arc<Root> = Arc::new(ViewRouter::<()>::new(Arc::new(view)));
    let dispatcher = Dispatcher::new(router);
    let address = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&address).serve(dispatcher);

    println!("Listening on http://{}", address);
    server.await?;

    Ok(())
}
