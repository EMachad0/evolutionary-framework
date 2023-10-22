use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::utils::BoxedFuture;
use serde::Deserialize;

pub struct TxtAssetPlugin;

impl Plugin for TxtAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TxtAsset>()
            .add_asset_loader::<TxtAssetLoader>(Default::default());
    }
}

#[derive(Debug, Deref, DerefMut, Deserialize, TypeUuid, TypePath)]
#[uuid = "7be4b846-288e-4797-ad7a-1ece450736f2"]
pub struct TxtAsset(pub String);

#[derive(Default)]
pub struct TxtAssetLoader;

impl AssetLoader for TxtAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let data_str = std::str::from_utf8(bytes)?;
            let asset = TxtAsset(data_str.into());
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}
