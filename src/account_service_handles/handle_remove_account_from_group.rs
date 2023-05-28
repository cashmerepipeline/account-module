use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;
use tonic::{Request, Response, Status};

use majordomo::get_majordomo;
use manage_define::general_field_ids::{GROUPS_FIELD_ID, ID_FIELD_ID};
use manage_define::manage_ids::GROUPS_MANAGE_ID;
use managers::traits::ManagerTrait;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;
use crate::protocols::{RemoveAccountFromGroupRequest, RemoveAccountFromGroupResponse};

#[async_trait]
pub trait HandleRemoveAccountFromGroup {
    async fn handle_remove_account_from_group(
        &self,
        request: Request<RemoveAccountFromGroupRequest>,
    ) -> UnaryResponseResult<RemoveAccountFromGroupResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let op_account_id = &request.get_ref().account_id;
        let op_group_id = &request.get_ref().group_id;

        let account_manage_id = ACCOUNTS_MANAGE_ID;
        let group_manage_id = GROUPS_MANAGE_ID;

        let majordomo_arc = get_majordomo();
        let account_manager = majordomo_arc
            .get_manager_by_id(ACCOUNTS_MANAGE_ID)
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():op_account_id.clone()
        };
        let modify_doc = doc! {
            GROUPS_FIELD_ID.to_string():op_group_id
        };

        let result = account_manager
            .remove_from_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(RemoveAccountFromGroupResponse {
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
