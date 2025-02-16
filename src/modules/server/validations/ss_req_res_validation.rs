use crate::models::share::SSReqRes;
use anyhow::{bail, Result};

pub fn ss_req_validation(v: &SSReqRes) -> Result<()> {
    let mut err = vec![];
    if v.ss_req_type.is_none() {
        err.push("ssReqType is required");
    }

    if v.client_id.is_none() || v.client_id.clone().unwrap().is_empty() {
        err.push("userId is required");
    }

    if !err.is_empty() {
        bail!(err.join(", "))
    }

    Ok(())
}

pub fn ss_res_validation(v: &SSReqRes) -> Result<()> {
    let mut err = vec![];
    if v.frame_size.is_none() {
        err.push("frameSize is required");
    }

    if !err.is_empty() {
        bail!(err.join(", "))
    }

    Ok(())
}
