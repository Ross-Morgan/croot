#![no_std]
#![forbid(unsafe_code, clippy::unwrap_used)]
extern crate alloc;

pub mod roots;
pub mod traits;

pub mod prelude {
    use super::*;

    pub use roots::{roots_of, roots_of_unity};
    pub use traits::prelude::*;
}
