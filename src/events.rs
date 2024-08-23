use crate::components::PaddleSide;
use bevy::prelude::Event;

#[derive(Event)]
pub enum GameEvent {
    GainPoint(PaddleSide),
}