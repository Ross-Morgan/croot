/// Returns the r value for any nth-root of a complex number
pub fn radius(n: f64, nth_root: usize) -> f64 {
    round(n.abs().powf(1.0 / (nth_root as f64)), 12)
}

/// Returns the nth-roots of 1

/// Round a number to a specified number of decimal places
///
/// # Panics
/// Will panic if to_round * 10^precision > f64::MAX
///
/// TODO: Experiment with rounding using logarithms
pub(super) fn round(to_round: f64, precision: i32) -> f64 {
    (to_round * (10.0f64.powi(precision))).round() / (10.0f64.powi(precision))
}