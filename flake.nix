{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.iay = naersk-lib.buildPackage {
            pname = "iay";
            root = ./.;
          };
          packages.default = packages.iay;

          # `nix run`
          apps.iay = flake-utils.lib.mkApp {
            drv = packages.iay;
          };
          apps.default = apps.iay;

          # `nix develop`
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [ cargo clippy rustc rustfmt pkg-config ];
            buildInputs = with pkgs; [ libgit2 ];
          };
        }
    );
}
