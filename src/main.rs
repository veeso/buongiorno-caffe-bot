const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod bot;
mod redis;
mod repository;
mod utils;

use bot::Buongiornissimo;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!(
        "buongiorno-caffe-bot v{} - developed by {}",
        APP_VERSION, APP_AUTHORS
    );
    let app = Buongiornissimo::init().await?;
    info!("application ready!");
    app.run().await
}
