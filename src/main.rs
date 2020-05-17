extern crate rand;

use std::char;
use rand::Rng;

fn generate(n: u32) -> String {
    (0..n).flat_map(|_| {
        let x = rand::thread_rng().gen_range(0x1400, 0x167F);
        char::from_u32(x)
    }).collect()
}

fn main() {
    println!("{}", generate(140));
}
