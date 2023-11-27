// CALCULATE ACCURACY WITH ABS VALUE OF ARROW Y POS - HITBAR Y POS

use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::{VIEW_HEIGHT, VIEW_WIDTH};

const FALL_SPEED: f32 = 150.0;
pub const ARROW_LEFT_POS: f32 =
    VIEW_WIDTH / 2.0 - (ARROW_WIDTH * 4.0 + ARROW_GAP * 3.0) / 4.0 - ARROW_WIDTH / 2.0; // i think this works
pub const ARROW_WIDTH: f32 = 128.0;
pub const ARROW_GAP: f32 = 10.0;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct ArrowPlugin;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ArrowSpawnTimer(Timer::from_seconds(
            1.5,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (spawn_arrows, move_arrows));
    }
}

#[derive(Component, Debug)]
struct Arrow {
    direction: Direction,
}

#[derive(Resource)]
struct ArrowSpawnTimer(Timer);

fn spawn_arrows(
    mut commands: Commands,
    mut spawn_timer: ResMut<ArrowSpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer
        .0
        .tick(Duration::from_secs_f32(time.delta_seconds()));

    info!(" Half of view_width: {} \n", VIEW_WIDTH / 2.0);

    if spawn_timer.0.just_finished() {
        let mut arrow_x_pos: f32;

        let (arrow_direction, arrow_sprite) = match rand::thread_rng().gen_range(1..=4) {
            1 => {
                arrow_x_pos = ARROW_LEFT_POS;
                (Direction::Up, asset_server.load("arrow_up.png"))
            }
            2 => {
                arrow_x_pos = ARROW_LEFT_POS + ARROW_GAP + ARROW_WIDTH;
                (Direction::Down, asset_server.load("arrow_down.png"))
            }
            3 => {
                arrow_x_pos = ARROW_LEFT_POS + ARROW_GAP * 2.0 + ARROW_WIDTH * 2.0;
                (Direction::Right, asset_server.load("arrow_right.png"))
            }
            4 => {
                arrow_x_pos = ARROW_LEFT_POS + ARROW_GAP * 3.0 + ARROW_WIDTH * 3.0;
                (Direction::Left, asset_server.load("arrow_left.png"))
            }
            _ => {
                arrow_x_pos = 50.0;
                (Direction::Up, asset_server.load("arrow_up.png"))
            }
        };

        //info!("Spawned Arrow with direction {:?}", arrow_direction);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite { ..default() },
                texture: arrow_sprite,
                transform: Transform {
                    translation: Vec3::new(arrow_x_pos, VIEW_HEIGHT - 100.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Name::from("Arrow"),
            Arrow {
                direction: arrow_direction,
            },
        ));
    }
}

fn move_arrows(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Arrow>>,
    time: Res<Time>,
) {
    for (mut arrow_transform, arrow) in query.iter_mut() {
        arrow_transform.translation.y -= FALL_SPEED * time.delta_seconds();

        if arrow_transform.translation.y < 0.0 - ARROW_WIDTH / 2.0 {
            commands.entity(arrow).despawn();
        }
    }
}
