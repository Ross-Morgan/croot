use alloc::vec::Vec;
use core::f64::consts::PI;

use num_complex::Complex64;

use crate::period::Period;

const I: Complex64 = Complex64::new(0.0, 1.0);

/// Returns radius / magnitude of a real or complex value
pub fn radius<T>(c: T) -> f64
    where Complex64: From<T>
{
    Complex64::from(c).norm()
}

/// Returns the root with the largest real part
///
/// # Examples
/// ```rust
/// # use croot::prelude::*;
/// # use num_complex::Complex64;
///
/// let principal = principal_root(81.0, 4.0).approx(5);
///
/// assert_eq!(principal, Complex64::from(3.0));
/// ```
pub fn principal_root<T, U>(base: T, power: U) -> Complex64
where
    Complex64: From<T> + From<U>,
{
    Complex64::from(base).powc(Complex64::from(power).inv())
}

/// Returns all complex roots of a value
///
/// # Examples
/// ```rust
/// # use croot::prelude::*;
/// # use num_complex::Complex64;
///
/// let roots = root(32.0, 5);
/// let original = roots.iter().map(|c| c.powi(5).approx(5)).collect::<Vec<_>>();
///
/// assert_eq!(original.as_slice(), &[Complex64::from(32.0); 5]);
/// ```
pub fn root<T>(radicand: T, index: usize) -> Vec<Complex64>
where
    T: Into<Complex64>,
{
    let base: Complex64 = radicand.into();

    let multiplicative_period = (2.0 * I * PI / (index as f64)).exp();
    let principal = principal_root(base, index as f64);

    let mut buf = Vec::with_capacity(index);

    buf.push(principal);

    //
    for _ in 1..index {
        buf.push(buf.last().expect("Buffer empty") * multiplicative_period);
    }

    buf
}

/// Returns the principal complex root
/// and the multiplicative period of that root
///
/// # Examples
/// ```rust
/// # use croot::prelude::*;
/// # use num_complex::Complex64;
///
/// let (root, period) = complex_root(10.0, Complex64::new(3.0, 4.0));
/// let original = root.powc(Complex64::new(3.0, 4.0)).approx(5);
///
/// assert_eq!(original, Complex64::from(10.0));
/// ```
pub fn complex_root<T>(radicand: T, index: Complex64) -> (Complex64, Period)
where
    T: Into<Complex64>,
{
    let base: Complex64 = radicand.into();

    let multiplicative_period = (2.0 * I * PI * index.conj() / index.norm_sqr()).exp();
    let principal = principal_root(base, index);

    (principal, Period(multiplicative_period))
}
