use dependencies_sync::bson::{doc, Document};

use majordomo::get_majordomo;
use managers::ManagerTrait;

use crate::ids_codes::{field_ids::*, manage_ids::INVITES_MANAGE_ID};

pub async fn get_invite_entity(phone: &str, inviter: &str, invite_code: &str) -> Option<Document> {
    let majordomo = get_majordomo();
    let manager = majordomo.get_manager_by_id(INVITES_MANAGE_ID).unwrap();

    let query_doc = doc! {
            INVITES_INVITED_FIELD_ID.to_string():phone.to_owned(),
            INVITES_INVITE_CODE_FIELD_ID.to_string(): invite_code.to_owned(),
            INVITES_INVITER_FIELD_ID.to_string(): inviter.to_owned(),
    };

    let invite_docs = if let Ok(docs) = manager.get_entities_by_filter(&Some(query_doc)).await {
        docs
    } else {
        return None;
    };

    let invite_entity = if let Some(d) = invite_docs.get(0) {
        d
    } else {
        return None;
    };

    Some(invite_entity.to_owned())
}
