use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    core::transform::Transform,
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata,
    },
    ui::UiCreator,
    utils::application_root_dir,
};

use crate::{
    bomb_resource::BombResource,
    wall_resource::WallResource,
    core::{Orientation, PlayerNumber},
    player::{Player, PLAYER_HEIGHT, PLAYER_WIDTH},
};

pub const ARENA_HEIGHT: f32 = 1080.0;
pub const ARENA_WIDTH: f32 = 1920.0;

#[derive(Default)]
pub struct Bomberman {
    progress: ProgressCounter,
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_players(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut left_player_transformation = Transform::default();
    let mut right_player_transformation = Transform::default();
    let y = ARENA_HEIGHT - (PLAYER_HEIGHT * 0.5);

    left_player_transformation.set_xyz(PLAYER_WIDTH * 0.5, y, 0.0);
    right_player_transformation.set_xyz(ARENA_WIDTH - (PLAYER_WIDTH * 0.5), y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2,
    };

    world
        .create_entity()
        .with(Player::new(PlayerNumber::PlayerOne, Orientation::South))
        .with(left_player_transformation)
        .with(sprite_render.clone())
        .build();

    world
        .create_entity()
        .with(Player::new(PlayerNumber::PlayerTwo, Orientation::South))
        .with(right_player_transformation)
        .with(sprite_render.clone())
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let texture_sprite_sheet_path = format!("{}/texture/bomberman.png", application_root_dir());

        loader.load(
            texture_sprite_sheet_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    let texture_sprite_sheet_spec = format!(
        "{}/texture/bomberman_sprite_sheet.ron",
        application_root_dir()
    );
    loader.load(
        texture_sprite_sheet_spec, // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

impl SimpleState for Bomberman {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world);

        let sprite_sheet_handle = load_sprite_sheet(world);
        BombResource::initialize(world);
        WallResource::initialize(world);

        let fps_path = format!("{}/resources/fps.ron", application_root_dir());
        let count_down_path = format!("{}/resources/count_down.ron", application_root_dir());

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create(fps_path, &mut self.progress);
            creator.create(count_down_path, &mut self.progress);
        });

        initialise_players(world, sprite_sheet_handle);
    }
}
