{
  description = "";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.crane = {
    url = "github:ipetkov/crane";
    inputs = {
      flake-utils.follows = "flake-utils";
      nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, crane }:

    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        craneLib = crane.lib.${system}.overrideToolchain
          fenix.packages.${system}.stable.toolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          buildInputs = with pkgs; [
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
          ];

          nativeBuildInputs = with pkgs; [ pkg-config llvmPackages.bintools ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        bevyGame =
          craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

        bevyGameClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny-warnings";
        });

        bevyGameFmt =
          craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });

      in {
        # checks = {
        #   inherit bevyGame bevyGameClippy bevyGameFmt;
        # };

        # packages.default = bevyGame;

        # apps.default = flake-utils.lib.mkApp {
        #   drv = bevyGame;
        # };

        devShell = craneLib.devShell {
          inputsFrom = [ bevyGame ];

          packages = [ pkgs.just ];

          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
              with pkgs;
              lib.makeLibraryPath [
                vulkan-loader
                # udev and alsaLib might be needed someday
              ]
            }";
        };
      });
}
