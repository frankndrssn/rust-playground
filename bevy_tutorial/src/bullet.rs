use bevy::prelude::*;

use crate::target;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct LifeTime {
    pub timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LifeTime>()
            .register_type::<Bullet>()
            .add_systems(Update, bullet_collision)
            .add_systems(Update, move_bullet)
            .add_systems(Update, despawn_bullets);
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut target::Health, &Transform), With<target::Target>>,
) {
    for (bullet, bullet_transform) in bullets.iter() {
        for (mut health, target_transform) in targets.iter_mut() {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.5 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
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
