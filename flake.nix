{
  description = "";

  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.crane = {
    url = "github:ipetkov/crane";
    inputs = {
      nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, crane }:

    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      fenixPkgs = fenix.packages.${system};

      craneLib = crane.lib.${system}.overrideToolchain fenixPkgs.default.toolchain;

      src = craneLib.path ./.;
      cleanedSrc = craneLib.cleanCargoSource src;

      commonArgs = {
        src = cleanedSrc;

        buildInputs = with pkgs; [
          udev
          alsa-lib
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          clang
          mold
        ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      bevyGameBin =
        craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

      bevyGameClippy = craneLib.cargoClippy (commonArgs // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets -- --deny-warnings";
      });

      bevyGameFmt =
        craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });

      # TODO: have a bin derivation and an asset derivation, then a separate third to merge them
      bevyGame = pkgs.stdenv.mkDerivation {
        name = "my_bevy_game";
        nativeBuildInputs = [ bevyGameBin ];

        src = ./assets;

        buildPhase = ''
          mkdir -p $out/bin/assets
          cp ${bevyGameBin}/bin/my_bevy_game $out/bin/
          cp -r $src/* $out/bin/assets
        '';
      };

    in {
      checks.${system} = {
        inherit bevyGameBin bevyGameClippy bevyGameFmt;
      };

      packages.${system}.default = bevyGame;

      apps.${system}.default = {
        type = "app";
        program = "${bevyGame}/bin/my_bevy_game";
      };

      devShells.${system}.default = craneLib.devShell {
        inputsFrom = [ bevyGameBin ];

        packages = [ pkgs.just fenixPkgs.rust-analyzer ];

        LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
            with pkgs;
            lib.makeLibraryPath [
              vulkan-loader
              udev
              alsaLib
            ]
          }";
      };
    };
}
