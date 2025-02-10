use crate::helpers::types::WsUserWriter;
use anyhow::Result;
use minifb::Window;
use tokio_tungstenite::tungstenite::Bytes;

pub fn handle_admin_binary_events(
    window: &mut Window,
    _writer: &mut WsUserWriter,
    bytes: Bytes,
    _addr: &str,
) -> Result<()> {
    if window.is_open() {
        let received_data: Vec<u32> = bytes
            .chunks_exact(4)
            .map(|chunk| u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();

        let mut buffer = vec![0u32; 1280 * 720];

        let min_pixels = received_data.len().min(1280 * 720);

        for i in 0..min_pixels {
            let pixel = received_data[i];
            let b = (pixel & 0xFF) as u32;
            let g = ((pixel >> 8) & 0xFF) as u32;
            let r = ((pixel >> 16) & 0xFF) as u32;
            buffer[i] = (r << 16) | (g << 8) | b;
        }

        window
            .update_with_buffer(&buffer, 1280, 720)
            .expect("Failed to update window buffer");
    }
    Ok(())
}
