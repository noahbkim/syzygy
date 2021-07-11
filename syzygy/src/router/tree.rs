use crate::router::{Path, Route};

pub mod collection;
pub mod view;

pub trait TreeNode<S>: Send + Sync
where
    S: ?Sized + Send + Sync,
{
    fn route(&self, path: Path, state: Box<S>) -> Option<Box<Route>>;
}
