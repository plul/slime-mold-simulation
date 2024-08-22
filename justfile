_list:
    @just --list --unsorted

remedy: && fmt check
    cargo fix --allow-staged --allow-dirty
    cargo clippy --fix --allow-staged --allow-dirty

# Check project
check:
    bc-check
    RUSTDOCFLAGS='-Dwarnings' cargo doc --no-deps
    cargo nextest run
    nix flake show
    cargo udeps
    cargo outdated --depth=1

# Format code
fmt:
    bc-fmt

run-sim RESOLUTION:
    cargo run --manifest-path crates/simulation/Cargo.toml -- --resolution {{ RESOLUTION }}

demo RESOLUTION:
    cargo run --manifest-path crates/simulation/Cargo.toml -- --presentation --resolution {{ RESOLUTION }}

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
