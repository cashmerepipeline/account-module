/*
    Author: 闫刚 (yes7rose@sina.com)
    account_service_handles.rs (c) 2021
    Desc: 账户管理服务
    Created:  2021-03-29T05:20:05.474Z
    Modified: !date!
*/

pub use handle_add_account_into_group::HandleAddAccountIntoGroup;
pub use handle_remove_account_from_group::HandleRemoveAccountFromGroup;

pub use handle_change_account_password::*;
pub use handle_change_account_status::*;
pub use handle_change_own_password::HandleChangeOwnPassword;
pub use handle_login::HandleLogin;
pub use handle_new_account::HandleNewAccount;

pub use handle_invite_friend::HandleInviteFriend;
pub use handle_activate_invite::HandleActivateInvite;

mod get_account_entity_doc;

mod handle_login;
mod handle_new_account;
mod handle_change_own_password;
mod handle_change_account_status;
mod handle_change_account_password;

mod handle_add_account_into_group;
mod handle_remove_account_from_group;

mod handle_invite_friend;
mod handle_activate_invite;
