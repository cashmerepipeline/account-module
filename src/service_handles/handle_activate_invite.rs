use dependencies_sync::bson::doc;
use dependencies_sync::log::{error, info};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::get_majordomo;
use manage_define::general_field_ids::*;
use managers::ManagerTrait;
use service_utils::types::UnaryResponseResult;
use validates::validate_entity_id;

use crate::account::get_account_passwd_hash;
use crate::account::{self, add_new_account};
use crate::service_handles::get_account_entity_doc::get_account_entity_doc;
use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::{ACCOUNTS_MANAGE_ID, INVITES_MANAGE_ID};
use crate::invite::{check_invite_status, get_invite_entity};
use crate::protocols::{ActivateInviteRequest, ActivateInviteResponse};

#[async_trait]
pub trait HandleActivateInvite {
    async fn handle_activate_invite(
        &self,
        request: Request<ActivateInviteRequest>,
    ) -> UnaryResponseResult<ActivateInviteResponse> {
        // let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        // let token = auth::get_auth_token(metadata).unwrap();
        // let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        // let role_group = auth::get_current_role(metadata).unwrap();

        let area_code = &request.get_ref().area_code;
        let phone = &request.get_ref().phone;
        let inviter = &request.get_ref().inviter;
        let invite_code = &request.get_ref().invite_code;
        let password = &request.get_ref().password;
        let nick_name = &request.get_ref().nick_name;

        // 检查邀请人是否有效
        validate_entity_id(ACCOUNTS_MANAGE_ID, inviter).await?;

        let majordomo_arc = get_majordomo();
        let account_manager = majordomo_arc.get_manager_by_id(ACCOUNTS_MANAGE_ID).unwrap();
        let invite_manager = majordomo_arc.get_manager_by_id(INVITES_MANAGE_ID).unwrap();

        // 创建人设置为自己
        let creator_id = format!("{}{}", area_code, phone);

        // 1. 取得邀请实体
        let query_doc = doc! {
          INVITES_INVITER_FIELD_ID.to_string():inviter.clone(),
          INVITES_INVITED_FIELD_ID.to_string():phone.clone(),
          INVITES_INVITED_FIELD_ID.to_string(): invite_code.clone()
        };
        let invite_entity = if let Some(d) = get_invite_entity(phone, inviter, invite_code).await {
            d
        } else {
            return Err(Status::data_loss(format!(
                "{}: {}, {}",
                t!("取得邀请失败"),
                inviter,
                phone
            )));
        };

        // 检查邀请是否已经激活
        if check_invite_status(&invite_entity) {
            return Err(Status::cancelled(format!("{}: {}", t!("号码已激活"), phone)));
        }

        // 帐号存在性检查
        let query_doc = doc! {
            ID_FIELD_ID.to_string():creator_id.clone()
        };
        if account_manager.entity_exists(&query_doc).await.is_some() {
            return Err(Status::already_exists(format!(
                "{}: {}",
                t!("帐号已存在"),
                phone,
            )));
        }

        // 2. 创建新帐号
        let result = add_new_account(
            area_code, phone, password, nick_name.as_ref(), &creator_id, &"user",
        )
        .await;

        match result {
            Ok(_r) => {
                info!("{}: {}", t!("激活帐号成功"), phone);
                Ok(Response::new(ActivateInviteResponse {
                    result: creator_id,
                }))
            }
            Err(e) => {
                error!("激活帐号发生错误--{}, {}", area_code, phone);
                Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                )))
            }
        }
    }
}
