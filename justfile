app_db_file := data_local_directory() / 'power-cube/db.sqlite'

run:
    pnpm run tauri dev

run-migrations:
    bash apply_migrations.sh {{ app_db_file }}

build:
    # There's a linuxdeploy bug that causes Appimage bundling to fail
    # https://github.com/linuxdeploy/linuxdeploy/issues/272
    ARCH={{ arch() }} NO_STRIP=true pnpm run tauri build

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
    pnpm run test
    cargo test --manifest-path lib/tnoodle-rs/Cargo.toml
    cargo test --manifest-path src-tauri/Cargo.toml

