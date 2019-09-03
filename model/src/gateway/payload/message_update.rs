use crate::{
    channel::{
        embed::Embed,
        message::MessageType,
        Attachment,
    },
    id::{ChannelId, MessageId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageUpdate {
    pub id: MessageId,
    pub attachments: Option<Vec<Attachment>>,
    pub author: Option<User>,
    pub channel_id: ChannelId,
    pub content: Option<String>,
    #[cfg(feature = "chrono")]
    pub edited_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub edited_timestamp: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub kind: Option<MessageType>,
    pub mention_everyone: Option<bool>,
    pub mention_roles: Option<Vec<RoleId>>,
    pub mentions: Option<Vec<User>>,
    pub pinned: Option<bool>,
    #[cfg(feature = "chrono")]
    pub timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub timestamp: Option<String>,
    pub tts: Option<bool>,
}
