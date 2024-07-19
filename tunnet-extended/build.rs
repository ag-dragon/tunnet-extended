use std::env;

fn main() {
    println!("cargo::rerun-if-env-changed=TE_VERSION");
    match env::var("TE_VERSION") {
        Ok(te_version) => {
            match te_version.as_str() {
                "Steam" | "Itchio" => println!("cargo::rustc-cfg={}", te_version),
                _ => panic!("Unrecognized TE_VERSION value, only 'Steam' and 'Itchio' are accepted."),
            }
        },
        Err(e) => panic!("Could not retrieve TE_VERSION enviornment variable, make sure it is set to either 'Steam' or 'Itchio': {}", e),
    }
}
