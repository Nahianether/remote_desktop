use super::user::UserInfo;

impl UserInfo {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn default() -> Self {
        Self {
            user_id: None,
            user_mode: None,
            addr: None,
        }
    }
}
