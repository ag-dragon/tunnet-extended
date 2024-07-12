use std::env;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(Steam)");
    println!("cargo::rustc-cfg={}", env::var("TE_VERSION").unwrap());
}