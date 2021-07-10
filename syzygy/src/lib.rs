#![feature(specialization)]

use hyper;

pub mod router;
pub mod tree;
pub mod parts;

pub type Request = hyper::Request<hyper::Body>;
pub type Response = hyper::Response<hyper::Body>;
