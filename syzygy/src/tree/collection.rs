use super::view::View;
use super::TreeNode;
use crate::parts::Parts;
use crate::router::{Path, Route};
use crate::{Request, Response};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ItemView<S>: Send + Sync + 'static
where
    S: ?Sized + Send + Sync + 'static,
{
    async fn handle(self: Arc<Self>, request: Request, id: &str, state: Box<S>) -> Response;

    fn prepare(self: Arc<Self>, id: &str, state: Box<S>) -> Box<Route> {
        Box::new(move |request: Request| self.handle(request, id, state))
    }
}

pub trait CollectionView<S>: View<S> + ItemView<S>
where
    S: ?Sized + Send + Sync + 'static,
{
}

pub trait CollectionViewTransition<S>
where
    S: ?Sized,
{
    fn from(s: Box<S>, i: &str, t: &str) -> Box<Self>;
}

pub trait CollectionViewParent<S>
where
    S: ?Sized + Send + Sync + 'static,
{
    type T: CollectionViewTransition<S> + ?Sized + Send + Sync + 'static;
    fn child(&self, part: &str) -> Option<Box<dyn TreeNode<Self::T>>>;
}

pub trait CollectionViewRouter<S>
where
    S: ?Sized + Send + Sync + 'static,
{
    fn view(&self) -> Arc<dyn CollectionView<S>>;
}

impl<S, V> TreeNode<S> for V
where
    S: ?Sized + Send + Sync + 'static,
    V: CollectionViewRouter<S> + CollectionViewParent<S> + Send + Sync + 'static,
{
    default fn route<'p>(&self, path: Path<'p>, state: Box<S>) -> Option<Box<Route>> {
        match Parts::from(path) {
            Parts::Nil => Some(CollectionView::prepare(self.view(), state)),
            Parts::Cons(id, rest) => match Parts::from(rest) {
                Parts::Nil => Some(ItemView::prepare(self.view(), id, state)),
                Parts::Cons(part, rest) => match self.child(part) {
                    Some(child) => child.route(rest, V::T::from(state, id, part)),
                    None => None,
                },
            },
        }
    }
}

impl<S, V> TreeNode<S> for V
where
    S: ?Sized + Send + Sync + 'static,
    V: CollectionViewRouter<S> + Send + Sync + 'static,
{
    default fn route(&self, path: Path, state: Box<S>) -> Option<Box<Route>> {
        match Parts::from(path) {
            Parts::Nil => Some(CollectionView::prepare(self.view(), state)),
            Parts::Cons(id, rest) => match Parts::from(rest) {
                Parts::Nil => Some(ItemView::prepare(self.view(), id, state)),
                _ => None,
            },
        }
    }
}
