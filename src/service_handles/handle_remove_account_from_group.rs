use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use tonic::{Request, Response, Status};

use majordomo::get_majordomo;
use manage_define::general_field_ids::{GROUPS_FIELD_ID, ID_FIELD_ID};
use manage_define::manage_ids::GROUPS_MANAGE_ID;
use managers::entity_interface::EntityInterface;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use validates::validate_name;

use crate::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;
use crate::protocols::{RemoveAccountFromGroupRequest, RemoveAccountFromGroupResponse};

#[async_trait]
pub trait HandleRemoveAccountFromGroup {
    async fn handle_remove_account_from_group(
        &self,
        request: Request<RemoveAccountFromGroupRequest>,
    ) -> UnaryResponseResult<RemoveAccountFromGroupResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_remove_account_from_group)
            .await
    }
}

async fn validate_view_rules(
    request: Request<RemoveAccountFromGroupRequest>,
) -> Result<Request<RemoveAccountFromGroupRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = ACCOUNTS_MANAGE_ID;
        let (account_id, groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_field_can_write(&manage_id, &role_group, GROUPS_FIELD_ID).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<RemoveAccountFromGroupRequest>,
) -> Result<Request<RemoveAccountFromGroupRequest>, Status> {
    Ok(request)
}

async fn handle_remove_account_from_group(
    request: Request<RemoveAccountFromGroupRequest>,
) -> UnaryResponseResult<RemoveAccountFromGroupResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let op_account_id = &request.get_ref().account_id;
    let op_group_id = &request.get_ref().group_id;

    let account_manage_id = ACCOUNTS_MANAGE_ID;
    let group_manage_id = GROUPS_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let account_manager = majordomo_arc.get_manager_by_id(ACCOUNTS_MANAGE_ID).unwrap();

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
