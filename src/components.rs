use bevy::prelude::{Component, KeyCode, Vec2};

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
}

#[derive(Component)]
pub struct Ball {
    pub direction: Vec2,
    pub speed: f32,
}