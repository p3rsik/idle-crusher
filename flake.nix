{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell =
          let runtimeDeps = with pkgs;[ libxkbcommon vulkan-loader ];
          in
          with pkgs; mkShell {
            buildInputs = [
              # rust packages
              cargo
              cargo-binutils
              cargo-llvm-cov
              rustc
              rustfmt
              rustPackages.clippy
              rust-analyzer
              # replace rust linker with lld
              lld
              clang
              # system libs and binaries
              wayland
              udev
              alsa-lib
              pkg-config
              # utilities
              pre-commit
            ] ++ runtimeDeps;
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            shellHook = ''
              LD_LIBRARY_PATH=${lib.makeLibraryPath runtimeDeps}
            '';
          };
      }
    );
}
