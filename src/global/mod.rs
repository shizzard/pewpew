pub mod main_camera;
pub mod main_window;

use bevy::prelude::*;

use self::main_camera::MainCameraPlugin;
use self::main_window::MainWindowPlugin;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainWindowPlugin)
            .add_plugins(MainCameraPlugin);
    }
}
