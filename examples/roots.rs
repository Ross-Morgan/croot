use croot::prelude::*;

fn main() {
    // All 4th-roots of 81
    let roots = root(81.0, 4).precision(5);

    println!("{roots:?}");
}