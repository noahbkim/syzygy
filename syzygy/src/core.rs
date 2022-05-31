use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper::service::Service;

use crate::{Request, Response, Error};
use crate::router::Router;

type Root = dyn Router<()>;

pub struct Handler {
    router: Arc<Root>,
}

impl Handler {
    fn new(router: Arc<Root>) -> Self {
        Self { router }
    }
}

impl Service<Request> for Handler {
    type Response = Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let router = self.router.clone();
        Box::pin(async move {
            let path = request.uri().path().to_string();
            let response = router.route(&path, request, Box::new(())).await;
            Ok(response)
        })
    }
}

pub struct Dispatcher {
    router: Arc<Root>,
}

impl Dispatcher {
    pub fn new(router: Arc<Root>) -> Self {
        Self { router }
    }
}

impl<T> Service<T> for Dispatcher {
    type Response = Handler;
    type Error = std::io::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        ready(Ok(Handler::new(self.router.clone())))
    }
}
