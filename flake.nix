{
  description = "A development environment for the Flerry compiler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      flerry = pkgs.rustPlatform.buildRustPackage {
        pname = "flerry";
        version = "0.0.1";
        src = ./flerry;
        cargoLock = {
          lockFile = ./flerry/Cargo.lock;
        };
      };
    in
    {
      packages.${system} = {
        default = flerry;
      };

      devShells.${system} = {
        default = pkgs.mkShell {
          name = "flerry-dev";
          packages = with pkgs; [
            # Rust toolchain
            rust-analyzer
            rustup
            self.packages.${system}.default
          ];
        };
      };
    };
}