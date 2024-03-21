pub mod camera;
pub mod game;
pub mod ui;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::WindowTheme;
use bevy::winit::WinitSettings;
use game::state::State as GameState;
use game::SystemSet;

const WINDOW_WIDTH_PX: f32 = 850.0;
const WINDOW_HEIGHT_PX: f32 = 1050.0;

// const ARENA_WIDTH_UNSCALED_PX: u32 = 800;
// const ARENA_HEIGHT_UNSCALED_PX: u32 = 800;
// const ARENA_WIDTH_TILES: u32 = 10;
// const ARENA_HEIGHT_TILES: u32 = 10;
// const ARENA_BACKGROUND_COLOR: Color = Color::BLACK;

// const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

// #[derive(Component, Clone, Copy, PartialEq, Eq)]
// struct Position {
//     x: i32,
//     y: i32,
// }

// #[derive(Component)]
// struct Size {
//     width: f32,
//     height: f32,
// }
// impl Size {
//     pub fn square(x: f32) -> Self {
//         Self {
//             width: x,
//             height: x,
//         }
//     }
// }

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pew-pew!".into(),
                name: Some("pewpew.app".into()),
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
    ))
    .insert_resource(WinitSettings::desktop_app())
    .init_state::<GameState>();

    app.add_plugins(camera::main::Camera)
        .configure_sets(
            Update,
            (
                SystemSet::Global,
                SystemSet::MainMenu
                    .run_if(in_state(GameState::MainMenu))
                    .after(SystemSet::Global),
            ),
        )
        .add_plugins(game::transition::StateTransition)
        .add_plugins(ui::main_menu::UI);

    app.run();
}

// #[derive(Component)]
// struct Arena;
// fn spawn_arena(windows: Query<&mut Window>, mut commands: Commands) {
//     let window = windows.get_single().unwrap();
//     let window_scale_factor = window.resolution.scale_factor();
//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 custom_size: Some(Vec2 {
//                     x: (ARENA_WIDTH_UNSCALED_PX as f32) / window_scale_factor,
//                     y: (ARENA_HEIGHT_UNSCALED_PX as f32) / window_scale_factor,
//                 }),
//                 color: ARENA_BACKGROUND_COLOR,
//                 anchor: bevy::sprite::Anchor::BottomLeft,
//                 ..default()
//             },
//             ..default()
//         })
//         .insert(Arena);
// }

// #[derive(Component)]
// struct Tile;

// #[derive(Component)]
// struct SnakeHead;

// fn spawn_snake(mut commands: Commands) {
//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: SNAKE_HEAD_COLOR,
//                 anchor: bevy::sprite::Anchor::BottomLeft,
//                 ..default()
//             },
//             ..default()
//         })
//         .insert(SnakeHead)
//         .insert(Tile)
//         .insert(Position { x: 0, y: 0 })
//         .insert(Size::square(1.0));
// }

// fn snake_movement(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut head_positions: Query<&mut Position, With<SnakeHead>>,
// ) {
//     for mut pos in head_positions.iter_mut() {
//         if keyboard_input.just_pressed(KeyCode::KeyA) {
//             pos.x -= 1;
//         }
//         if keyboard_input.just_pressed(KeyCode::KeyD) {
//             pos.x += 1;
//         }
//         if keyboard_input.just_pressed(KeyCode::KeyS) {
//             pos.y -= 1;
//         }
//         if keyboard_input.just_pressed(KeyCode::KeyW) {
//             pos.y += 1;
//         }
//     }
// }

// fn size_scaling(
//     windows: Query<&mut Window>,
//     mut entities: Query<(&mut Size, &mut Transform), With<Tile>>,
// ) {
//     let window = windows.get_single().unwrap();
//     let window_scale_factor = window.resolution.scale_factor();
//     for (size, mut transform) in entities.iter_mut() {
//         transform.scale = Vec3::new(
//             (size.width * (ARENA_WIDTH_UNSCALED_PX / ARENA_WIDTH_TILES) as f32)
//                 / window_scale_factor,
//             (size.height * (ARENA_HEIGHT_UNSCALED_PX / ARENA_HEIGHT_TILES) as f32)
//                 / window_scale_factor,
//             1.0,
//         );
//     }
// }

// fn position_translation(
//     windows: Query<&mut Window>,
//     mut entities: Query<(&Position, &mut Transform), With<Tile>>,
// ) {
//     let window = windows.get_single().unwrap();
//     let window_scale_factor = window.resolution.scale_factor();
//     fn convert(pos: f32, bound_px: f32, bound_tiles: f32, scale_factor: f32) -> f32 {
//         let tile_size = bound_px / bound_tiles;
//         let adj_pos = (pos * tile_size) / scale_factor;
//         adj_pos
//     }
//     for (pos, mut transform) in entities.iter_mut() {
//         transform.translation = Vec3::new(
//             convert(
//                 pos.x as f32,
//                 ARENA_WIDTH_UNSCALED_PX as f32,
//                 ARENA_WIDTH_TILES as f32,
//                 window_scale_factor,
//             ),
//             convert(
//                 pos.y as f32,
//                 ARENA_HEIGHT_UNSCALED_PX as f32,
//                 ARENA_HEIGHT_TILES as f32,
//                 window_scale_factor,
//             ),
//             0.0,
//         );
//     }
// }
