#[cfg(test)]
mod broadcast_test {
    use crate::helpers::{
        enums::SSReqType,
        lock::broad_cast::{
            add_remove_ss_req_broadcast, get_ss_broadcast_admins, ss_broadcast_is_active,
        },
    };

    #[test]
    #[ignore = "pass"]
    fn a_r_ss_r_b() {
        let admins = get_ss_broadcast_admins(&"client_1".to_string());
        assert_eq!(admins, None);

        add_remove_ss_req_broadcast(
            "admin_1".to_string(),
            "client_1".to_string(),
            SSReqType::Start,
            (1920, 1080),
        );
        let r1 = get_ss_broadcast_admins(&"client_1".to_string());
        assert_eq!(r1, Some(vec!["admin_1".to_string()]));

        add_remove_ss_req_broadcast(
            "admin_2".to_string(),
            "client_1".to_string(),
            SSReqType::Start,
            (1920, 1080),
        );
        let r2 = get_ss_broadcast_admins(&"client_1".to_string());
        assert_eq!(r2, Some(vec!["admin_1".to_string(), "admin_2".to_string()]));

        add_remove_ss_req_broadcast(
            "admin_2".to_string(),
            "client_1".to_string(),
            SSReqType::Stop,
            (1920, 1080),
        );
        let r3 = get_ss_broadcast_admins(&"client_1".to_string());
        assert_eq!(r3, Some(vec!["admin_1".to_string()]));

        add_remove_ss_req_broadcast(
            "admin_1".to_string(),
            "client_1".to_string(),
            SSReqType::Stop,
            (1920, 1080),
        );
        let r4 = get_ss_broadcast_admins(&"client_1".to_string());
        assert_eq!(r4, None);
        assert_eq!(ss_broadcast_is_active("client_1"), false);
    }
}
