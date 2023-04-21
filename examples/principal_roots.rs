use croot::prelude::*;

fn main() {
    let x = 81;

    // Principal 4th root of 81
    let principal = principal_root(x as f64, 4);

    println!("{principal:?}");
}
