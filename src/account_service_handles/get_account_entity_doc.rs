use dependencies_sync::bson::Document;

use cash_result::OperationResult;

use crate::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;
use managers::ManagerTrait;

// 取得账户密钥加密串
pub async fn get_account_entity_doc(account_id: &String) -> Result<Document, OperationResult> {
    let majordomo_lock_arc = majordomo::get_majordomo();
    let manager_arc = majordomo_lock_arc
        .get_manager_by_id(ACCOUNTS_MANAGE_ID)
        .unwrap();

    manager_arc.get_entity_by_id(account_id, &vec![]).await
}
