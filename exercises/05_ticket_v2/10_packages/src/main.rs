// This is a `main.rs` file, therefore `cargo` interprets this as the root of a binary target.
use packages::hello_world;

// This is the entrypoint of the binary.
fn main() {
    hello_world();
}
