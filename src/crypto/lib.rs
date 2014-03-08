#[crate_type = "rlib"];
#[crate_type = "dylib"];

#[crate_id = "crypto#0.1.0"];

#[license = "MIT"];

extern crate num;

#[cfg(test)]
extern crate test;

pub mod base58;
pub mod hash;
