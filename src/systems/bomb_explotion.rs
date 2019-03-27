use amethyst::{
    core::Time,
    ecs::prelude::{Entities, Join, Read, System, WriteStorage},
};

use crate::bomb::Bomb;

pub struct BombExplotionSystem;

impl<'s> System<'s> for BombExplotionSystem {
    type SystemData = (Entities<'s>, WriteStorage<'s, Bomb>, Read<'s, Time>);

    fn run(&mut self, (entities, mut bombs, time): Self::SystemData) {
        for (e, bomb) in (&*entities, &mut bombs).join() {
            if bomb.time_left < 0.0 {
                println!("Bomb exploded");
                entities.delete(e);
            }
        }
    }
}
