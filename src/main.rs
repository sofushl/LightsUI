mod components;
mod config;
mod layout;
mod pages;
mod request;
mod routes;

use dotenvy::dotenv;
use tokio::net::TcpListener;
use topcoat::{
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{Router, RouterBuilderDiscoverExt},
    runtime::RouterBuilderRuntimeExt,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let conf = config::Config::load();
    println!("{}", conf.ip);
    println!("{}", conf.port);

    let router = Router::builder()
        .discover()
        .assets(AssetBundle::load().expect("Asset load failed"))
        .runtime()
        .build();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", conf.port))
        .await
        .unwrap();

    topcoat::serve(listener, router).await.unwrap();
}
