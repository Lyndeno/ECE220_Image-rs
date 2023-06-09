{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    pre-commit-hooks-nix,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.ece-img = naersk-lib.buildPackage {
        pname = "ece-img";
        root = ./.;
        doDoc = true;
      };
      packages.default = packages.ece-img;

      apps.ece-img = utils.lib.mkApp {
        drv = packages.ece-img;
      };
      apps.default = apps.ece-img;

      formatter = pkgs.alejandra;

      devShells.default = let
        pre-commit-format = pre-commit-hooks-nix.lib.${system}.run {
          src = ./.;

          hooks = {
            alejandra.enable = true;
            rustfmt.enable = true;
          };
        };
      in
        pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustc cargo rustfmt clippy];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          shellHook = ''
            ${pre-commit-format.shellHook}
          '';
        };
    });
}
