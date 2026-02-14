use bevy::prelude::*;

mod gameplay;
mod menu;
mod states;

use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((menu::MenuPlugin, gameplay::GameplayPlugin))
        .run();
}
