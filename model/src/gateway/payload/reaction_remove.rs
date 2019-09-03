use crate::channel::Reaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReactionRemove(pub Reaction);
