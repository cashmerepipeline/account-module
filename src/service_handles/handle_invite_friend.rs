use dependencies_sync::bson::doc;
use dependencies_sync::bson::{self, Document};
use dependencies_sync::log::{error, info};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::get_majordomo;
use manage_define::cashmere::Name;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::GROUPS_MANAGE_ID;
use managers::utils::make_new_entity_document;
use managers::ManagerTrait;
use service_utils::types::UnaryResponseResult;
use validates::{validate_entity_id, validate_role_group};

use crate::account::{add_new_account, validate_is_admin};
use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::{ACCOUNTS_MANAGE_ID, INVITES_MANAGE_ID};
use crate::invite::{self, check_invite_limit};
use crate::protocols::{
    InviteFriendRequest, InviteFriendResponse, NewAccountRequest, NewAccountResponse,
};

#[async_trait]
pub trait HandleInviteFriend {
    async fn handle_invite_friend(
        &self,
        request: Request<InviteFriendRequest>,
    ) -> UnaryResponseResult<InviteFriendResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        // 帐号是否存在
        validate_entity_id(ACCOUNTS_MANAGE_ID, &account_id).await?;
        // 组
        validate_entity_id(GROUPS_MANAGE_ID, &role_group).await?;

        let phone = &request.get_ref().phone;

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(INVITES_MANAGE_ID).unwrap();

        // TODO: 是否已经超过邀请上限
        if check_invite_limit(&manager, &account_id, &role_group).await{
            return Err(Status::permission_denied(t!("邀请次数已用完")));
        };

        // 是否已经存在
        let mut query_doc = Document::new();
        query_doc.insert(INVITES_INVITED_FIELD_ID.to_string(), phone);
        query_doc.insert(INVITES_INVITER_FIELD_ID.to_string(), account_id.clone());
        if manager.entity_exists(&query_doc).await.is_some() {
            return Err(Status::already_exists(t!("该手机号已经邀请过了")));
        };

        if let Ok(mut new_doc) = make_new_entity_document(&manager, &account_id).await {
            // 使用objid的后6位作为邀请码
            let oid = new_doc.get_object_id("_id").unwrap().to_hex();
            let invite_code = &oid[oid.len() - 6..];

            new_doc.insert(INVITES_INVITED_FIELD_ID.to_string(), phone.clone());
            new_doc.insert(INVITES_INVITER_FIELD_ID.to_string(), account_id.clone());
            new_doc.insert(INVITES_INVITE_STATUS_FIELD_ID.to_string(), false);
            new_doc.insert(INVITES_INVITE_CODE_FIELD_ID.to_string(), invite_code);
            new_doc.insert(
                INVITES_INVITE_TIME_FIELD_ID.to_string(),
                bson::DateTime::now(),
            );

            let result = manager
                .sink_entity(&mut new_doc, &account_id, &role_group)
                .await;
            match result {
                Ok(r) => Ok(Response::new(InviteFriendResponse { result: r })),
                Err(e) => {
                    error!("{}: {:?}", t!("添加新帐号失败"), e.details());

                    Err(Status::internal(t!("添加新帐号失败")))
                }
            }
        } else {
            error!("{}: {:?}", t!("添加新帐号失败"), t!("无法生成新文档"));
            Err(Status::internal(t!("添加新帐号失败")))
        }
    }
}
