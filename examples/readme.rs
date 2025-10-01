use bevy::{prelude::*, sprite::Anchor};
use bevy_pixcam::{PixelCameraPlugin, PixelViewport, PixelZoom};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("mire-64x64.png"),
            ..Default::default()
        },
        Anchor::BOTTOM_LEFT,
    ));
}
