use async_trait::async_trait;

use crate::{Request, Response};

#[async_trait]
pub trait View<S>: Send + Sync
where
    S: ?Sized + Send + Sync,
{
    async fn handle(&self, request: Request, state: Box<S>) -> Response;
}

#[async_trait]
pub trait CollectionView<S>: Send + Sync
where
    S: ?Sized + Send + Sync,
{
    async fn handle(&self, request: Request, state: Box<S>) -> Response;
}

#[async_trait]
pub trait ItemView<S>: Send + Sync
where
    S: ?Sized + Send + Sync,
{
    async fn handle(&self, request: Request, id: String, state: Box<S>) -> Response;
}
