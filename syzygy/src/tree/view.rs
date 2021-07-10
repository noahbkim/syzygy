use std::sync::Arc;
use async_trait::async_trait;
use crate::{Request, Response};
use crate::router::{Route, Path};
use crate::parts::Parts;
use super::TreeNode;

pub trait Combines<S, T> {
    fn from(s: S, t: T) -> Box<Self>;
}

#[async_trait]
pub trait View<S>: Send + Sync + 'static
where
    S: ?Sized + Send + Sync + 'static
{
    async fn handle(self: Arc<Self>, request: Request, state: Box<S>) -> Response;

    fn prepare(self: Arc<Self>, state: Box<S>) -> Box<Route> {
        Box::new(move |request: Request| self.handle(request, state))
    }
}

pub trait ViewRouter<S>
where
    S: ?Sized + Send + Sync + 'static
{
    fn view(&self) -> Arc<dyn View<S>>;
}

pub trait ViewParent<'a, S>
where
    S: ?Sized + Send + Sync + 'static,
{
    type T: Combines<Box<S>, &'a str> + ?Sized + Send + Sync + 'static;

    fn child(&self, part: &str) -> Option<Box<dyn TreeNode<Self::T>>>;
}

impl<'a, S, V> TreeNode<S> for V
where
    S: ?Sized + Send + Sync + 'static,
    V: ViewRouter<S> + ViewParent<'a, S> + Send + Sync + 'static,
{
    default fn route(&self, path: Path<'a>, state: Box<S>) -> Option<Box<Route>> {
        match Parts::from(path) {
            Parts::Nil => Some(self.view().prepare(state)),
            Parts::Cons(part, rest) => match self.child(part) {
                Some(child) => child.route(rest, V::T::from(state, part)),
                None => None,
            },
        }
    }
}

impl<S, V> TreeNode<S> for V
where
    S: ?Sized + Send + Sync + 'static,
    V: ViewRouter<S> + Send + Sync + 'static,
{
    default fn route(&self, path: Path, state: Box<S>) -> Option<Box<Route>> {
        match Parts::from(path) {
            Parts::Nil => Some(self.view().prepare(state)),
            _ => None,
        }
    }
}
