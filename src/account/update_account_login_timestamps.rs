use cash_result::{add_call_name_to_chain, operation_succeed, OperationResult};
use dependencies_sync::bson::doc;
use dependencies_sync::log::error;
use manage_define::general_field_ids::ID_FIELD_ID;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;

// 更新登录时间戳
pub async fn update_account_login_timestamps(
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    let configs = configs::get_server_configs();

    let query_doc = doc! {
        ID_FIELD_ID.to_string(): account_id.clone()
    };

    let mut modify_doc = doc! {
        ACCOUNTS_LOGIN_TIMESTAMPS_FIELD_ID.to_string():{"$type":"timestamp"}
    };

    let result = entity::update_timestamp_field(
        &ACCOUNTS_MANAGE_ID.to_string(),
        query_doc,
        &mut modify_doc,
        account_id,
    )
    .await;

    match result {
        Ok(_r) => Ok(operation_succeed("ok")),
        Err(e) => {
            error!("{}", e.details());

            Err(add_call_name_to_chain(
                e,
                "update_account_login_timestamps".to_string(),
            ))
        }
    }
}
