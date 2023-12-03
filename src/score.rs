use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
    }
}

#[derive(Resource, Debug)]
pub struct Score(pub i64);
