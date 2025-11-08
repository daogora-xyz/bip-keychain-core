{
  description = "BIP-Keychain: Multi-schema semantic hierarchical key derivation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        # Native dependencies required by alkali (libsodium bindings)
        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
        ];

        buildInputs = with pkgs; [
          libsodium  # Required by alkali crate (BLAKE2b support)
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            inherit nativeBuildInputs buildInputs;

            # Enable tests during build
            doCheck = true;

            meta = with pkgs.lib; {
              description = cargoToml.package.description;
              homepage = cargoToml.package.repository;
              license = licenses.bsd2;
              maintainers = [ ];
              platforms = platforms.unix;
            };
          };

          # Binary-only package (smaller, faster to build for users who just want CLI)
          bip-keychain-bin = pkgs.rustPlatform.buildRustPackage {
            pname = "${cargoToml.package.name}-bin";
            version = cargoToml.package.version;

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            inherit nativeBuildInputs buildInputs;

            # Only build the binary, skip library
            cargoBuildFlags = [ "--bin" "bip-keychain" ];

            doCheck = true;

            meta = with pkgs.lib; {
              description = "${cargoToml.package.description} (binary only)";
              homepage = cargoToml.package.repository;
              license = licenses.bsd2;
              mainProgram = "bip-keychain";
            };
          };
        };

        # Development shell with all tools
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            # Rust development tools
            cargo-edit          # cargo add, cargo rm, cargo upgrade
            cargo-watch         # cargo watch -x test
            cargo-expand        # expand macros for debugging
            cargo-audit         # security vulnerability scanning
            cargo-outdated      # check for outdated dependencies

            # Additional development tools
            git                 # version control
            just                # command runner (alternative to make)

            # Documentation
            mdbook              # for future tutorial generation
          ]);

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          LIBSODIUM_LIB_DIR = "${pkgs.libsodium}/lib";
          LIBSODIUM_INCLUDE_DIR = "${pkgs.libsodium}/include";

          # Helpful shell hook
          shellHook = ''
            echo "🔐 BIP-Keychain development environment"
            echo ""
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo "libsodium: ${pkgs.libsodium.version}"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run tests"
            echo "  cargo run -- <args>  - Run the CLI"
            echo "  cargo watch -x test  - Watch mode for tests"
            echo "  cargo audit          - Security audit"
            echo ""
            echo "Examples:"
            echo "  cargo run -- generate-seed 24"
            echo "  cargo run -- derive examples/person-identity.json"
            echo ""
          '';
        };

        # CI-focused shell (minimal, fast to build)
        devShells.ci = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "CI environment ready"
          '';
        };

        # Checks (run with: nix flake check)
        checks = {
          # Run cargo test
          test = pkgs.rustPlatform.buildRustPackage {
            pname = "${cargoToml.package.name}-test";
            version = cargoToml.package.version;

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            inherit nativeBuildInputs buildInputs;

            buildPhase = ''
              cargo test --all-features
            '';

            installPhase = ''
              mkdir -p $out
              echo "Tests passed" > $out/test-result
            '';
          };

          # Run clippy
          clippy = pkgs.rustPlatform.buildRustPackage {
            pname = "${cargoToml.package.name}-clippy";
            version = cargoToml.package.version;

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            inherit nativeBuildInputs buildInputs;

            buildPhase = ''
              cargo clippy --all-features -- -D warnings
            '';

            installPhase = ''
              mkdir -p $out
              echo "Clippy passed" > $out/clippy-result
            '';
          };

          # Check formatting
          fmt = pkgs.runCommand "check-fmt" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo fmt -- --check
            mkdir -p $out
            echo "Formatting check passed" > $out/fmt-result
          '';
        };

        # Apps (run with: nix run)
        apps = {
          default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/bip-keychain";
          };

          bip-keychain = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/bip-keychain";
          };

          # Example: generate seed
          generate-seed = {
            type = "app";
            program = pkgs.writeShellScript "generate-seed" ''
              ${self.packages.${system}.default}/bin/bip-keychain generate-seed "''${1:-24}"
            '';
          };
        };

        # Formatter (run with: nix fmt)
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
