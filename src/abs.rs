use {Abs, Eq};

macro_rules! impls {
    ($($ty:ident),+) => {
        $(
            impl Eq<Abs<$ty>> for $ty {
                fn approx_eq(&self, rhs: &$ty, tolerance: Abs<$ty>) -> bool {
                    let diff = (self - rhs).abs();

                    diff <= tolerance.0
                }
            }
         )+
    }
}

impls!(f32, f64);

macro_rules! test {
    ($ty:ident) => {
        mod $ty {
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
