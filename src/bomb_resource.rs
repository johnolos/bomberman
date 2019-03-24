use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::World,
    renderer::{PngFormat, SpriteSheet, SpriteSheetFormat, Texture},
    renderer::TextureMetadata,
    utils::application_root_dir
};

pub struct BombResource {
  pub sprite_sheet: Handle<SpriteSheet>
}


impl BombResource {
  pub fn initialize(world: &mut World) {
    let texture_handle = {
      let loader = world.read_resource::<Loader>();
      let texture_storage = world.read_resource::<AssetStorage<Texture>>();
      let bomb_texture = format!(
        "{}/texture/bomb_texture.png",
        application_root_dir()
      );
      loader.load(
        bomb_texture,
        PngFormat,
        TextureMetadata::srgb(),
        (),
        &texture_storage
      )
    };

    let sprite_sheet_handle = {
      let loader = world.read_resource::<Loader>();
      let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

      let bomb_sprite_sheet = format!(
          "{}/texture/bomb_sprite_sheet.ron",
          application_root_dir()
      );
      loader.load(
          bomb_sprite_sheet,
          SpriteSheetFormat,
          texture_handle,
          (),
          &sprite_sheet_store,
      )
    };

    let bomb_resources = BombResource {
      sprite_sheet: sprite_sheet_handle
    };

    world.add_resource(bomb_resources);
  }
}
