use cash_result::{add_call_name_to_chain, operation_succeed, OperationResult};
use dependencies_sync::bson::doc;
use manage_define::general_field_ids::ID_FIELD_ID;

use crate::ids_codes::manage_ids::*;
use crate::ids_codes::field_ids::*;

// 更新登录时间戳
pub async fn update_account_login_timestamps(
    account_id: &String,
    timestamps: &Vec<i64>,
    new_timestapm: i64,
) -> Result<OperationResult, OperationResult> {
    let configs = configs::get_server_configs();

    let mut timestamps = timestamps.clone();
    // 没有超过最大登录限制则加入
    if timestamps.is_empty() || timestamps.len() < configs.login_limit as usize {
        timestamps.push(new_timestapm);
    } else {
        // 更新最晚登录时间戳
        let min = timestamps.iter().min().unwrap();
        let min_index = timestamps.iter().position(|x| x == min).unwrap();
        timestamps[min_index] = new_timestapm;
    }

    let query_doc = doc! {
        ID_FIELD_ID.to_string(): account_id.clone()
    };
    let mut modify_doc = doc! {
        ACCOUNTS_LOGIN_TIMESTAMPS_FIELD_ID.to_string():timestamps
    };

    let result = entity::update_entity_field(
        &ACCOUNTS_MANAGE_ID.to_string(),
        query_doc,
        &mut modify_doc,
        account_id,
    )
    .await;

    match result {
        Ok(_r) => Ok(operation_succeed("ok")),
        Err(e) => Err(add_call_name_to_chain(
            e,
            "update_account_login_timestamps".to_string(),
        )),
    }
}
