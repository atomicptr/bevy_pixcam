//! A simple camera plugin for the Bevy game engine, to help with the use of
//! pixel-art sprites.
//!
//! This crates provides a plugin to automatically configure Bevy's
//! `Camera2d`. It works by setting the camera to an integer scaling
//! factor (using Bevy's `ScalingMode::WindowSize`), and automatically updating
//! the zoom level so that the specified target resolution fills as much of the
//! sceen as possible.
//!
//! The plugin can also automatically set and resize the viewport of the camera
//! to match the target resolution.
//!
//! # Comparison with other methods
//!
//! There is two main methods to render pixel-art games: upscale each sprite
//! independently, or render everything to an offscreen texture and only upscale
//! this texture. This crate use the first method. There is advantages and
//! drawbacks to both approaches.
//!
//! Advantages of the "upscale each sprite independently" method (i.e. this
//! crate):
//!
//! - allows for smoother scrolling and movement of sprites, if you're willing
//!   to temporarily break the alignment on virtual pixels (this would be even
//!   more effective with a dedicated upscaling shader);
//! - easier to mix pixel-art and high resolution graphics (for example for
//!   text, particles or effects).
//!
//! Advantages of the "offscreen texture" method:
//!
//! - always ensure perfect alignment on virtual pixels (authentic "retro"
//!   look);
//! - may be more efficient (in most cases, the difference is probably
//!   negligible on modern computers).
mod plugin;
mod zoom;

pub use plugin::*;
pub use zoom::*;
