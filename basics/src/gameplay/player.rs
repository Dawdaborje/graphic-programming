use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let circle = meshes.add(Circle::new(40.0));

    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 1.0))),
        Transform::default(),
        Player,
    ));
}

pub fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut t = query.single_mut().unwrap();

    if keyboard.pressed(KeyCode::ArrowLeft) {
        t.translation.x -= 300.0 * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        t.translation.x += 300.0 * time.delta_secs();
    }
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
