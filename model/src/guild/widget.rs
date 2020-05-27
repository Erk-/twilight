use crate::id::ChannelId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GuildWidget {
    pub channel_id: ChannelId,
    pub enabled: bool,
}
