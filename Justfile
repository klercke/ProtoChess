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
  cargo test

build:
  cargo build

run:
  cargo run

