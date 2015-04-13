use {Eq, Rel};

macro_rules! impls {
    ($($ty:ident),+) => {
        $(
            impl Eq<Rel<$ty>> for $ty {
                fn approx_eq(&self, rhs: &$ty, tolerance: Rel<$ty>) -> bool {
                    let diff = (self - rhs).abs();
                    let largest = self.abs().max(rhs.abs());

                    diff <= tolerance.0 * largest
                }
            }
         )+
    }
}

impls!(f32, f64);

macro_rules! test {
    ($ty:ident) => {
        mod $ty {
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
