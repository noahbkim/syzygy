use super::TreeNode;
use crate::parts::Parts;
use crate::router::{Path, Route};
use crate::{Request, Response};
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

#[async_trait]
pub trait CollectionView<S>: Send + Sync + 'static
where
    S: ?Sized + Send + Sync + 'static,
{
    async fn handle(self: Arc<Self>, request: Request, state: Box<S>) -> Response;

    fn prepare(self: Arc<Self>, state: Box<S>) -> Box<Route> {
        Box::new(move |request: Request| self.handle(request, state))
    }
}

#[async_trait]
pub trait ItemView<S>: Send + Sync + 'static
where
    S: ?Sized + Send + Sync + 'static,
{
    async fn handle(self: Arc<Self>, request: Request, id: String, state: Box<S>) -> Response;

    fn prepare(self: Arc<Self>, id: String, state: Box<S>) -> Box<Route> {
        Box::new(move |request: Request| self.handle(request, id, state))
    }
}

pub trait CollectionViewTransition<S>
where
    S: ?Sized,
{
    fn from(state: Box<S>, id: &str, part: &str) -> Box<Self>;
}

pub trait ParentCollectionViewRouter<S>
where
    S: ?Sized + Send + Sync + 'static,
{
    type T: CollectionViewTransition<S> + ?Sized + Send + Sync + 'static;

    fn child(&self, part: &str) -> Option<&Box<dyn TreeNode<Self::T>>>;
}

pub trait CollectionViewRouter<S>
where
    S: ?Sized + Send + Sync + 'static,
{
    fn item(&self) -> Arc<dyn ItemView<S>>;
    fn collection(&self) -> Arc<dyn CollectionView<S>>;
}

// impl<S, R> TreeNode<S> for R
// where
//     S: ?Sized + Send + Sync + 'static,
//     R: CollectionViewRouter<S> + CollectionViewRouterParent<S> + Send + Sync + 'static,
// {
//     default fn route<'p>(&self, path: Path<'p>, state: Box<S>) -> Option<Box<Route>> {
//         match Parts::from(path) {
//             Parts::Nil => Some(self.collection().prepare(state)),
//             Parts::Cons(id, rest) => match Parts::from(rest) {
//                 Parts::Nil => Some(self.item().prepare(id.into(), state)),
//                 Parts::Cons(part, rest) => match self.child(part) {
//                     Some(child) => child.route(rest, R::T::from(state, id, part)),
//                     None => None,
//                 },
//             },
//         }
//     }
// }

// impl<S, R> TreeNode<S> for R
// where
//     S: ?Sized + Send + Sync + 'static,
//     R: CollectionViewRouter<S> + Send + Sync + 'static,
// {
//     default fn route(&self, path: Path, state: Box<S>) -> Option<Box<Route>> {
//         match Parts::from(path) {
//             Parts::Nil => Some(self.collection().prepare(state)),
//             Parts::Cons(id, rest) => match Parts::from(rest) {
//                 Parts::Nil => Some(self.item().prepare(id.into(), state)),
//                 _ => None,
//             },
//         }
//     }
// }

pub struct DefaultParentCollectionViewRouter<S, T>
where
    S: ?Sized + Send + Sync + 'static,
    T: CollectionViewTransition<S> + ?Sized + Send + Sync + 'static,
{
    item: Arc<dyn ItemView<S>>,
    collection: Arc<dyn CollectionView<S>>,
    children: HashMap<&'static str, Box<dyn TreeNode<T>>>,
}

impl<S, T> CollectionViewRouter<S> for DefaultParentCollectionViewRouter<S, T>
where
    S: ?Sized + Send + Sync + 'static,
    T: CollectionViewTransition<S> + ?Sized + Send + Sync + 'static,
{
    fn item(&self) -> Arc<dyn ItemView<S>> {
        self.item.clone()
    }

    fn collection(&self) -> Arc<dyn CollectionView<S>> {
        self.collection.clone()
    }
}

impl<S, T> ParentCollectionViewRouter<S> for DefaultParentCollectionViewRouter<S, T>
where
    S: ?Sized + Send + Sync + 'static,
    T: CollectionViewTransition<S> + ?Sized + Send + Sync + 'static,
{
    type T = T;

    fn child(&self, part: &str) -> Option<&Box<dyn TreeNode<Self::T>>> {
        self.children.get(part)
    }
}

impl<S, T> TreeNode<S> for DefaultParentCollectionViewRouter<S, T>
where
    S: ?Sized + Send + Sync + 'static,
    T: CollectionViewTransition<S> + ?Sized + Send + Sync + 'static,
{
    fn route<'p>(&self, path: Path<'p>, state: Box<S>) -> Option<Box<Route>> {
        match Parts::from(path) {
            Parts::Nil => Some(self.collection().prepare(state)),
            Parts::Cons(id, rest) => match Parts::from(rest) {
                Parts::Nil => Some(self.item().prepare(id.into(), state)),
                Parts::Cons(part, rest) => match self.child(part) {
                    Some(child) => child.route(rest, T::from(state, id, part)),
                    None => None,
                },
            },
        }
    }
}
