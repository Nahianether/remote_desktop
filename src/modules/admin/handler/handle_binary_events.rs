use crate::helpers::types::WsUserWriter;
use anyhow::Result;
use minifb::Window;
use tokio_tungstenite::tungstenite::Bytes;

pub async fn handle_admin_binary_events(
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

        let mut buffer = vec![0u32; 1920 * 1080];
        for (i, chunk) in received_data.chunks(4).enumerate() {
            let b = chunk[0] as u32;
            let g = chunk[1] as u32;
            let r = chunk[2] as u32;
            buffer[i] = (r << 16) | (g << 8) | b;
        }

        window
            .update_with_buffer(&buffer, 1920, 1080)
            .expect("Failed to update window buffer");
    }
    Ok(())
}
