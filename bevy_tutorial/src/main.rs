use bevy::prelude::*;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(3.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));

    commands.spawn(MaterialMeshBundle {
        mesh: floor,
        material: materials.add(Color::srgb(0.2, 0.9, 0.1)),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: cube,
        material: materials.add(Color::srgb(0.2, 0.3, 0.4)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2_000_000.0,
            color: Color::srgb(1.0, 0.5, 0.1),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, spawn_basic_scene)
        .add_systems(Startup, spawn_camera)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "A window".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .run();
}
