# This is a Justfile, a file that contains tasks and their descriptions.
# Windows Specific Settings

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Default task, runs the project in dev mode
default: run

# Run the project in development mode
[group('run')]
run: dev play

[group('run')]
[windows]
game $WGPU_BACKEND="dx12" $RUST_LOG="info,wgpu=error,naga=warn,project_z=debug":
    cargo run

[group('run')]
[private]
[windows]
play $WGPU_BACKEND="dx12" $RUST_LOG="info,wgpu=error,naga=warn,project_z=debug":
    cargo run -F dev

[group('run')]
[private]
[unix]
play:
    cargo run -F dev

# Build the project in development mode
[group('build')]
dev: build-cargo-dev

# Build the project using Cargo in development mode
[group('build')]
[private]
build-cargo-dev:
    cargo build -F dev

# Build the project in release mode
[group('build')]
release: build-cargo-release

# Build the project using Cargo in release mode
[group('build')]
[private]
build-cargo-release:
    cargo build --release

# Build the project in distribution mode
[group('build')]
dist: lint build-cargo-dist

# Build the project using Cargo in distribution mode
[group('build')]
[private]
build-cargo-dist:
    cargo build --profile distribution -F tracing/release_max_level_off -F log/release_max_level_off

# Linting and formatting checks
[group('lint')]
lint: fmt-check clippy

# Check the formatting of the code
[group('lint')]
fmt-check:
    cargo fmt --all -- --check

# Check the code for any linting issues
[group('lint')]
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Clean up the everything
[confirm("Are you sure you want to clean up everything?")]
[group('clean')]
clean: clean-blenvy-assets clean-cargo

# Clean up **only** the Blenvy specific assets
[confirm("Are you sure you want to clean up the Blenvy specific assets?")]
[group('clean')]
[windows]
clean-blenvy-assets:
    Remove-Item -Recurse -Force assets/blueprints
    Remove-Item -Recurse -Force assets/levels
    Remove-Item -Recurse -Force assets/materials

# Clean up **only** the Blenvy specific assets
[confirm("Are you sure you want to clean up the Blenvy specific assets?")]
[group('clean')]
[unix]
clean-blenvy-assets:
    rm -rf assets/blueprints
    rm -rf assets/levels
    rm -rf assets/materials

# Clean up **only** build artifacts using Cargo
[confirm("Are you sure you want to clean up the build artifacts using Cargo?")]
[group('clean')]
clean-cargo:
    cargo clean
