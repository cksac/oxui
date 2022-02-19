#![feature(ptr_metadata)]

#[macro_use]
extern crate lazy_static;

pub mod foundation;
pub mod painting;
pub mod rendering;
pub mod ui;

#[cfg(test)]
mod tests;
