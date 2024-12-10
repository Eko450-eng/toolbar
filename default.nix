{ pkgs ? import <nixpkgs> { } }:

pkgs.stdenv.mkDerivation {
  pname = "toolbar";
  version = "0.0.1";

  src = ./.; # Path to your Tauri app source code

  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
    glib.dev
    glib
    pnpm

    libthai
  ];

  buildInputs = with pkgs;[
    at-spi2-atk
    atkmm
    cairo
    rustup
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

  buildPhase = ''
    echo "Building Tauri app..."
    export RUST_BACKTRACE=1
    export HOME=$(mktemp -d)
    export PATH="$PATH:HOME/.cargo/bin" # Ensure rustup-installed cargo is available
    pnpm install
    pnpm run tauri build
  '';

  installPhase = ''
    echo "Installing Tauri app..."
    mkdir -p $out/bin
    cp src-tauri/target/release/your-app $out/bin/
  '';

  meta = with pkgs.lib; {
    description = "toolbar";
    license = licenses.mit; # Update with your license
    maintainers = [ ekrem.atmaca ];
    platforms = platforms.linux;
  };
}
