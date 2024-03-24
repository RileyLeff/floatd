//! Tiny crate that provides a "FloatD" trait, i.e. num_traits::Float + Debug + Display.
//! This is likely to be subsumed into a successor crate once my project is more mature.
//! For now, this is the most convenient place to keep the trait.

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
