#![feature(specialization)]

use hyper;

pub mod parts;
pub mod router;
pub mod tree;

pub type Request = hyper::Request<hyper::Body>;
pub type Response = hyper::Response<hyper::Body>;
