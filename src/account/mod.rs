/*
Author: 闫刚 (yes7rose@sina.com)
mod.rs (c) 2020
Desc:  账号管理模块
Created:  2020-09-17T01:00:33.046Z
Modified: !date!
*/

pub use update_account_login_timestamps::*;
pub use account_status::*;
pub use login_status::*;
pub use get_account_by_id::*;
pub use get_account_passwd_hash::*;
pub use get_account_groups::*;
pub use is_account_stopped::*;
pub use get_account_login_timestamps::*;

pub mod group;
mod update_account_login_timestamps;
mod account_status;
mod login_status;
mod get_account_by_id;
mod get_account_passwd_hash;
mod get_account_groups;
mod is_account_stopped;
mod get_account_login_timestamps;

// pub fn get_account_manage_view_rules(doc: &Document) -> Option<Vec<String>>{
//     let view_rules:Vec<String> = match doc.get_array("view_rules") {
//         Ok(ss) => ss.iter().map(|x| x.as_i64().unwrap()).collect(),
//         Err(_e) => vec![]
//     };
//     Some(view_rules)
// }
