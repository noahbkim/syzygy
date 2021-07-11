use super::TreeNode;
use crate::parts::Parts;
use crate::router::{Path, Route};
use crate::{Request, Response};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait View<S>: Send + Sync + 'static
where
    S: ?Sized + Send + Sync + 'static,
{
    async fn handle(self: Arc<Self>, request: Request, state: Box<S>) -> Response;

    fn prepare(self: Arc<Self>, state: Box<S>) -> Box<Route> {
        Box::new(move |request: Request| self.handle(request, state))
    }
}

pub trait ViewTransition<S>
where
    S: ?Sized,
{
    fn from(s: Box<S>, t: &str) -> Box<Self>;
}

pub trait ViewRouterParent<S>
where
    S: ?Sized + Send + Sync + 'static,
{
    type T: ViewTransition<S> + ?Sized + Send + Sync + 'static;
    fn child(&self, part: &str) -> Option<&Box<dyn TreeNode<Self::T>>>;
}

pub trait ViewRouter<S>
where
    S: ?Sized + Send + Sync + 'static,
{
    fn view(&self) -> Arc<dyn View<S>>;
}

impl<S, V> TreeNode<S> for V
where
    S: ?Sized + Send + Sync + 'static,
    V: ViewRouter<S> + ViewRouterParent<S> + Send + Sync + 'static,
{
    default fn route<'p>(&self, path: Path<'p>, state: Box<S>) -> Option<Box<Route>> {
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
