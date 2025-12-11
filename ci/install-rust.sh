source ./ci/env.sh
export CARGO_HOME='/usr/local/cargo'

RUSTUP_VERSION=1.28.2
RUST_ARCH="x86_64-unknown-linux-gnu"

RUSTUP_URL=https://static.rust-lang.org/rustup/archive/$RUSTUP_VERSION/$RUST_ARCH/rustup-init
wget $RUSTUP_URL

chmod +x rustup-init;
./rustup-init -y --no-modify-path --profile minimal;
rm rustup-init;
chmod -R a+w $RUSTUP_HOME $CARGO_HOME

for rust_version in "$@"
do
  rustup toolchain install "$rust_version"
done

rustup default stable

rustup --version
cargo --version
rustc --version

rustup component add --toolchain stable clippy-preview
rustup component add --toolchain stable rustfmt
cargo install --force cargo-deny
cargo install --force --git https://github.com/kbknapp/cargo-outdated

# Install miri which requires a specific version of nightly
echo nightly-$(curl -s https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/miri) > /nightly-version
rustup component add --toolchain $(cat /nightly-version) miri
cargo +$(cat /nightly-version) miri setup

# fetch project deps
# msrv
cargo generate-lockfile --config 'resolver.incompatible-rust-versions="fallback"'
cargo fetch --locked
# latest compatible
cargo generate-lockfile
cargo fetch --locked

