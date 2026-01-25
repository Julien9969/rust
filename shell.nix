{ pkgs ? import <nixpkgs> {} }:

let
  myPackages = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    pkg-config
    openssl

    # X11
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libxcb
    libxkbcommon

    # Wayland
    wayland
    wayland-protocols
  ];
in

pkgs.mkShell {
  buildInputs = myPackages;

  shellHook = ''
    export TMPDIR=/tmp
    export WINIT_UNIX_BACKEND=x11   # comment this to allow Wayland
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath myPackages)}"
    echo "🦀 Rust GUI dev shell ready (iced/winit)"
  '';
}
