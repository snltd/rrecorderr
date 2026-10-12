[parallel]
run-dev: run-dev-api run-dev-ui

run-dev-api:
    cd crates/api && cargo run

run-dev-ui:
    cd frontend && npm ci && npm run dev

build:
    cd frontend && npm ci && npm run build
    cargo build --release
