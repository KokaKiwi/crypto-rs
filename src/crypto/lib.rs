#![crate_id = "crypto#0.2.0"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![license = "MIT"]

extern crate num;

#[cfg(test)]
extern crate test;
#[cfg(test)]
extern crate serialize;

pub mod base58;
pub mod hash;
