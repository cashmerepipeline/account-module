use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;
use tonic::{Request, Response, Status};

use majordomo::get_majordomo;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::GROUPS_MANAGE_ID;
use managers::traits::ManagerTrait;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;
use crate::protocols::{AddAccountIntoGroupRequest, AddAccountIntoGroupResponse};

#[async_trait]
pub trait HandleAddAccountIntoGroup {
    async fn handle_add_account_into_group(
        &self,
        request: Request<AddAccountIntoGroupRequest>,
    ) -> UnaryResponseResult<AddAccountIntoGroupResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let op_account_id = &request.get_ref().account_id;
        let op_group_id = &request.get_ref().group_id;

        let account_manage_id = ACCOUNTS_MANAGE_ID;
        let group_manage_id = GROUPS_MANAGE_ID;

        // 管理可写性
        if !view::can_manage_write(&account_id, &role_group, &account_manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有帐号可写权限"));
        }
        // 实际不写入到组中
        if !view::can_manage_write(&account_id, &role_group, &group_manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有组可写权限"));
        }

        // 集合可写性检查
        if !view::can_collection_write(&account_id, &role_group, &account_manage_id.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有集合可写权限"));
        }
        if !view::can_collection_write(&account_id, &role_group, &group_manage_id.to_string()).await
        {
            return Err(Status::unauthenticated("用户不具有集合可写权限"));
        }
        // 检查帐号组属性字段可写性
        if !view::can_field_write(
            &account_id,
            &role_group,
            &account_manage_id.to_string(),
            &GROUPS_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有字段可写权限"));
        }

        let majordomo_arc = get_majordomo();
        let account_manager = majordomo_arc
            .get_manager_by_id(ACCOUNTS_MANAGE_ID)
            .unwrap();

        let group_manager = majordomo_arc
            .get_manager_by_id(GROUPS_MANAGE_ID)
            .unwrap();

        // 检查组是否存在
        if !group_manager
            .entity_exists(&doc! {ID_FIELD_ID.to_string():op_group_id.clone()})
            .await
        {
            return Err(Status::data_loss(format!("组不存在:{}", op_group_id)));
        }
        // 检查组是否标记为移除
        let group_entity = group_manager.get_entity_by_id(op_group_id).await.unwrap();
        if group_entity
            .get_bool(ENTITY_REMOVED_FIELD_ID.to_string())
            .unwrap()
        {
            return Err(Status::cancelled("组已经被移除"));
        }

        let query_doc = doc! {
            ID_FIELD_ID.to_string():op_account_id.clone()
        };
        let modify_doc = doc! {
            GROUPS_FIELD_ID.to_string():op_group_id
        };

        let result = account_manager
            .add_to_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AddAccountIntoGroupResponse {
                result: _r.details(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
