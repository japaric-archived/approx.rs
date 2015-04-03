//! An attempt at sane float comparisons
//!
//! First, you must read [this][1].
//!
//! [1]: http://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition
//!
//! # What's in the crate?
//!
//! This crate provides three methods to test for *approximate* equality between floats:
//!
//! - Absolute difference (`Abs`): Checks that the absolute difference `|lhs - rhs|` is within some
//! tolerance.
//! - Relative difference (`Rel`): Checks that the relative difference
//! `|lhs - rhs| / max(lhs, rhs)` is within some tolerance.
//!
//! The recommended way to use this crate, is to use the `approx::eq` free function:
//!
//! ``` ignore
//! use approx::{Rel, self};
//!
//! let v = create_rand_vec();
//! let x = naive_sum(&v);
//! let y = simd_sum(&v);
//!
//! assert!(aprox::eq(&x, &y, Rel::tol(1e-5)))
//! ```
//!
//! You can also use a macro to reduce verbosity:
//!
//! ``` ignore
//! macro_rules! assert_approx_eq {
//!     ($lhs:expr, $rhs:expr) => {
//!         assert!(::approx::eq(&($lhs), &($rhs), ::approx::Rel::tol(1e-5)));
//!     }
//! }
//!
//! assert_approx_eq!(x, y);
//! ```
//!
//! # What method should I use?
//!
//! There is no silver bullet, each method has it pros and cons. Some thoughts:
//!
//! - `Rel` performs poorly when the values are near zero. `Abs` should be preferred in those
//!   cases.
//! - Picking a good tolerance for `Abs` requires having an idea of how big/small the values
//!   actually are.
//! - You can combine the methods by `||`ing them, with this approach you can counter each other
//!   weaknesses. For example:
//!
//!
//! ``` ignore
//! approx::eq(&x, &y, Abs::tol(1e-5)) || approx::eq::(&x, &y, Abs::rel(1e-5));
//! ```
//!
//! For values near zero, the `Abs` method will correctly detect approximate equality, and for
//! large values `Abs` tolerance will be too small, but then the `Rel` method will kick in and
//! catch what `Abs` missed.

#![cfg_attr(test, allow(trivial_casts))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(plugin)]

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate rand;

extern crate float;

mod abs;
mod rel;

use float::Float;

/// Approximate equality
pub trait Eq<Method> {
    /// Checks if two floats are approximately equal according to specified `Method` and
    /// `tolerance`
    fn approx_eq(&self, rhs: &Self, tolerance: Method) -> bool;
}

/// Absolute difference
///
/// Note: You should prefer this method when the values are near zero
#[derive(Clone, Copy)]
pub struct Abs<T>(T) where T: Float;

// TODO(rust-lang/rfcs#735) move this `impl` to the `abs` module
impl<T> Abs<T> where T: Float {
    /// Creates an absolute difference tolerance
    ///
    /// # Panics
    ///
    /// Panics if `x` is negative
    pub fn tol(x: T) -> Abs<T> {
        let _0 = T::from(0);

        assert!(x >= _0);

        Abs(x)
    }
}

/// Relative difference
///
/// Note: This method breaks down when the values are near zero
#[derive(Clone, Copy)]
pub struct Rel<T>(T) where T: Float;

// TODO(rust-lang/rfcs#735) move this `impl` to the `rel` module
impl<T> Rel<T> where T: Float {
    /// Creates a relative difference tolerance
    ///
    /// # Panics
    ///
    /// Panics if `x` is negative
    pub fn tol(x: T) -> Rel<T> {
        let _0 = T::from(0);

        assert!(x >= _0);

        Rel(x)
    }
}

/// Checks if `lhs` is approximately equal to `rhs` using the specified `Method` and `tolerance`
pub fn eq<A, Method>(lhs: &A, rhs: &A, tolerance: Method) -> bool where A: Eq<Method> {
    lhs.approx_eq(rhs, tolerance)
}
