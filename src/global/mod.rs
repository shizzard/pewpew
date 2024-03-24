pub mod main_camera;
pub mod main_window;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rand::prelude::EntropyPlugin;
use bevy_rand::prelude::WyRand;

use self::main_camera::MainCameraPlugin;
use self::main_window::MainWindowPlugin;
use crate::encounter::component::*;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainWindowPlugin)
            .add_plugins(MainCameraPlugin)
            .add_plugins(
                WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::KeyI)),
            )
            .add_plugins(EntropyPlugin::<WyRand>::default())
            .register_type::<MovableX>()
            .register_type::<MovableY>()
            .register_type::<MovementBound>()
            .register_type::<Speed>()
            .register_type::<EntitySize>();
    }
}
