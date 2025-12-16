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
          let
            runtimeDeps = with pkgs;[ libxkbcommon vulkan-loader ];
            rustDeps = with pkgs; [
              cargo
              cargo-binutils
              cargo-llvm-cov
              rustc
              rustfmt
              rustPackages.clippy
              rust-analyzer
            ];
            sysDeps = with pkgs; [
              wayland
              udev
              alsa-lib
              pkg-config
            ];
          in
          with pkgs; mkShell {
            buildInputs = [
              # replace rust linker with lld
              lld
              clang
              # utilities
              pre-commit
            ] ++ sysDeps ++ rustDeps ++ runtimeDeps;
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            shellHook = ''
              LD_LIBRARY_PATH=${lib.makeLibraryPath runtimeDeps}
            '';
          };
      }
    );
}
