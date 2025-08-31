README.md

???

# Cross-compilation directly

## Install cross-compilation toolchains

brew install musl-cross

or 
brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-musl
brew install x86_64-unknown-linux-musl


# Cross-compilation with cross

## Install cross
cargo install cross

## Build for different targets (uses Docker internally)

Need
docker pull --platform linux/x86_64 ghcr.io/cross-rs/x86_64-unknown-linux-gnu

docker run --privileged --rm tonistiigi/binfmt --install amd64
rustup toolchain install stable-x86_64-unknown-linux-gnu --force-non-host


cross build --release --target x86_64-unknown-linux-musl --force-non-host
cross build --release --target aarch64-unknown-linux-musl --force-non-hos


# Cross compilation with docker
docker create --name temp-container hello
docker cp temp-container:/hello_cargo ./hello_cargo-arm64
docker rm temp-container