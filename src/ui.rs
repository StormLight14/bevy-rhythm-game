use crate::score::Score;
use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, score_update_system);
    }
}

#[derive(Component)]
struct ScoreText;

fn spawn_game_ui(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 32.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ScoreText,
    ));
}

fn score_update_system(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        // Update the value of the second section
        text.sections[0].value = format!("Score: {}", score.0);
    }
}
