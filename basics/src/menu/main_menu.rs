use crate::states::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct MenuUI;

pub fn spawn_menu(mut commands: Commands) {
    commands.spawn((Camera2d, MenuUI));

    commands.spawn((Text::new("PRESS SPACE TO START"), MenuUI));
}

pub fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InGame);
    }
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
