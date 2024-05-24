use cash_result::{operation_failed, OperationResult};
use dependencies_sync::{
    bson::{self, doc},
    log::{error, info},
    rust_i18n::{self, t},
};
use majordomo::get_majordomo;
use managers::entity_interface::EntityInterface;

use tonic::{Response, Status};

use manage_define::{cashmere::Name, general_field_ids::*};

use crate::{
    ids_codes::{field_ids::*, manage_ids::ACCOUNTS_MANAGE_ID},
    protocols::NewAccountResponse,
};

pub async fn add_new_account(
    area_code: &str,
    phone: &str,
    password: &str,
    nick_name: Option<&Name>,
    creator_id: &str,
    creator_role_group: &str,
) -> Result<String, OperationResult> {
    let manage_id = ACCOUNTS_MANAGE_ID;

    let new_account_id = format!("{}{}", area_code, phone);

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(ACCOUNTS_MANAGE_ID).unwrap();

    // 帐号存在性检查
    let query_doc = doc! {
        ID_FIELD_ID.to_string():new_account_id.clone()
    };
    if manager.entity_exists(&query_doc).await.is_some() {
        return Err(operation_failed(
            "add_new_account",
            t!("帐号已经注册，请使用没有注册过的手机号。"),
        ));
    }

    let encrypt_password = auth::jwt::hash_password(&password.to_string()).await.unwrap();

    let mut new_account_doc = doc! {};
    let empty_vec: Vec<String> = vec![];
    let default_name = &Name {
        language: "zh".to_string(),
        name: "默认昵称".to_string(),
    };
    let nick_name = match nick_name {
        Some(n) => n,
        None => default_name,
    };

    new_account_doc.insert(ID_FIELD_ID.to_string(), new_account_id.clone());
    new_account_doc.insert(
        NAME_MAP_FIELD_ID.to_string(),
        doc! {nick_name.language.clone():nick_name.name.clone()},
    );

    new_account_doc.insert(
        DESCRIPTION_FIELD_ID.to_string(),
        bson::to_bson(&"").unwrap(),
    );
    new_account_doc.insert(
        TAGS_FIELD_ID.to_string(),
        bson::to_bson(&empty_vec).unwrap(),
    );

    new_account_doc.insert(REMOVED_FIELD_ID.to_string(), false);
    new_account_doc.insert(ACCOUNTS_PHONE_AREA_CODE_FIELD_ID.to_string(), area_code);
    new_account_doc.insert(ACCOUNTS_PHONE_FIELD_ID.to_string(), phone);
    new_account_doc.insert(ACCOUNTS_PASSWORD_FIELD_ID.to_string(), encrypt_password);

    let result = manager
        .sink_entity(&mut new_account_doc, &creator_id, &creator_role_group)
        .await;

    match result {
        Ok(r) => {
            info!("{}: {}", t!("创建帐号成功"), new_account_id);

            Ok(r)
        }
        Err(e) => {
            error!("{}--{}", t!("创建帐号发生错误"), new_account_id);

            Err(e)
        }
    }
}
