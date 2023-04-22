//! **Croot** - *Complex Roots*
//!
//! For finding *all* nth-roots

#![no_std]
#![forbid(unsafe_code, clippy::unwrap_used)]

extern crate alloc;

pub mod roots;
pub mod traits;

pub mod prelude {
    use super::*;

    pub use roots::prelude::*;
    pub use traits::prelude::*;
}
