use alloc::vec::Vec;

use num_complex::Complex64;

pub trait Approx {
    fn approx(&self, precision: usize) -> Self;
}

impl Approx for Complex64 {
    fn approx(&self, precision: usize) -> Self {
        let scale_factor = 10.0_f64.powi(precision as i32);

        Complex64::new(
            (self.re * scale_factor).round() / scale_factor,
            (self.im * scale_factor).round() / scale_factor,
        )
    }
}

impl Approx for Vec<Complex64> {
    fn approx(&self, precision: usize) -> Self {
        self.iter().map(|c| c.approx(precision)).collect()
    }
}

#[cfg(test)]
mod tests {
    use core::assert_eq;

    use crate::prelude::root;

    use super::Approx;

    #[test]
    fn approx_test() {
        let original = root(81.0, 4).get(1).copied().unwrap();

        let rounded = original.approx(10);

        assert_eq!(rounded.re, 0.0);
    }
}
