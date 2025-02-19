default:
    @just --list

fmt:
  cargo fmt --all

clippy:
  cargo clippy --all-targets --all-features

lint: fmt clippy

check:
  cago check

test:
  cargo test --workspace --no-fail-fast

pretty-test:
  cargo pretty-test --workspace --no-fail-fast

build:
  cargo build --release

run:
  cargo run --release

cli:
  cargo run --bin cli --release

