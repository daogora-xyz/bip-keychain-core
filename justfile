# BIP-Keychain Development Commands
# Run `just` or `just --list` to see all available commands

# Default: show all available commands
default:
  @just --list

# Build the project
build:
  cargo build

# Build release binary
build-release:
  cargo build --release

# Run all tests
test:
  cargo test

# Run tests with output
test-verbose:
  cargo test -- --nocapture

# Run specific test
test-one TEST:
  cargo test {{TEST}}

# Run tests for specific module
test-module MODULE:
  cargo test {{MODULE}}::

# Run tests with coverage (requires cargo-tarpaulin)
test-coverage:
  cargo tarpaulin --out Html --output-dir coverage

# Watch tests (auto-run on file changes)
watch:
  cargo watch -x test

# Watch and run specific command
watch-cmd CMD:
  cargo watch -x "{{CMD}}"

# Check code without building
check:
  cargo check

# Run clippy lints
lint:
  cargo clippy --all-features -- -D warnings

# Format code
fmt:
  cargo fmt

# Check formatting without modifying files
fmt-check:
  cargo fmt -- --check

# Security audit
audit:
  cargo audit

# Check for outdated dependencies
outdated:
  cargo outdated

# Update dependencies
update:
  cargo update

# Clean build artifacts
clean:
  cargo clean

# Generate seed phrase (default: 24 words)
generate-seed WORDS="24":
  cargo run -- generate-seed {{WORDS}}

# Derive key from example entity
derive EXAMPLE:
  cargo run -- derive examples/{{EXAMPLE}}.json

# Derive key from person-identity example
derive-person:
  @just derive person-identity

# Derive key from GitHub repo example
derive-github:
  @just derive github-repo

# Derive key from API service example
derive-api:
  @just derive api-service-key

# Derive key from IoT device example
derive-iot:
  @just derive iot-device-key

# Run comprehensive demo script
demo:
  ./examples/comprehensive-demo.sh

# Run batch derivation script
batch-derive:
  ./examples/batch-derive-keys.sh

# Run SSH provisioning example
ssh-provision:
  ./examples/ssh-provision-servers.sh

# Run key rotation workflow example
key-rotation:
  ./examples/key-rotation-workflow.sh

# Run backup and recovery example
backup-recovery:
  ./examples/backup-and-recovery.sh

# View examples documentation
examples:
  cat examples/EXAMPLES.md

# Build documentation
docs:
  cargo doc --no-deps --open

# Build documentation with private items
docs-all:
  cargo doc --no-deps --document-private-items --open

# Run all CI checks (test, lint, format check)
ci: fmt-check lint test
  @echo "✅ All CI checks passed!"

# Pre-commit hook (format, lint, test)
pre-commit: fmt lint test
  @echo "✅ Ready to commit!"

# Install the CLI tool locally
install:
  cargo install --path .

# Uninstall the CLI tool
uninstall:
  cargo uninstall bip-keychain

# Show project status
status:
  @echo "📊 Project Status:"
  @echo ""
  @echo "Git branch: $(git branch --show-current)"
  @echo "Git status:"
  @git status --short
  @echo ""
  @echo "Cargo version: $(cargo --version)"
  @echo "Rust version: $(rustc --version)"
  @echo ""
  @echo "Tests:"
  @cargo test --quiet 2>&1 | grep -E "test result|running" || true

# Nix: Build with Nix flake
nix-build:
  nix build

# Nix: Run checks (tests, clippy, fmt)
nix-check:
  nix flake check

# Nix: Update flake inputs
nix-update:
  nix flake update

# Nix: Show flake metadata
nix-info:
  nix flake metadata

# Nix: Enter development shell
nix-shell:
  nix develop

# View README
readme:
  cat README.md

# View roadmap
roadmap:
  cat ROADMAP.md

# View TODO list
todo:
  cat TODO.md

# View review checklist
checklist:
  cat REVIEW-CHECKLIST.md

# Count lines of code
loc:
  @echo "Lines of Rust code:"
  @find src -name "*.rs" | xargs wc -l | tail -1
  @echo ""
  @echo "Lines of test code:"
  @find tests -name "*.rs" | xargs wc -l | tail -1 || echo "0"
  @echo ""
  @echo "Total lines (including comments):"
  @find src tests -name "*.rs" | xargs cat | wc -l

# Show dependency tree
deps:
  cargo tree

# Expand macros for debugging
expand:
  cargo expand

# Benchmark (if benchmarks exist)
bench:
  cargo bench

# Run quick development cycle (format, build, test)
dev: fmt build test
  @echo "✅ Development cycle complete!"

# Initialize environment (for new developers)
init:
  @echo "🔐 Initializing BIP-Keychain development environment..."
  @echo ""
  @echo "1. Installing dependencies..."
  @cargo fetch
  @echo ""
  @echo "2. Running initial build..."
  @cargo build
  @echo ""
  @echo "3. Running tests..."
  @cargo test --quiet
  @echo ""
  @echo "✅ Environment ready!"
  @echo ""
  @echo "Next steps:"
  @echo "  - Read README.md: just readme"
  @echo "  - View examples: just examples"
  @echo "  - Generate seed: just generate-seed 24"
  @echo "  - Derive key: just derive person-identity"

# Show helpful development tips
tips:
  @echo "💡 BIP-Keychain Development Tips:"
  @echo ""
  @echo "Common workflows:"
  @echo "  just dev              # Quick dev cycle (fmt, build, test)"
  @echo "  just watch            # Auto-run tests on file changes"
  @echo "  just ci               # Run all CI checks locally"
  @echo "  just pre-commit       # Run before committing"
  @echo ""
  @echo "Key derivation:"
  @echo "  just generate-seed    # Generate BIP-39 seed phrase"
  @echo "  just derive-person    # Derive from person identity"
  @echo "  just derive-github    # Derive from GitHub repo"
  @echo "  just demo             # Run comprehensive demo"
  @echo ""
  @echo "Documentation:"
  @echo "  just examples         # View all example entities"
  @echo "  just roadmap          # View project roadmap"
  @echo "  just checklist        # View review checklist"
  @echo ""
  @echo "Nix users:"
  @echo "  just nix-build        # Build with Nix"
  @echo "  just nix-check        # Run Nix flake checks"
  @echo ""
  @echo "Run 'just' to see all available commands"
