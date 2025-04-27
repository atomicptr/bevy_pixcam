{ pkgs, ... }:

{
  packages = with pkgs; [
    udev
    alsa-lib-with-plugins
    vulkan-loader

    # x11
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr

    # wayland
    libxkbcommon
    wayland
  ];

  env = {
    WINIT_UNIX_BACKEND = "wayland";
    RUST_BACKTRACE = "1"; # "full" for even more
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.libxkbcommon
      pkgs.vulkan-loader
      pkgs.wayland
    ];
  };
}
