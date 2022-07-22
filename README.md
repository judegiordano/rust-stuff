# Rust Stuff

[Crates](https://crates.io)

[The Book](https://doc.rust-lang.org/stable/book/title-page.html)

[Rust By Example](https://doc.rust-lang.org/stable/rust-by-example)

[Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/about.html)

```sh
rustup toolchain install nightly
rustup component add clippy
rustup show
rustc src/main.rs
rustc --explain <error code>
cargo +nightly test
cargo init
cargo --version
cargo check
cargo fmt
cargo run
cargo build
cargo update
cargo add <package>
cargo add <package>@<versionNumber>
cargo add <package> --features <features>
cargo build --release
cargo doc --open
cargo test
cargo bench
cargo clippy
cargo clippy --fix
cargo new --lib <name>
cargo install cargo-audit
cargo audit
# do not run tests with parallelism
cargo test -- --test-threads=1
cargo test -- --show-output
cargo test <specific_test_name>
cargo test <partial_test_name>
cargo test -- --ignored
cargo test -- --include-ignored
cargo run <arguments> > output.txt
# diesel
cargo install diesel_cli
```
