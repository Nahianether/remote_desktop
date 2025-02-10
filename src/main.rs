use modules::screen_capture::screen_capture_fl::capture_screen;

pub mod helpers;
pub mod models;
pub mod modules;
pub mod run;

#[tokio::main]
async fn main() {
    run::run().await;
    // task();
}

fn task() {
    capture_screen();
}
