pub mod player;
use crate::states::GameState;
use bevy::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), player::spawn_player)
            .add_systems(Update, player::movement.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), player::cleanup);
    }
}
