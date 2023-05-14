use num_complex::Complex64;

/// Wrapper around a complex number that represents
/// the multiplicative period of a root
///
/// For a root `R_n` The multiplicative period `P` satisfies the equation:
/// `R_n+1 = R_n * P`
///
/// The multiplicative period can be calculated with the following formula:
///
/// # Calculation
///
/// `e^(2iπz* / |z|²)`
///
/// where `z*` is the complex conjugate and `|z|` is the magnitude.
///
/// # Examples
/// ```rust
/// # use croot::prelude::*;
/// # use core::f64::consts::PI;
/// # use num_complex::Complex64;
/// # const I: Complex64 = Complex64::new(0.0, 1.0);
/// let (root, period) = complex_root(10.0, Complex64::new(3.0, 4.0));
///
/// let root_1 = root.powc(Complex64::new(3.0, 4.0)).approx(5);
/// let root_2 = (root * period.into_inner()).powc(Complex64::new(3.0, 4.0)).approx(5);
///
/// assert_eq!(root_1, root_2);
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Period(pub(crate) Complex64);

impl Period {
    pub const fn new(c: Complex64) -> Self {
        Self(c)
    }

    pub const fn raw(self) -> Complex64 {
        self.0
    }

    pub const fn into_inner(self) -> Complex64 {
        self.raw()
    }

    pub fn nth(&self, n: i32) -> Complex64 {
        self.0 * (n as f64)
    }
}
