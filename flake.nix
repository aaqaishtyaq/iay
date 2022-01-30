{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
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
          defaultPackage = packages.iay;

          # `nix run`
          apps.iay = flake-utils.lib.mkApp {
            drv = packages.iay;
          };
          defaultApp = apps.iay;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo ];
          };
        }
    );
}
