{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    udev
    alsa-lib-with-plugins
    vulkan-loader
    vulkan-tools
    libudev-zero
    # x11
    libxkbcommon
    libx11
    libxcursor
    libxi
    libxrandr
    # wayland
    wayland
  ];

  WINIT_UNIX_BACKEND = "wayland";
  RUST_BACKTRACE = "1";

  shellHook = ''
    export LD_LIBRARY_PATH="${
      pkgs.lib.makeLibraryPath [
        pkgs.libxkbcommon
        pkgs.vulkan-loader
        pkgs.wayland
        pkgs.libx11
        pkgs.libxcursor
        pkgs.libxi
      ]
    }:$LD_LIBRARY_PATH"
  '';
}
