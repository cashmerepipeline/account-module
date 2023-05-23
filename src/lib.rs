/*
Author: 闫刚 (yes7rose@sina.com)
account_server.rs (c) 2021
Desc: 账户服务
Created:  2021-11-27T14:00:58.767Z
Modified: !date!
*/

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod account;
mod account_service_handles;
pub mod protocols;
pub mod managers;
pub mod account_server;
pub mod ids_codes;

