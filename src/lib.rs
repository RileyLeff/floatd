//! Tiny crate that provides a "FloatD" trait, i.e. num_traits::Float + Debug + Display.
//! This is likely to be subsumed into a successor crate once my project is more mature.
//! For now, this is the most convenient place to keep the trait.
//!
//! According to the rust api guidelines chapter on [futureproofing](https://rust-lang.github.io/api-guidelines/future-proofing.html),
//! specifying Display and Debug as trait bounds is bad practice.
//!
//! I think my downstream use case for this crate (guaranteeing a struct's underlying data can be formatted into an associated error type)
//! falls under exception 1 listed in the guidelines. Your mileage may vary, though.

// configure no_std if both std_math and std_errors features are inactive
#![cfg_attr(not(feature = "std"), no_std)]

use num_traits::Float;

#[cfg(not(feature = "std"))]
use core::fmt::Debug;
#[cfg(not(feature = "std"))]
use core::fmt::Display;

#[cfg(feature = "std")]
use std::fmt::Debug;
#[cfg(feature = "std")]
use std::fmt::Display;

pub trait FloatD: Display + Debug + Float {}

impl<T: Display + Debug + Float> FloatD for T {}
