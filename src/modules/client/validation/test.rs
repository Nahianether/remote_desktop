#[cfg(test)]
mod vcmt {
    use tokio_tungstenite::tungstenite::Message;

    use crate::modules::client::validation::msg_validation::validate_client_message_type;

    #[test]
    #[ignore = "pass"]
    fn test() {
        let json_str = r#"{
        "flag" : "ss_request",    
        "ssReqType" : "start",    
        "clientId" : "1@client.user",
        "frameSize" : null
    }"#;
        let m = Message::text(json_str);
        let v = validate_client_message_type(m);
        assert_eq!(v.is_ok(), true);
    }
}
