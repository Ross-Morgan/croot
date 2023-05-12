use num_complex::Complex64;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Period(pub(crate) Complex64);

impl Period {
    pub const fn new(c: Complex64) -> Self { Self(c) }
    pub const fn into_inner(self) -> Complex64 { self.0 }

    pub fn nth(&self, n: i32) -> Complex64 { self.0 * (n as f64) }
}