_list:
    @just --list --unsorted

remedy: && fmt check
    cargo fix --allow-staged --allow-dirty
    cargo clippy --fix --allow-staged --allow-dirty

# Check project
check:
    just --unstable --fmt --check
    nix fmt -- --check .
    taplo fmt --check `fd --extension=toml`
    prettier --check `fd --extension=md`
    cargo fmt --manifest-path crates/frontend/Cargo.toml --check
    cargo fmt --manifest-path crates/simulation/Cargo.toml --check
    cargo fmt --manifest-path crates/common/Cargo.toml --check
    cargo clippy --tests --examples -- -D warnings
    taplo lint `fd --extension=toml`
    RUSTDOCFLAGS='-Dwarnings' cargo doc --no-deps
    cargo nextest run
    nix flake show
    cargo udeps
    cargo outdated --depth=1

# Format code
fmt:
    just --unstable --fmt
    nix fmt
    taplo fmt `fd --extension=toml`
    cargo fmt --manifest-path crates/frontend/Cargo.toml
    cargo fmt --manifest-path crates/simulation/Cargo.toml
    cargo fmt --manifest-path crates/common/Cargo.toml
    prettier --write `fd --extension=md`

run-sim RESOLUTION:
    cargo run --manifest-path crates/simulation/Cargo.toml -- --resolution {{RESOLUTION}}

demo RESOLUTION:
    cargo run --manifest-path crates/simulation/Cargo.toml -- --presentation --resolution {{RESOLUTION}}

run-frontend: trunk-serve

# Run cargo check on changes
watch-check:
    watchexec --clear --restart --exts='rs,toml' -- cargo check --tests --examples

# Connect to websocket (locally)
subscribe-ws:
    websocat ws://127.0.0.1:32167/api/ws

# Trunk serve
trunk-serve:
    trunk serve crates/frontend/index.html

# Trunk serve on 0.0.0.0
trunk-serve-public:
    trunk serve --address=0.0.0.0 crates/frontend/index.html

# Serve mdbook
serve-book:
    mdbook serve --open book
