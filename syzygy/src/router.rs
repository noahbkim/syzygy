use std::pin::Pin;
use std::future::Future;
use super::{Request, Response};

pub type ResponseFuture = Pin<Box<dyn Future<Output = Response> + Send>>;
pub type Route = dyn FnOnce(Request) -> ResponseFuture;
pub type Path<'a> = &'a str;

pub trait Root: Send + Sync {
    fn route(&self, path: Path) -> Box<Route>;
}

pub trait Node: Send + Sync {
    fn route(&self, path: Path) -> Option<Box<Route>>;
}
