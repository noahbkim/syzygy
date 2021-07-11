use crate::router::{Path, Route};

pub mod view;

pub trait TreeNode<S>: Send + Sync
where
    S: ?Sized + Send + Sync,
{
    fn route(&self, path: Path, state: Box<S>) -> Option<Box<Route>>;
}

pub trait Transition<S>
where
    S: ?Sized,
{
    fn from(s: Box<S>, t: &str) -> Box<Self>;
}
