let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
    glib.dev
    glib

    libthai
  ];

  buildInputs = with pkgs;[
    at-spi2-atk
    atkmm
    cairo
    cargo # Rust's Package Manager
    cargo-xwin
    fuse3
    gdk-pixbuf
    glib
    glib.dev
    gtk3
    gtk4
    harfbuzz
    librsvg
    libsoup_3
    nodejs # Node.js
    openssl # SSL Libraries
    pango
    rustc # Rust Compiler
    squashfsTools
    webkitgtk_4_1
    xdotool
  ];

  shellHook = ''
    export GSETTINGS_SCHEMA_DIR=/nix/store/6r6rzv2v3x8mh2ki391gvc9y9954yzlg-glib-2.80.4-dev/share/glib-2.0/schemas
  '';
}
