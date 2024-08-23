mod components;
mod systems;
mod consts;
mod events;

use crate::components::GameState;
use crate::consts::*;
use crate::events::GameEvent;
use crate::systems::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    println!("Hello, world!");
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
    app.add_event::<GameEvent>();
    app.insert_resource(GameState::default());
    app.add_systems(Startup, (
        spawn_camera,
        spawn_players,
        spawn_ball,
        spawn_score,
    ));
    app.add_systems(Update, (
        exit_on_esc,
        start,
        move_puddle,
        move_ball,
        collide_ball,
        score,
    ));
    app.run();
}

