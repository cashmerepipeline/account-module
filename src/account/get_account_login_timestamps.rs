use dependencies_sync::bson::Document;
use crate::ids_codes::field_ids::ACCOUNTS_LOGIN_TIMESTAMPS_FIELD_ID;

// 从doc中取得登录时间戳
pub fn get_account_login_timestamps(doc: &Document) -> Option<Vec<i64>> {
    let timstamps: Vec<i64> = match doc.get_array(&ACCOUNTS_LOGIN_TIMESTAMPS_FIELD_ID.to_string()) {
        Ok(ss) => ss.iter().map(|x| x.as_i64().unwrap()).collect(),
        Err(_e) => vec![],
    };

    Some(timstamps)
}
