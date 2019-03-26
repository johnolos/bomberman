extern crate amethyst;

use amethyst::{
    input::InputBundle,
    prelude::*,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, DrawShaded, PosNormTex},
    ui::{UiBundle, DrawUi},
    utils::{
        application_root_dir,
        fps_counter::{FPSCounterBundle},
    },
};

mod bomb;
mod bomb_resource;
mod bomberman;
mod core;
mod player;
mod systems;

use bomberman::Bomberman;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_config_path = format!("{}/resources/display_config.ron", application_root_dir());
    let display_config = DisplayConfig::load(&display_config_path);
    let resources = format!("{}/texture", application_root_dir());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawShaded::<PosNormTex>::new())
            .with_pass(DrawUi::new()),
    );

    let bindings_file = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(bindings_file)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(FPSCounterBundle::default())?
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::CreateBombSystem, "create_bomb_system", &["input_system"])
        .with(systems::FPSSystem::default(), "fps_system", &[])
        .with(systems::BombTimerSystem, "bomb_timer_system", &[])
        .with(systems::BombExplotionSystem, "bomb_explotion_system", &[]);

    let mut game = Application::build(resources, Bomberman::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();

    Ok(())
}
