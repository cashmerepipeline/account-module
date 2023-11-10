use dependencies_sync::bson;
use dependencies_sync::bson::doc;
use dependencies_sync::log::{error, info};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::get_majordomo;
use manage_define::cashmere::Name;
use manage_define::general_field_ids::*;
use managers::ManagerTrait;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;
use crate::protocols::{NewAccountRequest, NewAccountResponse};

#[async_trait]
pub trait HandleNewAccount {
    async fn handle_new_account(
        &self,
        request: Request<NewAccountRequest>,
    ) -> UnaryResponseResult<NewAccountResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let area_code = &request.get_ref().area_code;
        let phone = &request.get_ref().phone;
        let password = &request.get_ref().password;
        let nick_name = &request.get_ref().nick_name;

        let manage_id = ACCOUNTS_MANAGE_ID;

        let new_account_id = format!("{}{}", area_code, phone);

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(ACCOUNTS_MANAGE_ID).unwrap();

        // 帐号存在性检查
        let query_doc = doc! {
            ID_FIELD_ID.to_string():new_account_id.clone()
        };
        if manager.entity_exists(&query_doc).await.is_some() {
            return Err(Status::already_exists(
                "手机已经注册，请使用没有注册过的手机号。",
            ));
        }

        let encrypt_password = auth::jwt::hash_password(password).await.unwrap();

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
            .sink_entity(&mut new_account_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(_r) => {
                info!("{}: {}", t!("创建帐号成功"), new_account_id);

                Ok(Response::new(NewAccountResponse {
                    result: new_account_id,
                }))
            }
            Err(e) => {
                error!("{}--{}", t!("创建帐号发生错误"), new_account_id);

                Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                )))
            }
        }
    }
}
