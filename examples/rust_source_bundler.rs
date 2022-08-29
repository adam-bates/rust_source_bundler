use rust_source_bundler::bundle_source;

const ROOT_SRC_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/src");

fn main() {
    let code = bundle_source(ROOT_SRC_DIR, "lib.rs").unwrap();
    println!("{code}");
}

