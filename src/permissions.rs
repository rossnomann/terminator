use carapax::types::{ChatMember, ChatPermissions};

/// A key to store old chat member permissions in session
pub const PERMISSIONS_SESSION_KEY: &str = "permissions";

pub fn obtain_chat_member_permissions(member: ChatMember) -> ChatPermissions {
    use self::ChatMember::*;
    match member {
        Administrator(_) | Creator(_) | Left(_) | Member(_) => ChatPermissions::allowed(),
        Kicked(_) => ChatPermissions::restricted(),
        Restricted(restricted) => ChatPermissions {
            can_send_messages: Some(restricted.can_send_messages),
            can_send_media_messages: Some(restricted.can_send_media_messages),
            can_send_polls: Some(restricted.can_send_polls),
            can_send_other_messages: Some(restricted.can_send_other_messages),
            can_add_web_page_previews: Some(restricted.can_add_web_page_previews),
            can_change_info: Some(restricted.can_change_info),
            can_invite_users: Some(restricted.can_invite_users),
            can_pin_messages: restricted.can_pin_messages,
        },
    }
}
