//! ULP distance

use std::mem;
use std::num::{Float, Int};

use {Eq, Ulp};

/// Float distance in ULPs
pub trait Distance: Float {
    type Ulp: Int;

    /// Returns the absolute distance between two floats in ULPs
    fn distance(&self, &Self) -> Option<Self::Ulp>;
}

impl Distance for f32 {
    type Ulp = i32;

    fn distance(&self, rhs: &f32) -> Option<i32> {
        let &lhs = self;
        let &rhs = rhs;

        if lhs.is_nan() || rhs.is_nan() {
            // ULP idstance doesn't make sense for `NaN`s
            None
        } else {
            let lhs: i32 = unsafe {
                mem::transmute(lhs)
            };
            let rhs: i32 = unsafe {
                mem::transmute(rhs)
            };

            if lhs >> 31 != rhs >> 31 {
                // ULP distance will overflow if the sign bit doesn't match
                None
            } else {
                Some(
                    if lhs > rhs {
                        lhs - rhs
                    } else {
                        rhs - lhs
                    }
                )
            }
        }
    }
}

impl Distance for f64 {
    type Ulp = i64;

    fn distance(&self, rhs: &f64) -> Option<i64> {
        let &lhs = self;
        let &rhs = rhs;

        if lhs.is_nan() || rhs.is_nan() {
            // ULP idstance doesn't make sense for `NaN`s
            None
        } else {
            let ilhs: i64 = unsafe {
                mem::transmute(lhs)
            };
            let irhs: i64 = unsafe {
                mem::transmute(rhs)
            };

            if ilhs >> 63 != irhs >> 63 {
                // ULP distance will overflow if the sign bit doesn't match
                None
            } else {
                Some(
                    if ilhs > irhs {
                        ilhs - irhs
                    } else {
                        irhs - ilhs
                    }
                )
            }
        }
    }
}

impl<T: Distance> Eq<Ulp<T::Ulp>> for T {
    fn approx_eq(&self, rhs: &T, tol: Ulp<T::Ulp>) -> bool {
        let lhs = self;

        match lhs.distance(rhs) {
            Some(diff) => {
                diff <= tol.0
            },
            None => {
                // Check for `0 == -0`, if any side is a `NaN` this returns false
                lhs == rhs
            },
        }
    }
}

macro_rules! test {
    ($float:ident, $int:ident) => {
        mod $float {
            /// Given:
            /// - `x`: any float (minus `NaN`)
            /// - `tol`: any tolerance >= 0
            ///
            /// Test that `x.approx_eq(NaN)` is always false
            #[quickcheck]
            fn nan(x: $float, n: $int) -> bool {
                let tol = ::Ulp::tol(if n > 0 { n } else { -n });

                let zero: $float = 0.;
                let nan = zero / zero;

                !::eq(&nan, &x, tol) && !::eq(&x, &nan, tol)
            }

            /// Given:
            /// - `x`: any float (minus `NaN`)
            /// - `tol`: any tolerance >= 0
            ///
            /// Test that `x.approx_eq(x)` is always true
            #[quickcheck]
            fn same(x: $float, n: $int) -> bool {
                let tol = ::Ulp::tol(if n > 0 { n } else { -n });

                ::eq(&x, &x, tol)
            }

            /// Given:
            /// - `tol`: any tolerance >= 0
            ///
            /// Test that `NaN.approx_eq(NaN)` is always false
            #[quickcheck]
            fn nan_nan(n: $int) -> bool {
                let tol = ::Ulp::tol(if n > 0 { n } else { -n });

                let zero: $float = 0.;
                let nan = zero / zero;

                !::eq(&nan, &nan, tol)
            }
        }
    };
}

#[cfg(test)]
mod test {
    test!(f32, i32);
    test!(f64, i64);
}
