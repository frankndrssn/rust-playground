use bevy::prelude::*;

#[derive(Component)]
struct CameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        CameraMarker,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}
