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
        # 构建用最小工具链：仅 rustc + cargo + rust-std，
        # 避免把 rust-docs/rust-src/clippy/rustfmt（~1 GiB）拖进产物闭包
        rust = pkgs.rust-bin.stable.latest.minimal;
        # devShell 专用：附加 rust-src（rust-analyzer 需要）与 clippy/rustfmt
        rust-dev = pkgs.rust-bin.stable.latest.minimal.override {
          extensions = [
            "rust-src"
            "clippy"
            "rustfmt"
          ];
        };
        rust-wasm = pkgs.rust-bin.stable.latest.minimal.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        # 用统一的 rust-bin 工具链构建，避免与 rustPlatform 自带的 rustc 混用
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        };
      in
      {
        packages = {
          default = self.packages.${system}.apps;

          apps = rustPlatform.buildRustPackage {
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
            doCheck = false;
          };

          senitui = rustPlatform.buildRustPackage {
            pname = "senime";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "senime-tui"
            ];
            doCheck = false;
          };

          senils = rustPlatform.buildRustPackage {
            pname = "senime";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "senime-lsp"
            ];
            doCheck = false;
          };

          senienc = rustPlatform.buildRustPackage {
            pname = "senime";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "senime-encode"
            ];
            doCheck = false;
          };

          fcitx5-senime =
            let
              # 仅用于取得 vendored crate 源码；实际使用在 configurePhase 中
              # 复制到构建树内，避免产物内嵌 store 路径
              cargoVendorDir = pkgs.rustPlatform.importCargoLock {
                lockFile = ./Cargo.lock;
              };
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

                # 把 vendor 目录复制进构建树，而不是直接引用 store 路径；
                # 否则产物内嵌的源码路径会指向 /nix/store/...-cargo-vendor-dir，
                # 导致整个 vendor 目录（~350 MiB）进入运行时闭包
                cp -r ${cargoVendorDir} $PWD/cargo-vendor-dir
                chmod -R u+w $PWD/cargo-vendor-dir
                mkdir -p .cargo
                cat > .cargo/config.toml <<EOF
                [source.crates-io]
                replace-with = "vendored-sources"

                [source.vendored-sources]
                directory = "$PWD/cargo-vendor-dir"
                EOF

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
              rust-dev
              rust-analyzer-unwrapped
              rust-bindgen
            ];
            RUST_SRC_PATH = "${rust-dev}/lib/rustlib/src/rust/library";
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
              rust-dev
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
            RUST_SRC_PATH = "${rust-dev}/lib/rustlib/src/rust/library";
          };
        };

        devShells.default = self.devShells.${system}.rust;
      }
    );
}
