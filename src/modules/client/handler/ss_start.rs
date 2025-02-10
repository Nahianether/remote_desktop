use std::{
    thread,
    time::{Duration, Instant},
};

use futures_util::SinkExt;
use scrap::Display;
use tokio::runtime::Runtime;

use crate::{
    helpers::{
        constraint::{constraint::FPS_LIMIT, flags::SS_RESPONSE},
        lock::{
            broad_cast::{get_client_boradcast_enable, set_client_boradcast_enable},
            client_buffer::add_bytes_in_client_buffer,
        },
        types::WsUserWriter,
    },
    models::share::SSReqRes,
    modules::screen_capture::screen_capture_fl::capture_screen,
};

pub fn client_ss_start(writer: &mut WsUserWriter) {
    match Display::primary() {
        Ok(display) => {
            let ss_res = SSReqRes::new(
                SS_RESPONSE,
                None,
                None,
                Some((display.width(), display.height())),
            );
            Runtime::new()
                .unwrap()
                .block_on(async { writer.send(ss_res.to_ws().unwrap()).await })
                .unwrap(); // sending the frame size to the server

            set_client_boradcast_enable(true);

            thread::spawn(move || {
                let frame_time = Duration::from_secs_f64(1.0 / FPS_LIMIT as f64);
                loop {
                    let start_time = Instant::now();

                    if get_client_boradcast_enable() == false {
                        break;
                    }

                    let screen_data = capture_screen();
                    if screen_data.is_empty() {
                        continue;
                    }
                    add_bytes_in_client_buffer(screen_data);
                    let elapsed = start_time.elapsed();
                    if elapsed < frame_time {
                        thread::sleep(frame_time - elapsed);
                    }
                }
            });
        }
        Err(e) => {
            println!("Failed to find primary display with : {:?}", e);
            return;
        }
    }
}
