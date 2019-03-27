use amethyst::{
    core::{
        Time,
        transform::Transform,
        nalgebra::Vector3,
    },
    ecs::prelude::{Join, Read, System, WriteStorage},

};

use crate::bomb::Bomb;

pub struct BombTimerSystem;

impl<'s> System<'s> for BombTimerSystem {
    type SystemData = (WriteStorage<'s, Bomb>, WriteStorage<'s, Transform>, Read<'s, Time>);

    fn run(&mut self, (mut bombs, mut transforms, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (bomb, transform) in (&mut bombs, &mut transforms).join() {
            bomb.time_left -= delta_time;
            let scale_factor = (5.0 * bomb.time_left).sin()*0.25 + 1.0;
            transform.set_scale(scale_factor, scale_factor, 1.0);
        }
    }
}
