#![feature(ptr_metadata)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate lazy_static;

pub mod painting;
pub mod rendering;
pub mod ui;

#[cfg(test)]
mod tests;
