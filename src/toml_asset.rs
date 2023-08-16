use bevy::{
    app::{App, Plugin},
    asset::{AddAsset, AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset},
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

pub struct TomlAssetPlugin;

impl Plugin for TomlAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TomlAsset>()
            .add_asset_loader::<TomlAssetLoader>(Default::default());
    }
}

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct TomlAsset(pub String);

#[derive(Default)]
pub struct TomlAssetLoader;

impl AssetLoader for TomlAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            let data_str = std::str::from_utf8(bytes)?;
            let asset = TomlAsset(data_str.into());
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
