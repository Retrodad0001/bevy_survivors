use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct AssetInfo {
    pub(crate) image: Handle<Image>,
    pub(crate) texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl AssetInfo {
    pub(crate) fn new(
        image: Handle<Image>,
        texture_atlas_layout: Handle<TextureAtlasLayout>,
    ) -> AssetInfo {
        AssetInfo {
            image,
            texture_atlas_layout,
        }
    }
}
