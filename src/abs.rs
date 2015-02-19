use std::num::Float;

use {Abs, Eq};

impl<T> Eq<Abs<T>> for T where T: Float {
    fn approx_eq(&self, rhs: &T, tolerance: Abs<T>) -> bool {
        let &lhs = self;
        let &rhs = rhs;

        let diff = (lhs - rhs).abs();

        diff <= tolerance.0
    }
}

impl<'a, T, U> Eq<Abs<T>> for &'a U where U: Eq<Abs<T>> {
    fn approx_eq(&self, rhs: &&U, tolerance: Abs<T>) -> bool {
        Eq::approx_eq(*self, *rhs, tolerance)
    }
}

macro_rules! test {
    ($ty:ident) => {
        mod $ty {
            use std::num::Float;

            /// Given:
            /// - `x`: any float (minus `NaN`)
            /// - `tol`: any tolerance in the range `(0, 1]`
            ///
            /// Test that `x.approx_eq(NaN)` is always false
            #[quickcheck]
            fn nan(x: $ty, n: $ty) -> bool {
                let tol = ::Abs::tol((-n.abs()).exp2());

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
                let tol = ::Abs::tol((-n.abs()).exp2());

                ::eq(&x, &x, tol)
            }

            /// Given:
            /// - `tol`: any tolerance in the range `(0, 1]`
            ///
            /// Test that `NaN.approx_eq(NaN)` is always false
            #[quickcheck]
            fn nan_nan(n: $ty) -> bool {
                let tol = ::Abs::tol((-n.abs()).exp2());

                let zero: $ty = 0.;
                let nan = zero / zero;

                !::eq(&nan, &nan, tol)
            }
        }
    };
}

#[cfg(test)]
mod test {
    test!(f32);
    test!(f64);
}
