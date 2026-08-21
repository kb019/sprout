set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

default:
    @just --list

run:
    cargo run

fmt:
    cargo fmt

lint:
    cargo fmt --check
    cargo clippy -- -D warnings
    typos


test:
    cargo test

setup:
    cargo test
    cargo install --path . --locked
    @echo "✅ Finished setup"

clean:
    cargo clean
