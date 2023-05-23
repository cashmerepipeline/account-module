use dependencies_sync::bson::Document;
use manage_define::general_field_ids::GROUPS_FIELD_ID;

// 从doc中取得groups
pub fn get_account_groups(doc: &Document) -> Option<Vec<String>> {
    let groups: Vec<String> = match doc.get_array(&GROUPS_FIELD_ID.to_string()) {
        Ok(g) => g.iter().map(|x| x.as_str().unwrap().to_string()).collect(),
        Err(_e) => return None,
    };

    Some(groups)
}
