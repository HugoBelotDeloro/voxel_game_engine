{
  description = "";

  inputs.utils.url = "github:numtide/flake-utils";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, fenix }:

    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        nativeBuildInputs = with pkgs; [
          pkg-config
          llvmPackages.bintools
        ];
        buildInputs = with pkgs; [
          fenix.packages.x86_64-linux.stable.toolchain
          udev
          alsa-lib
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];
      in {
        devShell = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${with pkgs; lib.makeLibraryPath [
            vulkan-loader
          ]}";
        };
      });
}
