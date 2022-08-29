# Rust Source Bundler

Easily bundle local Rust library files into a single file.

This can be useful when importing or generating Rust code as you can `include!` or `include_str!` on a single generated file containing all modules.

### Example Usage

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
