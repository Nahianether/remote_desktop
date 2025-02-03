pub mod helpers;
pub mod models;
pub mod modules;
pub mod run;

#[tokio::main]
async fn main() {
    run::run().await;
}
