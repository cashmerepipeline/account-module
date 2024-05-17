use dependencies_sync::bson::doc;
use dependencies_sync::log::error;

use dependencies_sync::rust_i18n::{self, t};
use majordomo::get_majordomo;
use managers::ManagerTrait;

use manage_define::general_field_ids::ID_FIELD_ID;
use tonic::Status;

use super::get_account_groups;

const ADMIN_GROUP: &str = "admin";

pub async fn validate_is_admin(account_id: &str) -> Result<(), Status> {
    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(account_id).unwrap();

    let query_doc = doc! {
      ID_FIELD_ID.to_string(): account_id.to_string(),
    };

    // 用户存在性已经检查
    let account_entity = manager
        .get_entity_by_id(account_id, &[], &[])
        .await
        .unwrap();

    match get_account_groups(&account_entity) {
        Some(groups) => {
            if groups.contains(&ADMIN_GROUP.to_string()) {
                return Ok(());
            } else {
                error!("{}: {}", t!("不是管理员"), account_id);

                return Err(Status::permission_denied("您不是管理员"));
            }
        }
        None => return Err(Status::not_found("用户不存在")),
    }
}
