//! A simple crate that houses most of the Eludris models & shared logic.

#[macro_use]
extern crate lazy_static;

pub mod ids;
pub mod models;

#[cfg(feature = "oprish")]
pub mod oprish;
