extern crate rand;

use std::char;
use std::env;
use rand::Rng;

fn generate(n: u32) -> String {
    (0..n).flat_map(|_| {
        let x = rand::thread_rng().gen_range(0x1400, 0x167F);
        char::from_u32(x)
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(cmd) => {
            match &**cmd {
                "rand" => {
                    let n: u32 = args.get(2).and_then( |s| { s.trim().parse().ok() }).unwrap_or(140);
                    println!("{}", generate(n));
                },
                _ => {
                    println!("unknow option ; {}", cmd);
                }
            }
        },
        None => {
          println!("args required ");
        }
    }
}
