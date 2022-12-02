set dotenv-load

run:
    cargo run
release:
    cross build --release --target aarch64-unknown-linux-gnu
    cross build --release --target aarch64-unknown-linux-musl
    cross build --release --target x86_64-unknown-linux-gnu
    cross build --release --target x86_64-unknown-linux-muslw