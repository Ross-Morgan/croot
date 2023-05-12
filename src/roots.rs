use alloc::vec::Vec;
use core::f64::consts::PI;

use num_complex::Complex64;

use crate::period::Period;

const I: Complex64 = Complex64::new(0.0, 1.0);

pub fn principal_root<T, U>(base: T, power: U) -> Complex64
    where Complex64: From<T> + From<U>,
{
    Complex64::from(base).powc(Complex64::from(power).inv())
}

pub fn root<T>(radicand: T, index: usize) -> Vec<Complex64>
    where T: Into<Complex64>
{
    let base: Complex64 = radicand.into();

    let multiplicative_period = (2.0 * I * PI / (index as f64)).exp();
    let principal = principal_root(base, index as f64);

    let mut buf = Vec::with_capacity(index);

    buf.push(principal);

    for _ in 1..index {
        buf.push(buf.last().expect("Buffer empty") * multiplicative_period);
    }

    buf
}

pub fn complex_root<T>(radicand: T, index: Complex64) -> (Complex64, Period)
    where T: Into<Complex64>
{
    let base: Complex64 = radicand.into();

    let multiplicative_period = (index.conj() / index.norm_sqr()).exp();

    let principal = principal_root(base, index);

    (principal, Period(multiplicative_period))
}
