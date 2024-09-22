use bevy::{prelude::*, window};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
struct CameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(10., 20., 0.),
            ..Default::default()
        },
        CameraMarker,
    ));
}

fn setup_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ryo_haskell.png"),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "sprite".into(),
                resizable: false,
                resolution:
                    window::WindowResolution::new(1000., 600.).with_scale_factor_override(1.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (setup_camera, setup_sprite))
        .run();
}
