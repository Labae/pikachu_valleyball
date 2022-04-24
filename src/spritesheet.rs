use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{formats::texture::ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};

pub fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/sprite_sheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
