{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      inherit (nixpkgs) lib;

      systems = lib.systems.flakeExposed;

      forAllSystems = lib.genAttrs systems;

      nixpkgsFor = forAllSystems (
        system:
        import nixpkgs {
          inherit system;
        }
      );
    in
    {
      overlays.default = final: prev: {
        wasm-bruteforce = self.packages.${final.stdenv.system}.wasm-bruteforce;
      };

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        rec {
          wasm-bruteforce = default;
          default = pkgs.callPackage ./build.nix { };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              llvmPackages.lld

              openssl
              pkg-config
              gnumake
              pnpm_10
              wasm-bindgen-cli_0_2_106
              caddy # caddy file-server --listen :8000 --browse --root result
            ];
          };
        }
      );
    };
}
