use crate::models::stream_data::SSStreamData;
use anyhow::{bail, Result};

pub fn ss_stream_validation(v: &SSStreamData) -> Result<()> {
    let mut err = vec![];
    if v.bytes.is_none() {
        err.push("Bytes is required");
    }

    if !err.is_empty() {
        bail!(err.join(", "))
    }

    Ok(())
}
