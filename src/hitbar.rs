// CALCULATE ACCURACY WITH ABS VALUE OF TARGET Y POS - HITBAR Y POS
use crate::{score::Score, target::Target};
use bevy::prelude::*;

pub struct HitbarPlugin;

use crate::target::{TARGET_GAP, TARGET_LEFT_POS, TARGET_WIDTH};

impl Plugin for HitbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_hitbars)
            .add_systems(Update, hitbar_input);
    }
}

#[derive(Component)]
struct Hitbar {
    id: u8,
}

impl Hitbar {
    fn new(id: u8) -> Self {
        Self { id }
    }
}

#[derive(Debug, PartialEq)]
enum Accuracy {
    Perfect,
    Great,
    Good,
    Meh,
    Bad,
    Miss,
}

fn hitbar_input(
    mut commands: Commands,
    mut hitbar_query: Query<(&Hitbar, &mut Handle<Image>, &Transform)>,
    target_query: Query<(&Target, &Transform, Entity), With<Target>>,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    for (hitbar, mut texture_handle, hitbar_transform) in hitbar_query.iter_mut() {
        let hitbar_input = (
            input.any_pressed([KeyCode::D, KeyCode::Up]),
            input.any_pressed([KeyCode::F, KeyCode::Down]),
            input.any_pressed([KeyCode::J, KeyCode::Left]),
            input.any_pressed([KeyCode::K, KeyCode::Right]),
        );

        let hitbar_input_just_pressed = (
            input.any_just_pressed([KeyCode::D, KeyCode::Up]),
            input.any_just_pressed([KeyCode::F, KeyCode::Down]),
            input.any_just_pressed([KeyCode::J, KeyCode::Left]),
            input.any_just_pressed([KeyCode::K, KeyCode::Right]),
        );

        match hitbar.id {
            1 => {
                if hitbar_input.0 {
                    *texture_handle = asset_server.load("hitbar_pressed.png");
                } else {
                    *texture_handle = asset_server.load("hitbar_normal.png");
                }
            }
            2 => {
                if hitbar_input.1 {
                    *texture_handle = asset_server.load("hitbar_pressed.png");
                } else {
                    *texture_handle = asset_server.load("hitbar_normal.png");
                }
            }
            3 => {
                if hitbar_input.2 {
                    *texture_handle = asset_server.load("hitbar_pressed.png");
                } else {
                    *texture_handle = asset_server.load("hitbar_normal.png");
                }
            }
            4 => {
                if hitbar_input.3 {
                    *texture_handle = asset_server.load("hitbar_pressed.png");
                } else {
                    *texture_handle = asset_server.load("hitbar_normal.png");
                }
            }
            _ => {
                warn!("There is hitbar that isnt 1-4");
            }
        }

        for (target, target_transform, target_entity) in target_query.iter() {
            let pixel_accuracy =
                (hitbar_transform.translation.y - target_transform.translation.y).abs();

            let accuracy: Accuracy;
            let add_score: i16;

            if pixel_accuracy <= 15.0 {
                accuracy = Accuracy::Perfect;
                add_score = 400;
            } else if pixel_accuracy <= 20.0 {
                accuracy = Accuracy::Great;
                add_score = 350;
            } else if pixel_accuracy <= 30.0 {
                accuracy = Accuracy::Good;
                add_score = 300;
            } else if pixel_accuracy <= 40.0 {
                accuracy = Accuracy::Meh;
                add_score = 150;
            } else if pixel_accuracy <= 50.0 {
                accuracy = Accuracy::Bad;
                add_score = 50;
            } else {
                add_score = -50;
                accuracy = Accuracy::Miss
            }

            if accuracy != Accuracy::Miss {
                if hitbar_input_just_pressed.0 && target.id == 1 {
                    commands.entity(target_entity).despawn();
                } else if hitbar_input_just_pressed.1 && target.id == 2 {
                    commands.entity(target_entity).despawn();
                } else if hitbar_input_just_pressed.2 && target.id == 3 {
                    commands.entity(target_entity).despawn();
                } else if hitbar_input_just_pressed.3 && target.id == 4 {
                    commands.entity(target_entity).despawn();
                }

                if input.any_just_pressed([KeyCode::D, KeyCode::F, KeyCode::J, KeyCode::K]) {
                    score.0 += add_score as i64;
                    info!("{:?}", accuracy);
                    info!("{:?}", score);
                }
            }
        }
    }
}

fn create_hitbars(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 1..=4 {
        let hitbar_x_pos = match i {
            1 => TARGET_LEFT_POS,
            2 => TARGET_LEFT_POS + TARGET_GAP + TARGET_WIDTH,
            3 => TARGET_LEFT_POS + TARGET_GAP * 2.0 + TARGET_WIDTH * 2.0,
            4 => TARGET_LEFT_POS + TARGET_GAP * 3.0 + TARGET_WIDTH * 3.0,
            _ => 50.0,
        };

        commands.spawn((
            Name::from(format!("Hitbar #{}", i)),
            Hitbar::new(i),
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
