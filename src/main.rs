mod components;
mod layout;
mod pages;
mod routes;
mod triangle;

use topcoat::{
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{Router, RouterBuilderDiscoverExt},
};

#[tokio::main]
async fn main() {
    topcoat::start(
        Router::builder()
            .discover()
            .assets(AssetBundle::load().expect("Asset load failed"))
            .build(),
    )
    .await
    .unwrap();
}
