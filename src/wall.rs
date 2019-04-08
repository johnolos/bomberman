use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Wall;

impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}
