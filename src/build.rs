use std::env;

fn main() {
    // Get the list of targets from the CARGO_TARGET_DIR environment variable
    let targets = env::var("CARGO_TARGET_DIR").unwrap();

    // Iterate over the targets and build each binary
    for target in targets.split(":") {
        println!("cargo:rustc-link-search={}", target);
        println!("cargo:rustc-link-lib=static=my_library");
    }
}
