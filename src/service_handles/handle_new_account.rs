use dependencies_sync::bson;
use dependencies_sync::bson::doc;
use dependencies_sync::log::{error, info};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::get_majordomo;
use manage_define::cashmere::Name;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::GROUPS_MANAGE_ID;
use managers::ManagerTrait;
use service_utils::types::UnaryResponseResult;
use validates::{validate_entity_id, validate_role_group};

use crate::account::{add_new_account, validate_is_admin};
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
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        // 管理员帐号是否存在
        validate_entity_id(ACCOUNTS_MANAGE_ID, &account_id).await?;
        // 管理员组
        validate_entity_id(GROUPS_MANAGE_ID, &role_group).await?;
        // 验证是否管理员
        validate_is_admin(&account_id).await?;

        let area_code = &request.get_ref().area_code;
        let phone = &request.get_ref().phone;
        let password = &request.get_ref().password;
        let nick_name = &request.get_ref().nick_name;

        let result = add_new_account(
            area_code, phone, password, nick_name.as_ref(), &account_id, &role_group,
        )
        .await;

        match result {
            Ok(r) => Ok(Response::new(NewAccountResponse { result: r })),
            Err(e) => {
                error!("{}: {:?}", t!("添加新帐号失败"), e.details());

                Err(Status::internal(t!("添加新帐号失败")))
            }
        }
    }
}
