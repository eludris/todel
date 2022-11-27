//! A simple crate that houses most of the Eludris models & shared logic.

#[cfg(feature = "logic")]
#[macro_use]
extern crate lazy_static;

pub mod conf;
#[cfg(feature = "logic")]
pub mod ids;
pub mod models;
#[cfg(feature = "http")]
pub mod oprish;

pub use conf::Conf;
