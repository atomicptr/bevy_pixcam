{ pkgs, ... }:

{
  packages = with pkgs; [
    udev
    alsa-lib-with-plugins
    vulkan-loader
    vulkan-tools
    libudev-zero

    # x11
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr

    # wayland
    wayland
  ];

  env = {
    WINIT_UNIX_BACKEND = "wayland";
    RUST_BACKTRACE = "1"; # "full" for even more
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.libxkbcommon
      pkgs.vulkan-loader
      pkgs.wayland
      pkgs.xorg.libX11
      pkgs.xorg.libXcursor
      pkgs.xorg.libXi
    ];
  };
}
