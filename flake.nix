{
  description = "ProtoChess";
  
  # Use stable nixpkgs
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
      # Dev shell
      devShells.${system}.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rust-analyzer
          ];

          # Make Rust SRC avialable for rust-analyzer
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };

      # Package build
      packages.default = pkgs.rustPlatform.buildRustPackage {
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };
    };
}
