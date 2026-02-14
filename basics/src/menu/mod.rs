pub mod main_menu;
use crate::states::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), main_menu::spawn_menu)
            .add_systems(
                Update,
                main_menu::menu_input.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), main_menu::despawn_menu);
    }
}
