use bevy::{math::FloatOrd, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Tower {
    shoot_cooldown: Timer,
    bullet_offset: Vec3,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct LifeTime {
    timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Target {
    speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Health {
    value: i32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Bullet {
    direction: Vec3,
    speed: f32,
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
    let target = meshes.add(Cuboid::new(0.5, 0.5, 0.5));

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
            bullet_offset: Vec3::ZERO,
        })
        .insert(Name::new("Tower"));

    commands
        .spawn(MaterialMeshBundle {
            mesh: target,
            material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(-5.0, 0.5, 2.0),
            ..default()
        })
        .insert(Target { speed: 0.5 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

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
    targets: Query<&GlobalTransform, With<Target>>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    time: Res<Time>,
) {
    for (entity, mut tower, transform) in towers.iter_mut() {
        tower.shoot_cooldown.tick(time.delta());
        if tower.shoot_cooldown.just_finished() {
            let spawn_v3 = transform.translation() + tower.bullet_offset;
            let direction = targets
                .iter()
                .min_by_key(|target| FloatOrd(Vec3::distance(target.translation(), spawn_v3)))
                .map(|closest_target| closest_target.translation() - spawn_v3);

            if let Some(direction) = direction {
                commands.entity(entity).with_children(|commands| {
                    commands
                        .spawn(MaterialMeshBundle {
                            mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                            transform: Transform::from_translation(spawn_v3),
                            ..default()
                        })
                        .insert(Name::new("Bullet"))
                        .insert(LifeTime {
                            timer: Timer::from_seconds(3.0, TimerMode::Once),
                        })
                        .insert(Bullet {
                            direction,
                            speed: 1.0,
                        });
                });
            }
        }
    }
}

fn move_bullet(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
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

fn move_target(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in targets.iter_mut() {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, spawn_basic_scene)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, tower_shooting)
        .add_systems(Update, despawn_bullets)
        .add_systems(Update, move_target)
        .add_systems(Update, move_bullet)
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
        .register_type::<LifeTime>()
        .register_type::<Target>()
        .register_type::<Health>()
        .register_type::<Bullet>()
        .run();
}
