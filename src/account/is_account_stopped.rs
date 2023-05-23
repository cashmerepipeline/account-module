use dependencies_sync::bson::Document;

/// 取得账号状态
pub fn is_account_stopped(doc: &Document) -> bool {
    match doc.get_bool("stopped") {
        Ok(g) => g,
        Err(_e) => false,
    }
}
