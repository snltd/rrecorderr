dev-api:
    cd api && cargo run

dev-ui:
    cd frontend && npm ci && npm run dev

build:
    cd frontend && npm ci && npm run build
    cargo build --release
