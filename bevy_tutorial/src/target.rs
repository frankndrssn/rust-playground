use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_systems(Update, target_death)
            .add_systems(Update, move_target);
    }
}

fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (target, health) in targets.iter() {
        if health.value <= 0 {
            commands.entity(target).despawn_recursive();
        }
    }
}

fn move_target(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in targets.iter_mut() {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}
