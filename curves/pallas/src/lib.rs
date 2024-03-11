#![no_std]

#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the prime-order curve Pallas, generated by
//! [Daira Hopwood](https://github.com/zcash/pasta). The main feature of this
//! curve is that it forms a cycle with Vesta, i.e. its scalar field and base
//! field respectively are the base field and scalar field of Vesta.
//!
//!
//! Curve information:
//! * Base field: q =
//!   28948022309329048855892746252171976963363056481941560715954676764349967630337
//! * Scalar field: r =
//!   28948022309329048855892746252171976963363056481941647379679742748393362948097
//! * Curve equation: y^2 = x^3 + 5
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 32

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "r1cs")]
pub mod constraints;
#[cfg(feature = "curve")]
mod curves;
#[cfg(any(feature = "scalar_field", feature = "base_field"))]
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;
#[cfg(any(feature = "scalar_field", feature = "base_field"))]
pub use fields::*;
