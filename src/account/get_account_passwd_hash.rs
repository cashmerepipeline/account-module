use dependencies_sync::bson::Document;
use crate::ids_codes::field_ids::ACCOUNTS_PASSWORD_FIELD_ID;

// 从doc中取得password
pub fn get_account_passwd_hash(doc: &Document) -> Option<String> {
    let pw_value = doc.get_str(&ACCOUNTS_PASSWORD_FIELD_ID.to_string());

    match pw_value {
        Ok(s) => Some(s.to_string()),
        Err(_e) => None,
    }
}
