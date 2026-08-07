set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

default:
    @just --list

run:
    cargo run

fmt:
    cargo fmt

lint:
    cargo fmt
    cargo clippy -- -D warnings
    typos


test:
    cargo test

install-hooks:
    git config core.hooksPath .githooks
    @echo "✅ Git hooks installed"

setup: install-hooks
    cargo install --path . --locked
    @echo "✅ Finished setup"

clean:
    cargo clean
