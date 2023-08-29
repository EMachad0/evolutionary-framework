use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use evolutionary_framework::GameState;

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/crown.png")]
    pub crown: Handle<Image>,
}
