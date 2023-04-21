use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// An object that can be transformed into a vec of Strings
pub trait ToStrings {
    fn to_strings(&self) -> Vec<String>;
}

impl<T: ToString> ToStrings for Vec<T> {
    fn to_strings(&self) -> Vec<String> {
        self.iter().map(|o| o.to_string()).collect::<_>()
    }
}
