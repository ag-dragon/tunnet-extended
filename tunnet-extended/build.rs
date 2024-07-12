use std::env;

fn main() {
    println!("cargo::rustc-cfg={}", env::var("TE_VERSION").unwrap());
}