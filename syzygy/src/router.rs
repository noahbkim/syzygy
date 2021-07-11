use super::{Request, Response};
use std::future::Future;
use std::pin::Pin;

pub type ResponseFuture = Pin<Box<dyn Future<Output = Response> + Send>>;
pub type Route = dyn FnOnce(Request) -> ResponseFuture;
pub type Path<'a> = &'a str;

pub trait Root: Send + Sync {
    fn route(&self, path: Path) -> Box<Route>;
}

pub trait Node: Send + Sync {
    fn route(&self, path: Path) -> Option<Box<Route>>;
}
