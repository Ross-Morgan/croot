use croot::prelude::*;

fn main() {
    let roots = roots_of_unity(4).precision(10).to_strings();

    println!("{roots:#?}");
}
