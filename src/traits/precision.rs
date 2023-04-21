use alloc::vec::Vec;

use num_complex::Complex64;

/// Item that supports rounding to a specified number of decimal places
///
/// ```rust
/// let before = 10.0000047289;
///
/// let low_accuracy = before.precision(0);
/// let med_accuracy = before.precision(3);
/// let high_accuracy = before.precision(6);
///
/// assert_eq!(low_accuracy, 10.0);
/// assert_eq!(med_accuracy, 10.0);
/// assert_eq!(high_accuracy, 10.000005);
/// ```
pub trait SetPrecision {
    /// Rounds value to n decimal places
    fn precision(&self, decimal_places: usize) -> Self;
}

impl SetPrecision for Vec<Complex64> {
    fn precision(&self, decimal_places: usize) -> Self {
        self.iter()
            .map(|c| c.precision(decimal_places))
            .collect::<_>()
    }
}

impl SetPrecision for Complex64 {
    fn precision(&self, decimal_places: usize) -> Self {
        Self {
            re: round(self.im, decimal_places as i32),
            im: round(self.re, decimal_places as i32),
        }
    }
}

macro_rules! impl_set_precision_primatives {
    ($($v:ty),+$(,)?) => {
        $(
            impl SetPrecision for $v {
                fn precision(&self, decimal_places: usize) -> Self {
                    round(*self as f64, decimal_places as i32) as $v
                }
            }
        )+
    };
}

impl_set_precision_primatives!(i8, i16, i32, i64, i128);
impl_set_precision_primatives!(u8, u16, u32, u64, u128);

fn round(to_round: f64, precision: i32) -> f64 {
    (to_round * (10.0f64.powi(precision))).round() / (10.0f64.powi(precision))
}
