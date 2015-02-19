use std::num::Float;

use {Eq, Rel};

impl<T> Eq<Rel<T>> for T where T: Float {
    fn approx_eq(&self, rhs: &T, tolerance: Rel<T>) -> bool {
        let &lhs = self;
        let &rhs = rhs;

        let diff = (lhs - rhs).abs();
        let largest = lhs.abs().max(rhs.abs());

        diff <= tolerance.0 * largest
    }
}

macro_rules! test {
    ($ty:ident) => {
        mod $ty {
            use std::num::Float;

            use quickcheck::TestResult;

            /// Given:
            /// - `x`: any float (minus `NaN`)
            /// - `tol`: any tolerance in the range `(0, 1]`
            ///
            /// Test that `x.approx_eq(NaN)` is always false
            #[quickcheck]
            fn nan(x: $ty, n: $ty) -> bool {
                let tol = ::Rel::tol((-n.abs()).exp2());

                let zero: $ty = 0.;
                let nan = zero / zero;

                !::eq(&nan, &x, tol) && !::eq(&x, &nan, tol)
            }

            /// Given:
            /// - `x`: any float (minus `NaN`)
            /// - `tol`: any tolerance in the range `(0, 1]`
            ///
            /// Test that `x.approx_eq(x)` is always true
            #[quickcheck]
            fn same(x: $ty, n: $ty) -> bool {
                let tol = ::Rel::tol((-n.abs()).exp2());

                ::eq(&x, &x, tol)
            }

            /// Given:
            /// - `tol`: any tolerance in the range `(0, 1]`
            ///
            /// Test that `NaN.approx_eq(NaN)` is always false
            #[quickcheck]
            fn nan_nan(n: $ty) -> bool {
                let tol = ::Rel::tol((-n.abs()).exp2());

                let zero: $ty = 0.;
                let nan = zero / zero;

                !::eq(&nan, &nan, tol)
            }

            /// Given:
            /// - `x`: any float (minus `NaN` and zero)
            /// - `tol`: any tolerance in the range `(0, 1]`
            ///
            /// Test that `x.approx_eq(zero)` is always false
            #[quickcheck]
            fn zero(x: $ty, n: $ty) -> TestResult {
                let tol = ::Rel::tol((-n.abs()).exp2());

                let zero: $ty = 0.;

                if x == zero || tol.0 == 1. {
                    TestResult::discard()
                } else {
                    TestResult::from_bool(!::eq(&zero, &x, tol) && !::eq(&x, &zero, tol))
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    test!(f32);
    test!(f64);
}
