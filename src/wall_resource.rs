use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::World,
    renderer::TextureMetadata,
    renderer::{PngFormat, SpriteSheet, SpriteSheetFormat, Texture},
    utils::application_root_dir,
};

pub struct WallResource {
    pub sprite_sheet: Handle<SpriteSheet>,
}

impl WallResource {
    pub fn initialize(world: &mut World) {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            let wall_texture = format!("{}/texture/wall_texture.png", application_root_dir());
            loader.load(
                wall_texture,
                PngFormat,
                TextureMetadata::srgb(),
                (),
                &texture_storage,
            )
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

            let wall_sprite_sheet =
                format!("{}/texture/wall_sprite_sheet.ron", application_root_dir());
            loader.load(
                wall_sprite_sheet,
                SpriteSheetFormat,
                texture_handle,
                (),
                &sprite_sheet_store,
            )
        };

        let wall_resources = WallResource {
            sprite_sheet: sprite_sheet_handle,
        };

        world.add_resource(wall_resources);
    }
}
