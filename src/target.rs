use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::{score::Score, VIEW_HEIGHT, VIEW_WIDTH};

const FALL_SPEED: f32 = 800.0;
pub const TARGET_LEFT_POS: f32 =
    VIEW_WIDTH / 2.0 - (TARGET_WIDTH * 4.0 + TARGET_GAP * 3.0) / 4.0 - TARGET_WIDTH / 2.0; // i think this works
pub const TARGET_WIDTH: f32 = 128.0;
pub const TARGET_GAP: f32 = 10.0;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TargetSpawnTimer(Timer::from_seconds(
            0.15,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (spawn_targets, move_targets));
    }
}

#[derive(Component, Debug)]
pub struct Target {
    pub id: u8,
}

#[derive(Resource)]
struct TargetSpawnTimer(Timer);

fn spawn_targets(
    mut commands: Commands,
    mut spawn_timer: ResMut<TargetSpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer
        .0
        .tick(Duration::from_secs_f32(time.delta_seconds()));

    if spawn_timer.0.just_finished() {
        let target_x_pos: f32;

        let (target_direction, target_sprite, target_id) = match rand::thread_rng().gen_range(1..=4)
        {
            1 => {
                target_x_pos = TARGET_LEFT_POS;
                (Direction::Up, asset_server.load("target-small.png"), 1)
            }
            2 => {
                target_x_pos = TARGET_LEFT_POS + TARGET_GAP + TARGET_WIDTH;
                (Direction::Down, asset_server.load("target-small.png"), 2)
            }
            3 => {
                target_x_pos = TARGET_LEFT_POS + TARGET_GAP * 2.0 + TARGET_WIDTH * 2.0;
                (Direction::Right, asset_server.load("target-small.png"), 3)
            }
            4 => {
                target_x_pos = TARGET_LEFT_POS + TARGET_GAP * 3.0 + TARGET_WIDTH * 3.0;
                (Direction::Left, asset_server.load("target-small.png"), 4)
            }
            _ => {
                target_x_pos = 50.0;
                (Direction::Up, asset_server.load("target-small.png"), 0)
            }
        };

        //info!("Spawned Target with direction {:?}", target_direction);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite { ..default() },
                texture: target_sprite,
                transform: Transform {
                    translation: Vec3::new(target_x_pos, VIEW_HEIGHT - 100.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Name::from("Target"),
            Target { id: target_id },
        ));
    }
}

fn move_targets(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Target>>,
    time: Res<Time>,
    mut score: ResMut<Score>,
) {
    for (mut target_transform, target) in query.iter_mut() {
        target_transform.translation.y -= FALL_SPEED * time.delta_seconds();

        if target_transform.translation.y < 0.0 - TARGET_WIDTH / 2.0 {
            commands.entity(target).despawn();
            if score.0 >= 400 {
                score.0 -= 400;
            } else {
                score.0 = 0;
            }
        }
    }
}
