//! **Croot** - *Complex Roots*
//!
//! For finding *all* nth-roots

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::all)]

extern crate alloc;

pub mod approx;
pub mod period;
pub mod roots;

pub mod prelude {
    use super::*;

    pub use approx::Approx;
    pub use period::Period;
    pub use roots::{complex_root, principal_root, root};
}
