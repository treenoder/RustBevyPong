mod components;
mod systems;
mod consts;

use crate::consts::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "RustBevyPong".to_string(),
            resolution: WindowResolution::from((WINDOW_WIDTH, WINDOW_HEIGHT)),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_systems(Startup, (
        spawn_camera,
        spawn_players,
        spawn_ball,
    ));
    app.add_systems(Update, (
        exit_on_esc,
        move_puddle,
        move_ball,
        collide_ball,
    ));
    app.run();
}

