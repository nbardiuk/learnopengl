let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  base = nixpkgs.latest.rustChannels.stable;
  rust = base.rust.override {
    extensions = [
      "rls-preview"
      "rust-analysis"
      "rust-src"
      "rust-std"
      "rustfmt-preview"
    ];
  };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";

    buildInputs = [
      rust

      cmake
      xorg.libX11
      xorg.libXrandr
      xorg.libXinerama
      xorg.libXcursor
      xorg.libXi
      xorg.libXext
    ];

    RUST_SRC_PATH="${rust}/lib/rustlib/src/rust/src";

    GLFW_LIB_DIR="${glfw}/lib";

    LD_LIBRARY_PATH="${lib.makeLibraryPath [ libGL]}";
  }
