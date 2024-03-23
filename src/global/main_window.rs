use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::WindowTheme;

const WINDOW_WIDTH_PX: f32 = 850.0;
const WINDOW_HEIGHT_PX: f32 = 1050.0;

const WINDOW_TITLE: &str = "Pew-pew!";
const APP_NAME: &str = "pewpew.app";

pub struct MainWindowPlugin;

impl Plugin for MainWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: WINDOW_TITLE.into(),
                    name: Some(APP_NAME.into()),
                    resolution: (WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX).into(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ));
    }
}
