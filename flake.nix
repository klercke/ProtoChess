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
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rust-analyzer
          clippy
        ];

        # Make Rust SRC avialable for rust-analyzer
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };

      # Package build
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "ProtoChess";
        src = pkgs.lib.cleanSource ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
  };
}
