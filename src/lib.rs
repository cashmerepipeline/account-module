/*
Author: 闫刚 (yes7rose@sina.com)
account_server.rs (c) 2021
Desc: 账户服务
Created:  2021-11-27T14:00:58.767Z
Modified: !date!
*/

use dependencies_sync::once_cell;
use dependencies_sync::rust_i18n::{self, i18n, t};
i18n!("locales");

mod account;
mod account_service_handles;
pub mod protocols;
pub mod managers;
pub mod account_server;
pub mod ids_codes;

