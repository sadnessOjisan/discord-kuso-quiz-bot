use std::{borrow::Cow, collections::HashMap, sync::Arc};
use tokio::sync::mpsc::Sender;

// Q: TypeMapKey は他の定義もあるけどこれでいいのか？
use serenity::{futures::lock::Mutex, model::id::ChannelId, prelude::TypeMapKey};

#[derive(Default)]
pub struct BotState {
    pub channel_sender_pair: HashMap<ChannelId, Sender<Cow<'static, str>>>,
}

impl TypeMapKey for BotState {
    type Value = Arc<Mutex<BotState>>;
}
