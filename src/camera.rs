use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

use crate::{BACKGROUND_COLOR, VIEW_HEIGHT, VIEW_WIDTH};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // camera has 0, 0 in bottom left
    camera.transform.translation += Vec3::new(VIEW_WIDTH / 2.0, VIEW_HEIGHT / 2.0, 0.0);

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: VIEW_WIDTH,
        min_height: VIEW_HEIGHT,
    };

    commands.spawn(camera);
}
