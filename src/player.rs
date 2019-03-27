use crate::core::{Orientation, Orientation::*, PlayerNumber};
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const PLAYER_HEIGHT: f32 = 64.0;
pub const PLAYER_WIDTH: f32 = 64.0;

pub const PLAYER_X_MOVEMENT: f32 = 960.0;
pub const PLAYER_Y_MOVEMENT: f32 = 540.0;

pub fn map_to_sprite(orient: Orientation) -> Option<i8> {
    match orient {
        North => Some(0),
        East => Some(1),
        South => Some(2),
        West => Some(3),
    }
}

pub struct Player {
    pub player_number: PlayerNumber,
    pub orient: Orientation,
    pub height: f32,
    pub width: f32,
    pub can_kick: bool,
    pub allowed_bombs: i8,
    pub active_bombs: i8,
    pub bomb_time_multiplier: f32,
    pub blast_radius_multiplier: i8,
}

impl Player {
    pub fn new(player_number: PlayerNumber, orient: Orientation) -> Player {
        Player {
            player_number,
            orient,
            height: PLAYER_HEIGHT,
            width: PLAYER_WIDTH,
            can_kick: false,
            allowed_bombs: 1,
            active_bombs: 0,
            bomb_time_multiplier: 1.0f32,
            blast_radius_multiplier: 1,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
