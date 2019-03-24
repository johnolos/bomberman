extern crate amethyst;

use amethyst::{
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    core::transform::TransformBundle,
    utils::application_root_dir
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

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let bindings_file = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(bindings_file)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::CreateBombSystem, "create_bomb_system", &["input_system"]);

    let mut game = Application::new("./", Bomberman, game_data)?;

    game.run();

    Ok(())
}
