{
  description = "Senime — 码表输入法引擎";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
        rust-wasm = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "rust-src" ];
        };
      in
      {
        packages = {
          default = self.packages.${system}.apps;

          apps = pkgs.rustPlatform.buildRustPackage {
            pname = "senime";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "senime-tui"
              "-p"
              "senime-lsp"
              "-p"
              "senime-encode"
            ];
            nativeBuildInputs = [ rust ];
            doCheck = false;
          };

          senitui = pkgs.rustPlatform.buildRustPackage {
            pname = "senime";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "senime-tui"
            ];
            nativeBuildInputs = [ rust ];
            doCheck = false;
          };

          senils = pkgs.rustPlatform.buildRustPackage {
            pname = "senime";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "senime-lsp"
            ];
            nativeBuildInputs = [ rust ];
            doCheck = false;
          };

          senienc = pkgs.rustPlatform.buildRustPackage {
            pname = "senime";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "senime-encode"
            ];
            nativeBuildInputs = [ rust ];
            doCheck = false;
          };

          fcitx5-senime =
            let
              cargoVendorDir = pkgs.rustPlatform.importCargoLock {
                lockFile = ./Cargo.lock;
              };
              cargoConfig = pkgs.writeText "cargo-config.toml" ''
                [source.crates-io]
                replace-with = "vendored-sources"

                [source.vendored-sources]
                directory = "${cargoVendorDir}"
              '';
            in
            pkgs.stdenv.mkDerivation {
              pname = "fcitx5-senime";
              version = "0.0.1";
              src = ./.;

              nativeBuildInputs = [
                rust
                pkgs.cmake
                pkgs.pkg-config
                pkgs.fcitx5
                pkgs.kdePackages.extra-cmake-modules
                pkgs.gettext
                pkgs.libclang
                pkgs.cargo
              ];

              buildInputs = [
                pkgs.fcitx5
              ];

              # 跳过默认的 configurePhase，手动处理目录结构
              dontUseCmakeConfigure = true;

              configurePhase = ''
                runHook preConfigure

                # src 解压后的目录（含 Cargo.toml 的 workspace 根）
                export CARGO_TARGET_DIR=$PWD/cargo-target
                mkdir -p .cargo
                cp ${cargoConfig} .cargo/config.toml

                # CMakeLists.txt 在 senime-fcitx5/ 子目录
                mkdir -p build
                cd build
                cmake ../senime-fcitx5 \
                  -DCMAKE_INSTALL_PREFIX=$out

                runHook postConfigure
              '';

              buildPhase = ''
                runHook preBuild
                cmake --build .
                runHook postBuild
              '';

              installPhase = ''
                runHook preInstall
                cmake --install .
                runHook postInstall
              '';
            };
        };

        devShells = {
          rust = pkgs.mkShell {
            name = "senime-rust";
            packages = with pkgs; [
              rust
              rust-analyzer-unwrapped
              rust-bindgen
            ];
            RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
          };

          wasm = pkgs.mkShell {
            name = "senime-wasm";
            packages = with pkgs; [
              rust-wasm
              wasm-pack
              wasm-bindgen-cli
              nodejs_24
              typescript-language-server
            ];
          };

          cpp = pkgs.mkShell {
            name = "senime-cpp";
            packages = with pkgs; [
              llvmPackages.clang
              gcc
              cmake
              pkg-config
              fcitx5
              kdePackages.extra-cmake-modules
              gettext
            ];
          };

          all = pkgs.mkShell {
            name = "senime-all";
            packages = with pkgs; [
              rust
              rust-analyzer-unwrapped
              rust-bindgen
              rust-wasm
              wasm-pack
              wasm-bindgen-cli
              llvmPackages.clang
              gcc
              cmake
              pkg-config
              fcitx5
              kdePackages.extra-cmake-modules
              gettext
              nodejs_24
              typescript-language-server
            ];
            RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
          };
        };

        devShells.default = self.devShells.${system}.rust;
      }
    );
}
