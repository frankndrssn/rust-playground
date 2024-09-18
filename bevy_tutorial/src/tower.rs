use crate::{bullet, target};
use bevy::{math::FloatOrd, prelude::*};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shoot_cooldown: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_systems(Update, tower_shooting);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    targets: Query<&GlobalTransform, With<target::Target>>,
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
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                        .insert(Name::new("Bullet"))
                        .insert(bullet::LifeTime {
                            timer: Timer::from_seconds(3.0, TimerMode::Once),
                        })
                        .insert(bullet::Bullet {
                            direction,
                            speed: 5.0,
                        });
                });
            }
        }
    }
}
