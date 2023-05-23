use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::doc;
use dependencies_sync::log::{error, info};
use tonic::{Request, Response, Status};

use majordomo::get_majordomo;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;

use crate::protocols::{ChangeAccountPasswordRequest, ChangeAccountPasswordResponse};

#[async_trait]
pub trait HandleChangeAccountPassword {
    async fn handle_change_account_password(
        &self,
        request: Request<ChangeAccountPasswordRequest>,
    ) -> UnaryResponseResult<ChangeAccountPasswordResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let target_account_id = &request.get_ref().account_id;
        let new_password = &request.get_ref().new_password;

        let manage_id = ACCOUNTS_MANAGE_ID;
        // 实体可写检查
        if !view::can_entity_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated(t!("用户不具有修改实体权限")));
        }
        // 字段可写检查
        if !view::can_field_write(
            &account_id,
            &role_group,
            &manage_id.to_string(),
            &ACCOUNTS_PASSWORD_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated(t!("用户不具有修改密码权限")));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(ACCOUNTS_MANAGE_ID)
            .await
            .unwrap();

        // 帐号存在性检查
        let query_doc = doc! {
            ID_FIELD_ID.to_string():account_id.clone()
        };
        if !manager.entity_exists(&query_doc).await {
            return Err(Status::data_loss(format!(
                "{}: {}",
                t!("帐号不存在"),
                account_id
            )));
        }

        let encrypt_password = auth::jwt::hash_password(new_password).await.unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string(): target_account_id.clone()
        };
        let mut modify_doc = doc! {};
        modify_doc.insert(ACCOUNTS_PASSWORD_FIELD_ID.to_string(), encrypt_password);

        let result = manager
            .update_entity_field(query_doc, &mut modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => {
                info!("{}: {}", t!("更新帐号密码成功"), account_id);
                Ok(Response::new(ChangeAccountPasswordResponse {
                    result: "ok".to_string(),
                }))
            }
            Err(e) => {
                error!("更新帐号密码发生错误--{}", account_id);
                Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                )))
            }
        }
    }
}
