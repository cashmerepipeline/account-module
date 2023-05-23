use dependencies_sync::tonic::{Request, Response, Status};
use service_utils::types::UnaryResponseResult;

use crate::protocols::account_grpc_server::AccountGrpc;
use crate::protocols::*;

use crate::account_service_handles::*;

/// 账号服务
#[derive(Default)]
pub struct AccountServer;

impl HandleLogin for AccountServer {}
impl HandleNewAccount for AccountServer {}
impl HandleAddAccountIntoGroup for AccountServer {}
impl HandleRemoveAccountFromGroup for AccountServer {}
impl HandleChangeOwnPassword for AccountServer {}
impl HandleChangeAccountStatus for AccountServer {}
impl HandleChangeAccountPassword for AccountServer {}

#[tonic::async_trait]
impl AccountGrpc for AccountServer {
    async fn login(&self, request: Request<LoginRequest>) -> UnaryResponseResult<LoginResponse> {
        self.handle_login(request).await
    }

    async fn new_account(
        &self,
        request: Request<NewAccountRequest>,
    ) -> Result<Response<NewAccountResponse>, Status> {
        self.handle_new_account(request).await
    }

    async fn add_account_into_group(
        &self,
        request: Request<AddAccountIntoGroupRequest>,
    ) -> Result<Response<AddAccountIntoGroupResponse>, Status> {
        self.handle_add_account_into_group(request).await
    }

    async fn remove_account_from_group(
        &self,
        request: Request<RemoveAccountFromGroupRequest>,
    ) -> Result<Response<RemoveAccountFromGroupResponse>, Status> {
        self.handle_remove_account_from_group(request).await
    }

    async fn change_own_password(
        &self,
        request: Request<ChangeOwnPasswordRequest>,
    ) -> Result<Response<ChangeOwnPasswordResponse>, Status> {
        self.handle_change_own_password(request).await
    }

    async fn change_account_status(
        &self,
        request: Request<ChangeAccountStatusRequest>,
    ) -> Result<Response<ChangeAccountStatusResponse>, Status> {
        self.handle_change_account_status(request).await
    }

    async fn change_account_password(
        &self,
        request: Request<ChangeAccountPasswordRequest>,
    ) -> Result<Response<ChangeAccountPasswordResponse>, Status> {
        self.handle_change_account_password(request).await
    }
}
