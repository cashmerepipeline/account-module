use std::{collections::HashMap, sync::OnceLock};

use dependencies_sync::rust_i18n::{self, t};

use configs::{get_config, ConfigTrait};
use serde_derive::{Deserialize, Serialize};

const GROUP_INVITE_LIMIT_CONFIGS_NAME: &str = "group_invite_limit";

static GROUP_INVITE_LIMIT_CONFIGS: OnceLock<GroupInviteLimitConfigs> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInviteLimitConfigs {
    pub group_limit_map: HashMap<String, u64>,
}

impl ConfigTrait for GroupInviteLimitConfigs {
    fn name() -> &'static str {
        GROUP_INVITE_LIMIT_CONFIGS_NAME
    }
    fn get() -> &'static Self {
        GROUP_INVITE_LIMIT_CONFIGS.get_or_init(|| {
            get_config::<GroupInviteLimitConfigs>().expect(t!("取得配置失败").as_str())
        })
        /*         if let Some(configs) = GROUP_INVITE_LIMIT_CONFIGS.get() {
            return configs;
        } else {
            let configs = get_config::<GroupInviteLimitConfigs>().expect(t!("取得配置失败").as_str());
            NOTICE_SYSTEM_CONFIGS.set(configs).expect("设置配置失败");
        }

        NOTICE_SYSTEM_CONFIGS.get().unwrap() */
    }
}

impl Default for GroupInviteLimitConfigs {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("admin".to_string(), 999_999_999_999);
        map.insert("usr".to_string(), 20);
        map.insert("store".to_string(), 100_000_000);
        map.insert("public".to_string(), 0);

        Self {
            group_limit_map: map,
        }
    }
}
