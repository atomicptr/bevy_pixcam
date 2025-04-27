# bevy_pixcam

A simple camera plugin for the Bevy game engine (0.15+), to help with the use of
pixel-art sprites.

This crates provides a plugin to automatically configure Bevy's
`Camera2d`. It works by setting the camera to an integer scaling
factor (using Bevy's `ScalingMode::WindowSize`), and automatically updating
the zoom level so that the specified target resolution fills as much of the
sceen as possible.

The plugin can also automatically set and resize the viewport of the camera
to match the target resolution.

**Note**: This is a hard fork of the great [bevy_pixel_camera](https://github.com/drakmaniso/bevy_pixel_camera)

## Comparison with other methods

There is two main methods to render pixel-art games: upscale each sprite
independently, or render everything to an offscreen texture and only upscale
this texture. This crate use the first method. There is advantages and
drawbacks to both approaches.

Advantages of the "upscale each sprite independently" method (i.e. this
crate):

- allows for smoother scrolling and movement of sprites, if you're willing
  to temporarily break the alignment on virtual pixels (this would be even
  more effective with a dedicated upscaling shader);
- easier to mix pixel-art and high resolution graphics (for example for
  text, particles or effects).

Advantages of the "offscreen texture" method:

- always ensure perfect alignment on virtual pixels (authentic "retro"
  look);
- may be more efficient (in most cases, the difference is probably
  negligible on modern computers).

## How to use

Note that Bevy uses linear sampling by default for textures, which is not
what you want for pixel art. The easiest way to change this is to configure
Bevy's default plugins with `ImagePlugin::default_nearest()`.

Also note that if either the width or the height of your sprite is not
divisible by 2, you may need to change the anchor of the sprite (which is at
the center by default), otherwise it won't be aligned with virtual pixels.

```rust
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_pixcam::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));

    commands.spawn(
        Sprite {
            image: asset_server.load("mire-64x64.png"),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        });
}
```

A small example is included in the crate. Run it with:

```console
cargo run --example flappin
```

## Bevy versions supported

`bevy_pixcam` is not using semver, every release uses the same major and minor versions as `bevy` while the patch part is reserved
for all kinds of changes, be it bug fixes or feature updates.

| bevy   | bevy_pixcam | branch |
|--------|-------------|--------|
| 0.16.x | 0.16.x      | master |
| 0.15.x | 0.15.x      | 0.15   |

## Migrating from bevy_pixel_camera

- Replace all instances of the string `bevy_pixel_camera` with `bevy_pixcam`

## License

MIT
