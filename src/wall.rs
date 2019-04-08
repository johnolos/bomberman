use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const WALL_HEIGHT: f32 = 128.0;
pub const WALL_WIDTH: f32 = 128.0;

pub struct Wall;

impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}
