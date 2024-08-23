use crate::consts::BALL_SPEED;
use bevy::prelude::{Component, KeyCode, Resource, Vec2};
use rand::Rng;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PaddleSide {
    Left,
    Right,
}

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
    pub is_out: bool,
}

impl Default for Ball {
    fn default() -> Self {
        let dir_x = if rand::thread_rng().gen_bool(0.5) {
            1.0
        } else {
            -1.0
        };
        let dir_y = rand::thread_rng().gen_range(-10..=10) as f32 / 10.0;
        Self {
            direction: Vec2::new(dir_x, dir_y).normalize(),
            speed: BALL_SPEED,
            is_out: false,
        }
    }
}

#[derive(Default, Component)]
pub struct ScoreText();

#[derive(Resource)]
pub struct GameState {
    pub left_score: u32,
    pub right_score: u32,
    pub winner: Option<PaddleSide>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            left_score: 0,
            right_score: 0,
            winner: Some(PaddleSide::Left),
        }
    }
}

