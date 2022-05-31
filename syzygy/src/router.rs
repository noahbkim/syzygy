use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::path::Parts;
use crate::view::{CollectionView, ItemView, View};
use crate::{Request, Response};

#[async_trait]
pub trait Router<S>: Send + Sync
where
    S: ?Sized + Send + Sync,
{
    async fn route(&self, path: &str, request: Request, state: Box<S>) -> Response;
}

pub trait ViewChild<S>
where
    S: ?Sized,
{
    fn from(state: Box<S>, part: &str) -> Box<Self>;
}

impl<S> ViewChild<S> for S
where
    S: ?Sized,
{
    fn from(state: Box<S>, _: &str) -> Box<Self> {
        state
    }
}

pub struct ViewRouter<S, T = S>
where
    S: ?Sized + Send + Sync,
    T: ?Sized + Send + Sync + ViewChild<S>,
{
    view: Arc<dyn View<S>>,
    children: HashMap<&'static str, Box<dyn Router<T>>>,
}

impl<S, T> ViewRouter<S, T>
where
    S: ?Sized + Send + Sync,
    T: ?Sized + Send + Sync + ViewChild<S>,
{
    pub fn new(view: Arc<dyn View<S>>) -> Self {
        Self {
            view,
            children: HashMap::new(),
        }
    }
}

#[async_trait]
impl<S, T> Router<S> for ViewRouter<S, T>
where
    S: ?Sized + Send + Sync,
    T: ?Sized + Send + Sync + ViewChild<S>,
{
    async fn route(&self, path: &str, request: Request, state: Box<S>) -> Response {
        match Parts::split(path) {
            Parts::Nil => self.view.handle(request, state).await,
            Parts::Cons(part, rest) => match self.children.get(part) {
                Some(child) => child.route(rest, request, T::from(state, part)).await,
                None => Response::new("not found!".into()),
            },
        }
    }
}

pub trait CollectionChild<S>
where
    S: ?Sized,
{
    fn from(state: Box<S>, id: &str, part: &str) -> Box<Self>;
}

impl<S> CollectionChild<S> for S
where
    S: ?Sized,
{
    fn from(state: Box<S>, _id: &str, _part: &str) -> Box<Self> {
        state
    }
}

pub struct CollectionRouter<S, T = S>
where
    S: ?Sized + Send + Sync,
    T: ?Sized + Send + Sync + CollectionChild<S>,
{
    item: Arc<dyn ItemView<S>>,
    collection: Arc<dyn CollectionView<S>>,
    children: HashMap<&'static str, Box<dyn Router<T>>>,
}

#[async_trait]
impl<S, T> Router<S> for CollectionRouter<S, T>
where
    S: ?Sized + Send + Sync,
    T: ?Sized + Send + Sync + CollectionChild<S>,
{
    async fn route(&self, path: &str, request: Request, state: Box<S>) -> Response {
        match Parts::split(path) {
            Parts::Nil => self.collection.handle(request, state).await,
            Parts::Cons(id, rest) => match Parts::split(rest) {
                Parts::Nil => self.item.handle(request, id.into(), state).await,
                Parts::Cons(part, rest) => match self.children.get(part) {
                    Some(child) => child.route(rest, request, T::from(state, id, part)).await,
                    None => Response::new("not found!".into()),
                },
            },
        }
    }
}
