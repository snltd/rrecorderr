# Start API and frontend on local host
[parallel]
run-dev:
    run-dev-api run-dev-ui

# Start API on local host
run-dev-api:
    cd crates/api && cargo run

# Start frontend on local host
run-dev-ui:
    cd frontend && npm ci && npm run dev

build:
    cd frontend && npm ci && npm run build
    cargo build --release

# Create SQLX fixtures - run after changing DB schema
regen-sqlx:
    cd crates/api && cargo sqlx prepare
    cd crates/assay && cargo sqlx prepare
