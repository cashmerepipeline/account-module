use dependencies_sync::{
    bson::Document,
    log::error,
    rust_i18n::{self, t},
};

use crate::ids_codes::field_ids::INVITES_INVITE_STATUS_FIELD_ID;

/// 检查邀请状态
pub fn check_invite_status(invite_entity: &Document) -> bool {
    match invite_entity.get_bool(INVITES_INVITE_STATUS_FIELD_ID.to_string()) {
        Ok(invite_status) => invite_status,
        Err(_e) => {
            error!("{}: {:?}", t!("无效的邀请"), invite_entity);

            false
        }
    }
}
