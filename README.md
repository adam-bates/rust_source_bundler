# Rust Source Bundler

[![Crates.io](https://img.shields.io/crates/v/rust_source_bundler.svg)](https://crates.io/crates/rust_source_bundler)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/adam-bates/rust_source_bundler#license)
[![Crates.io](https://img.shields.io/crates/d/rust_source_bundler.svg)](https://crates.io/crates/rust_source_bundler)

Easily bundle local Rust library files into a single file.

This can be useful when importing or generating Rust code as you can `include!` or `include_str!` on a single generated file containing all modules.

### Example

You can run the program on this library's source files
```
cargo run --example rust_source_bundler
```

### Usage

Given the following files:

```
project/src/
|- helpers/
|  |- inner.rs
|  |- mod.rs
|- lib.rs
|- utils.rs

```

```rust
// project/src/lib.rs

pub mod utils;

mod helpers;

use helpers::helper_fn;

pub fn lib_fn() {}
```

```rust
// project/src/utils.rs

pub fn utils_fn() {}
```

```rust
// project/src/helpers/mod.rs

mod inner;

pub use inner::*;

pub fn helper_fn() {}
```

```rust
// project/src/helpers/inner.rs

pub fn inner_fn() {}
```

You can use this library:

```rust
// project/build.rs to generate code on build

fn main() {
    let code = rust_source_bundler::bundle_source("./project/src", "lib.rs").unwrap();
    
    println!("{code}");

    /* Prints:

pub mod utils {
    pub fn utils_fn() {}
}
mod helpers {
    mod inner {
        pub fn inner_fn() {}
    }

    pub use inner::*;

    pub fn helper_fn() {}
}

use helpers::helper_fn;

pub fn lib_fn() {}

    */
}
```

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
