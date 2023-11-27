use bevy::prelude::*;

pub struct HitbarPlugin;

use crate::arrow::{ARROW_GAP, ARROW_LEFT_POS, ARROW_WIDTH};

impl Plugin for HitbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_hitbars);
    }
}

#[derive(Component)]
struct Hitbar;

fn create_hitbars(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 1..=4 {
        let mut hitbar_x_pos: f32;

        match i {
            1 => {
                hitbar_x_pos = ARROW_LEFT_POS;
            }
            2 => {
                hitbar_x_pos = ARROW_LEFT_POS + ARROW_GAP + ARROW_WIDTH;
            }
            3 => {
                hitbar_x_pos = ARROW_LEFT_POS + ARROW_GAP * 2.0 + ARROW_WIDTH * 2.0;
            }
            4 => {
                hitbar_x_pos = ARROW_LEFT_POS + ARROW_GAP * 3.0 + ARROW_WIDTH * 3.0;
            }
            _ => {
                hitbar_x_pos = 50.0;
            }
        }

        commands.spawn((
            Name::from(format!("Hitbar #{}", i)),
            Hitbar,
            SpriteBundle {
                sprite: Sprite { ..default() },
                texture: asset_server.load("hitbar_normal.png"),
                transform: Transform {
                    translation: Vec3::new(hitbar_x_pos, 200.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ));
    }
}
