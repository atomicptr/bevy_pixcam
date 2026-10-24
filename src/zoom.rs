use bevy::{
    camera::{NormalizedRenderTarget, RenderTarget, Viewport},
    platform::collections::HashSet,
    prelude::*,
    window::{PrimaryWindow, WindowCreated, WindowResized},
};

const FLOAT_ERROR_MARGIN: f32 = 0.0001;

/// Configure a `Camera2d` to use integer scaling and automatically match
/// a specified resolution.
///
/// Note: when this component is present, a plugin system will automatically
/// update the `ScalingMode` of the camera bundle.
#[derive(Component, Debug, Clone, PartialEq, FromTemplate)]
pub enum PixelZoom {
    /// Manually specify the camera zoom, i.e. the number of screen pixels
    /// (logical pixels) used to display one virtual pixel (world unit).
    #[default]
    Fixed(i32),
    /// Automatically set the camera zoom to fit the specified resolution inside
    /// the window.
    FitSize { width: i32, height: i32 },
    /// Automatically set the camera zoom to fit the specified width inside the
    /// window.
    FitWidth(i32),
    /// Automatically set the camera zoom to fit the specified height inside the
    /// window.
    FitHeight(i32),
}

/// Configure a `Camera2d` to automatically set the viewport so that only
/// pixels inside the desired resolution (as defined by the `PixelZoom`
/// component) are displayed.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct PixelViewport;

pub(crate) fn pixel_zoom_system(
    mut window_resized_events: MessageReader<WindowResized>,
    mut window_created_events: MessageReader<WindowCreated>,
    mut image_asset_events: MessageReader<AssetEvent<Image>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    mut cameras: Query<(
        &mut Camera,
        &RenderTarget,
        &PixelZoom,
        Option<&PixelViewport>,
        &mut Projection,
    )>,
) {
    // Most of the change detection code is copied from https://github.com/bevyengine/bevy/blob/v0.11.0/crates/bevy_render/src/camera/camera.rs
    let primary_window = primary_window.iter().next();

    let mut changed_window_ids = HashSet::new();
    changed_window_ids.extend(window_created_events.read().map(|event| event.window));
    changed_window_ids.extend(window_resized_events.read().map(|event| event.window));

    let changed_image_handles: HashSet<&AssetId<Image>> = image_asset_events
        .read()
        .filter_map(|event| {
            if let AssetEvent::Modified { id } = event {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    for (mut camera, target, pixel_zoom, pixel_viewport, mut projection) in &mut cameras {
        if let Some(normalized_target) = target.normalize(primary_window)
            && (is_changed(
                &normalized_target,
                &changed_window_ids,
                &changed_image_handles,
            ) || camera.is_added()
                || projection.is_changed())
        {
            let Some(logical_size) = camera.logical_target_size() else {
                continue;
            };

            let Some(physical_size) = camera.physical_target_size() else {
                continue;
            };

            let zoom = auto_zoom(pixel_zoom, logical_size) as f32;
            let scale = 1.0 / zoom;

            match projection.as_mut() {
                Projection::Orthographic(projection) => {
                    if (scale - projection.scale).abs() > FLOAT_ERROR_MARGIN {
                        projection.scale = scale;
                    }
                }
                _ => continue,
            }

            if pixel_viewport.is_some() {
                set_viewport(&mut camera, pixel_zoom, zoom, physical_size, logical_size);
            }
        }
    }
}

fn is_changed(
    render_target: &NormalizedRenderTarget,
    changed_window_ids: &HashSet<Entity>,
    changed_image_handles: &HashSet<&AssetId<Image>>,
) -> bool {
    match render_target {
        NormalizedRenderTarget::Window(window_ref) => {
            changed_window_ids.contains(&window_ref.entity())
        }
        NormalizedRenderTarget::Image(image_handle) => {
            changed_image_handles.contains(&image_handle.handle.id())
        }
        NormalizedRenderTarget::TextureView(_)
        | NormalizedRenderTarget::None {
            width: _,
            height: _,
        } => true,
    }
}

fn auto_zoom(mode: &PixelZoom, logical_size: Vec2) -> i32 {
    match mode {
        PixelZoom::FitSize { width, height } => {
            let zoom_x = (logical_size.x as i32) / i32::max(*width, 1);
            let zoom_y = (logical_size.y as i32) / i32::max(*height, 1);
            let zoom = i32::min(zoom_x, zoom_y);
            i32::max(zoom, 1)
        }
        PixelZoom::FitWidth(width) => {
            let zoom = (logical_size.x as i32) / i32::max(*width, 1);
            i32::max(zoom, 1)
        }
        PixelZoom::FitHeight(height) => {
            let zoom = (logical_size.y as i32) / i32::max(*height, 1);
            i32::max(zoom, 1)
        }
        PixelZoom::Fixed(zoom) => i32::max(*zoom, 1),
    }
}

fn set_viewport(
    camera: &mut Camera,
    mode: &PixelZoom,
    zoom: f32,
    physical_size: UVec2,
    logical_size: Vec2,
) {
    let (auto_width, auto_height) = match mode {
        PixelZoom::FitSize { width, height } => (Some(*width), Some(*height)),
        PixelZoom::FitWidth(width) => (Some(*width), None),
        PixelZoom::FitHeight(height) => (None, Some(*height)),
        PixelZoom::Fixed(..) => (None, None),
    };

    let scale_factor = (physical_size.x as f32) / logical_size.x;

    let mut viewport_width = physical_size.x;
    let mut viewport_x = 0;
    if let Some(target_width) = auto_width {
        let logical_target_width = (target_width as f32) * zoom;
        viewport_width = (scale_factor * logical_target_width) as u32;
        viewport_x = (scale_factor * (logical_size.x - logical_target_width)) as u32 / 2;
    }

    let mut viewport_height = physical_size.y;
    let mut viewport_y = 0;
    if let Some(target_height) = auto_height {
        let logicat_target_height = (target_height as f32) * zoom;
        viewport_height = (scale_factor * logicat_target_height) as u32;
        viewport_y = (scale_factor * (logical_size.y - logicat_target_height)) as u32 / 2;
    }

    camera.viewport = Some(Viewport {
        physical_position: UVec2 {
            x: viewport_x,
            y: viewport_y,
        },
        physical_size: UVec2 {
            x: viewport_width,
            y: viewport_height,
        },
        ..Default::default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_zoom_is_clamped_to_at_least_one() {
        let size = Vec2::new(1920.0, 1080.0);

        assert_eq!(auto_zoom(&PixelZoom::Fixed(0), size), 1);
        assert_eq!(auto_zoom(&PixelZoom::Fixed(-5), size), 1);
        assert_eq!(auto_zoom(&PixelZoom::Fixed(1), size), 1);
        assert_eq!(auto_zoom(&PixelZoom::Fixed(3), size), 3);
    }

    #[test]
    fn other_modes_still_clamp_to_one() {
        let size = Vec2::new(100.0, 100.0);

        assert_eq!(
            auto_zoom(
                &PixelZoom::FitSize {
                    width: 200,
                    height: 200
                },
                size
            ),
            1
        );
        assert_eq!(auto_zoom(&PixelZoom::FitWidth(200), size), 1);
        assert_eq!(auto_zoom(&PixelZoom::FitHeight(200), size), 1);
    }
}
