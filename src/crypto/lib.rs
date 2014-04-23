#![crate_id = "crypto#0.2.0"]
#![crate_type = "lib"]
#![license = "MIT"]

extern crate num;

#[cfg(test)]
extern crate test;
#[cfg(test)]
extern crate serialize;

pub mod base58;
pub mod hash;
