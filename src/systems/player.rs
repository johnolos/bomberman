use amethyst::{
    core::timing::Time,
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler
};

use crate::{
    core::PlayerNumber,
    player::{Player, PLAYER_X_MOVEMENT, PLAYER_Y_MOVEMENT}
};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, players, input, time): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let movement_y_axis = match player.player_number {
                PlayerNumber::PlayerOne => input.axis_value("left_player_vertical"),
                PlayerNumber::PlayerTwo => input.axis_value("right_player_vertical")
            };

            let movement_x_axis = match player.player_number {
                PlayerNumber::PlayerOne => input.axis_value("left_player_horizontal"),
                PlayerNumber::PlayerTwo => input.axis_value("right_player_horizontal")
            };

            if let Some(mv_y_axis) = movement_y_axis {
                let scaled_y_movement = PLAYER_Y_MOVEMENT * mv_y_axis as f32;
                transform.translate_y(scaled_y_movement * time.delta_seconds());
            }

            if let Some(mv_x_axis) = movement_x_axis {
                let scaled_x_movement = PLAYER_X_MOVEMENT * mv_x_axis as f32;
                transform.translate_x(scaled_x_movement * time.delta_seconds());
            }
        }
    }
}
