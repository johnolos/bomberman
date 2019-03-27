use amethyst::{
    core::Time,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

use crate::bomb::Bomb;

pub struct BombTimerSystem;

impl<'s> System<'s> for BombTimerSystem {
    type SystemData = (WriteStorage<'s, Bomb>, Read<'s, Time>);

    fn run(&mut self, (mut bombs, time): Self::SystemData) {
        for bomb in (&mut bombs).join() {
            bomb.time_left -= time.delta_seconds();
        }
    }
}
