{
    description = "Basic rust template";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        crane = { 
            url = "github:ipetkov/crane"; 
            inputs.nixpkgs.follows = "nixpkgs"; 
        };
        fenix = { 
            url = "github:nix-community/fenix"; 
            inputs.nixpkgs.follows = "nixpkgs"; 
            inputs.rust-analyzer-src.follows = ""; 
        };
    };

    outputs = inputs @ { self, nixpkgs, crane, fenix, ... }:
    let 
        forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" ];
        nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in {
        packages = forAllSystems (system: {
            elnafo = let
                pkgs = nixpkgsFor.${system};
                craneLib = crane.lib.${system};
            in craneLib.buildPackage {
                src = craneLib.cleanCargoSource (craneLib.path ./.);
                strictDeps = true;

                buildInputs = [];
            };

            default = self.packages.${system}.elnafo;
        });

        checks = forAllSystems (system: { 
            inherit (self.packages.${system}.elnafo);

            elnafo-fmt = let craneLib = crane.lib.${system}; in craneLib.cargoFmt { 
                src = craneLib.cleanCargoSource (craneLib.path ./.);
            };
        });

        apps = forAllSystems (system: {
            default = {
                type = "app";
                program = "${self.packages.${system}.elnafo}/bin/elnafo"; 
            };
        });

        devShells = forAllSystems (system: {
            default = let pkgs = nixpkgsFor.${system}; in pkgs.mkShell {
                nativeBuildInputs = [ 
                    fenix.packages.${system}.complete.toolchain 
                ];
            };
            elnafo = crane.lib.${system}.devShell {
                checks = self.checks.${system};

                packages = [];
            };
        });
    };

}
