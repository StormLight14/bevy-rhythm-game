use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const VIEW_WIDTH: f32 = 1920.0;
pub const VIEW_HEIGHT: f32 = 1080.0;
pub const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub mod arrow;
pub mod camera;
pub mod hitbar;

use arrow::ArrowPlugin;
use camera::CameraPlugin;
use hitbar::HitbarPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Rhythm Game".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins((CameraPlugin, ArrowPlugin, HitbarPlugin))
        .run();
}
