use std::any::{Any, TypeId};

use crate::{get_auth_token, get_claims_account_and_roles, get_current_role};

use crate::jwt::{get_claims, validate_jwt_token};
use dependencies_sync::chrono::Utc;
use dependencies_sync::tonic::{Request, Status};
use dependencies_sync::log::debug;
use dependencies_sync::rust_i18n::{self, t};

use manage_define::cashmere::GetPhoneAreaCodesRequest;

///检查授权token
pub fn check_auth_token(request: Request<()>) -> Result<Request<()>, Status> {
    // token检查
    let auth_meta = request.metadata();
    let auth_token = match get_auth_token::get_auth_token(auth_meta) {
        Some(token) => token,
        None => return Err(Status::unauthenticated(t!("请先登录"))),
    };

    // token校验
    if !validate_jwt_token(&auth_token) {
        return Err(Status::unauthenticated(format!(
            "{}, {}",
            t!("权限验证错误"),
            t!("请重新登录")
        )));
    }

    let (_account_id, _groups) =
        match get_claims_account_and_roles::get_claims_account_and_roles(&auth_token) {
            Some(r) => r,
            None => return Err(Status::unauthenticated(t!("请先登录"))),
        };

    let claims = match get_claims(&auth_token) {
        Some(r) => r,
        None => return Err(Status::unauthenticated(t!("请先登录"))),
    };

    // 过期校验
    let now = Utc::now().timestamp();
    if now > claims.exp as i64 {
        return Err(Status::cancelled(format!(
            "{}, {}",
            t!("登录已过期"),
            t!("请重新登录")
        )));
    }

    // 检查当前使用角色组是否在组表中存在
    if let Some(ref r) = get_current_role(auth_meta) {
        if !_groups.contains(r) {
            return Err(Status::unauthenticated(t!("没有授权组")));
        }
    } else {
        return Err(Status::data_loss(t!("需要指定所用角色组")));
    };

    // 3. 登录限制校验, mongodb使用async-std
    let account_doc = match get_account_by_id(&account_id)
    {
        Ok(r) => r,
        Err(_e) => return Err(Status::data_loss("账号不存在"))
    };
    
    let timestamps = match account::get_account_login_timestamps(&account_doc) {
        Some(r) => r,
        None => return Err(Status::data_loss("请先登录"))
    };
    for stamp in timestamps {
        if stamp > claims.exp as i64{
            return Err(Status::cancelled("登录过期，可能已在不同设备登录"));
        }
    }
    
    // 4. 账户是否停用
    if account::is_account_stopped(&account_doc) {
        return Err(Status::unauthenticated("账号已停用"));
    }

    Ok(request)
}

