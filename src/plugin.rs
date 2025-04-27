use bevy::prelude::{App, Plugin, PostUpdate};
use bevy::{ecs::schedule::IntoScheduleConfigs, render::camera};

use crate::pixel_zoom_system;

/// Provides the camera system.
pub struct PixelCameraPlugin;

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, pixel_zoom_system.after(camera::camera_system));
    }
}
