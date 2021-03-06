extern crate amethyst;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, DrawShaded, Pipeline, PosNormTex, RenderBundle, Stage, ALPHA, ColorMask},
    ui::{DrawUi, UiBundle},
    utils::{application_root_dir, fps_counter::FPSCounterBundle},
};

mod bomb;
mod bomb_resource;
mod wall;
mod wall_resource;
mod bomberman;
mod core;
mod player;
mod systems;

use bomberman::Bomberman;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources = format!("{}/resources", application_root_dir());

    let display_config_path = format!("{}/display_config.ron", resources);
    let display_config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None))
            .with_pass(DrawShaded::<PosNormTex>::new())
            .with_pass(DrawUi::new()),
    );

    let bindings_file = format!("{}/bindings_config.ron", resources);

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(bindings_file)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(FPSCounterBundle::default())?
        .with(systems::WallSystem::default(), "wall_system", &[])
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(
            systems::CreateBombSystem::default(),
            "create_bomb_system",
            &["input_system"],
        )
        .with(systems::FPSSystem::default(), "fps_system", &[])
        .with(
            systems::CountDownSystem::default(),
            "count_down_system",
            &[],
        )
        .with(systems::BombTimerSystem, "bomb_timer_system", &[])
        .with(systems::BombExplotionSystem, "bomb_explotion_system", &[]);

    let mut game = Application::build(resources, Bomberman::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();

    Ok(())
}
