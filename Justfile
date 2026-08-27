# Justfile - Terminal Sand Toy (Qwen 3.6 35B Test Project)

# Default command
default: help

# Show help message
@help:
    @just --list

# Build the simulator in release mode
build:
    cd simulator && cargo build --release

# Run the simulator in debug mode
run:
    cd simulator && cargo run

# Run the simulator in release mode (faster)
release:
    cd simulator && cargo run --release

# Update dependencies and upgrade all crates at once
update:
    cd simulator && cargo update

# Run clippy lints with warnings
lint:
    cd simulator && cargo clippy -- -Dwarnings

# Check formatting without making changes
fmt-check:
    cd simulator && cargo fmt -- --check

# Fix formatting issues automatically
fmt:
    cd simulator && cargo fmt

# Remove build artifacts and compiled code
clean:
    cd simulator && cargo clean

# Check simulator compiles without linking (fast validation)
check:
    cd simulator && cargo check

# Run all tests in CI mode to avoid parallelism errors
ci-test:
    cd simulator && cargo test -- --test-threads=1

# Clone the sandbox repository fresh from github into a clean directory
sandbox:
    git clone https://github.com/meltingscales/sandbox.git simulator && cd simulator && cp ../justfile . 2>/dev/null || true

# Show current Rust and Cargo versions for compatibility checks
info:
    rustc --version && cargo --version && tput cols

