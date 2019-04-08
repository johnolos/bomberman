use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const BOMB_HEIGHT: f32 = 128.0;
pub const BOMB_WIDTH: f32 = 128.0;

pub const BOMB_X_MOVEMENT: f32 = 1.2;
pub const BOMB_Y_MOVEMENT: f32 = 0.675;

pub const INITIAL_BOMB_TIME: f32 = 3.0;
pub const INITIAL_BLAST_RADIUS: i8 = 1;

use crate::core::PlayerNumber;

pub struct Bomb {
    pub owner: PlayerNumber,
    pub velocity: [f32; 2],
    pub blast_radius: i8,
    pub time_left: f32,
}

impl Bomb {
    pub fn new(owner: PlayerNumber, initial_time: f32, blast_radius: i8) -> Self {
        Bomb {
            owner,
            time_left: initial_time,
            velocity: [0.0f32, 0.0f32],
            blast_radius,
        }
    }
}

impl Component for Bomb {
    type Storage = DenseVecStorage<Self>;
}
