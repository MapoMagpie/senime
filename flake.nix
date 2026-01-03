{
  description = "Nix Flake for word-segment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      naersk,
      ...
    }:
    let
      inherit (nixpkgs) lib;

      genSystems = lib.genAttrs [
        "aarch64-linux"
        "x86_64-linux"
      ];

      pkgsFor =
        system:
        import nixpkgs {
          inherit system;

          overlays = [
            self.overlays.default
            rust-overlay.overlays.default
          ];
        };

      mkRustToolchain =
        pkgs:
        pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

    in
    {
      overlays.default =
        final: prev:
        let
          rust = mkRustToolchain final;
          rustPlatform = prev.makeRustPlatform {
            cargo = rust;
            rustc = rust;
          };
          naersk' = prev.callPackage naersk {
            cargo = rust;
            rustc = rust;
          };

          props = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          mkDate =
            longDate:
            (lib.concatStringsSep "-" [
              (builtins.substring 0 4 longDate)
              (builtins.substring 4 2 longDate)
              (builtins.substring 6 2 longDate)
            ]);

          builder = "naersk";
        in
        {
          wordseg =
            let
              version = props.package.version + "+date=" + (mkDate (self.lastModifiedDate or "19700101")) + "_" + (self.shortRev or "dirty");
            in
            prev.callPackage ./nix/default.nix {
              inherit version;
              inherit rustPlatform;
              builderName = builder;
              builder = naersk';
            };
        };

      packages = genSystems (
        system:
        let
          pkgs = pkgsFor system;
        in
        (self.overlays.default pkgs pkgs)
        // {
          default = self.packages.${system}.wordseg;
        }
      );

      apps = genSystems (
        system:
        let
          pkgs = pkgsFor system;
        in
        rec {
          wordseg = {
            type = "app";
            program = "${pkgs.wordseg}/bin/wordseg";
          };

          default = wordseg;
        }
      );

      devShells = genSystems (
        system:
        let
          pkgs = pkgsFor system;
          rust = mkRustToolchain pkgs;

        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rust
              rust-analyzer-unwrapped
              lldb
            ];
            RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
            shellHook = "exec zsh";
          };

        }
      );
    };

  # nixConfig = {
  #   extra-substituters = [ "https://cache.garnix.io" ];
  #   extra-trusted-public-keys = [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
  # };
}
