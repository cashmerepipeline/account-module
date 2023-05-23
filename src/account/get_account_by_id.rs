use dependencies_sync::bson::Document;
use cash_result::OperationResult;
use crate::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;

pub async fn get_account_by_id(id: &String) -> Result<Document, OperationResult> {
    entity::get_entity_by_id(&ACCOUNTS_MANAGE_ID.to_string(), id).await
}
