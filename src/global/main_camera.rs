use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component, Debug)]
pub struct Tag;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, on_startup);
    }
}

fn on_startup(windows: Query<&Window, With<PrimaryWindow>>, mut cmd: Commands) {
    let window = windows.get_single().expect("Expected primary window");
    cmd.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3 {
                x: window.resolution.width() / 2.0,
                y: window.resolution.height() / 2.0,
                z: 0.0,
            },
            ..default()
        },
        ..default()
    })
    .insert(Tag);
}
