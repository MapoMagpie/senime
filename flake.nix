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

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in
      {
        packages = {
          default = self.packages.${system}.senime-tui;

          senime-tui = pkgs.rustPlatform.buildRustPackage {
            pname = "senime-tui";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [ "-p" "senime-tui" ];
            nativeBuildInputs = [ rust ];
            doCheck = false;
          };

          senime-lsp = pkgs.rustPlatform.buildRustPackage {
            pname = "senime-lsp";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [ "-p" "senime-lsp" ];
            nativeBuildInputs = [ rust ];
            doCheck = false;
          };

          senime-encode = pkgs.rustPlatform.buildRustPackage {
            pname = "senime-encode";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [ "-p" "senime-encode" ];
            nativeBuildInputs = [ rust ];
            doCheck = false;
          };

          senime-fcitx5 = pkgs.stdenv.mkDerivation {
            pname = "senime-fcitx5";
            version = "0.0.1";
            src = ./.;

            nativeBuildInputs = [
              rust
              pkgs.cmake
              pkgs.ninja
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

            preConfigure = ''
              export CARGO_TARGET_DIR=$PWD/cargo-target
              cargo build --release -p senime-fcitx5
            '';

            cmakeFlags = [
              "-GNinja"
            ];

            postInstall = ''
              mkdir -p $out/lib/fcitx5
              mkdir -p $out/share/fcitx5/addon
              mkdir -p $out/share/fcitx5/inputmethod
              mkdir -p $out/share/icons/hicolor/scalable/apps
              cp senime.so $out/lib/fcitx5/
              cp ${./senime-fcitx5/data/fcitx5/addon/senime.conf} $out/share/fcitx5/addon/
              cp ${./senime-fcitx5/data/fcitx5/inputmethod/senime.conf} $out/share/fcitx5/inputmethod/
              cp ${./senime-fcitx5/data/fcitx5/icon/fcitx-senime-cn.svg} $out/share/icons/hicolor/scalable/apps/
              cp ${./senime-fcitx5/data/fcitx5/icon/fcitx-senime-en.svg} $out/share/icons/hicolor/scalable/apps/
            '';
          };
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rust
            pkgs.rust-analyzer-unwrapped
            pkgs.rust-bindgen
          ];
          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
        };
      });
}
