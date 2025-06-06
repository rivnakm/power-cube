run:
    pnpm run tauri dev

build:
    pnpm run tauri build

format:
    pnpm exec prettier . --write
    cargo fmt --verbose --manifest-path lib/tnoodle-rs/Cargo.toml
    cargo fmt --verbose --manifest-path src-tauri/Cargo.toml

lint:
    -pnpm exec eslint .
    -cargo clippy --manifest-path lib/tnoodle-rs/Cargo.toml
    -cargo clippy --manifest-path src-tauri/Cargo.toml

lint-fix:
    pnpm exec eslint . --fix
    cargo clippy --manifest-path lib/tnoodle-rs/Cargo.toml --fix
    cargo clippy --manifest-path src-tauri/Cargo.toml --fix

clean:
    rm -rf dist
    cargo clean --manifest-path lib/tnoodle-rs/Cargo.toml
    cargo clean --manifest-path src-tauri/Cargo.toml

test:
    cargo test --manifest-path lib/tnoodle-rs/Cargo.toml
    cargo test --manifest-path src-tauri/Cargo.toml

