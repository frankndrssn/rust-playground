use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Tower {
    shoot_cooldown: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct LifeTime {
    timer: Timer,
}

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
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    commands
        .spawn(MaterialMeshBundle {
            mesh: floor,
            material: materials.add(Color::srgb(0.22, 0.5, 0.02)),
            ..default()
        })
        .insert(Name::new("Floor"));

    commands
        .spawn(MaterialMeshBundle {
            mesh: cube,
            material: materials.add(Color::srgb(0.737, 0.678, 0.549)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shoot_cooldown: Timer::from_seconds(5.0, TimerMode::Repeating),
        })
        .insert(Name::new("Tower"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 2_000_000.0,
                color: Color::srgb(1.0, 0.5, 0.1),
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Point light"));
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in towers.iter_mut() {
        tower.shoot_cooldown.tick(time.delta());
        if tower.shoot_cooldown.just_finished() {
            let spawn_transform = Transform::from_xyz(0.0, 0.7, 0.6)
                .with_rotation(Quat::from_rotation_y(-3.14 / 2.0));

            commands
                .spawn(MaterialMeshBundle {
                    mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                    material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                    transform: spawn_transform,
                    ..default()
                })
                .insert(Name::new("Bullet"))
                .insert(LifeTime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                });
        }
    }
}

fn despawn_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in bullets.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, spawn_basic_scene)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, tower_shooting)
        .add_systems(Update, despawn_bullets)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "A window".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .run();
}
