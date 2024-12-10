id: de.wipdesign.toolbar

runtime: org.gnome.Platform
runtime-version: '46'
sdk: org.gnome.Sdk

command: toolbar
finish-args:
  - --socket=wayland # Permission needed to show the window
  - --socket=fallback-x11 # Permission needed to show the window
  - --device=dri # OpenGL, not necessary for all projects
  - --share=ipc

modules:
  - name: binary
    buildsystem: simple
    sources:
      - type: file
        url: https://github.com/Eko450-eng/toolbar/releases/download/app-v0.1.0/toolbar_0.1.0_amd64.deb
        sha256: c0cfe6dafc813c084d9e5a6c896a4efcd023c03c27cf5323cd4e0f3f846a49e9
        only-arches: [x86_64] #This source is only used on x86_64 Computers
        # This path points to the binary file which was created in the .deb bundle.
        # Tauri also creates a folder which corresponds to the content of the unpacked .deb.
    build-commands:
      - ar -x *.deb
      - tar -xf data.tar.gz
      - 'install -Dm755 ./usr/bin/toolbar /app/bin/toolbar'
      - install -Dm644 ./usr/share/applications/toolbar.desktop /app/share/applications/de.wipdesign.toolbar.desktop
      - install -Dm644 ./usr/share/icons/hicolor/128x128/apps/toolbar.png /app/share/icons/hicolor/128x128/apps/de.wipdesign.toolbar.png
      - install -Dm644 ./usr/share/icons/hicolor/32x32/apps/toolbar.png /app/share/icons/hicolor/32x32/apps/de.wipdesign.toolbar.png
      - install -Dm644 ./usr/share/icons/hicolor/256x256@2/apps/toolbar.png /app/share/icons/hicolor/256x256@2/apps/de.wipdesign.toolbar.png
      #- install -Dm644 de.wipdesign.toolbar.metainfo.xml /app/share/metainfo/de.wipdesign.toolbar.rosary.metainfo.xml

flatpak-builder --repo=./toolbar build-dir manifest.yaml
flatpak --user remote-add --no-gpg-verify toolbar ./toolbar/
flatpak --user install toolbar de.wipdesign.toolbar
flatpak run de.wipdesign.toolbar
