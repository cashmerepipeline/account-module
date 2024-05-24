use configs::ConfigTrait;
use dependencies_sync::bson::Document;
use managers::Manager;
use managers::entity_interface::EntityInterface;


use crate::ids_codes::field_ids::INVITES_INVITER_FIELD_ID;

use super::GroupInviteLimitConfigs;

pub async fn check_invite_limit(manager: &Manager, account_id: &str, role_group: &str) -> bool {
    let limit_count = if let Some(l) = GroupInviteLimitConfigs::get()
        .group_limit_map
        .get(role_group)
    {
        l
    } else {
        // 没有指定上限则不能邀请
        return false;
    };

    let mut query_doc = Document::new();
    query_doc.insert(INVITES_INVITER_FIELD_ID.to_string(), account_id.to_owned());

    let count = if let Ok(c) = manager.count_entity(query_doc).await {
        c
    } else {
        0
    };
    if count >= *limit_count {
        return false;
    }
    true
}
