use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::Sender;

// Q: TypeMapKey は他の定義もあるけどこれでいいのか？
use serenity::{
    futures::{lock::Mutex},
    model::id::ChannelId,
    prelude::TypeMapKey,
};

pub struct BotState {
    pub channel_sender_pair: HashMap<ChannelId, Sender<String>>,
}

impl TypeMapKey for BotState {
    type Value = Arc<Mutex<BotState>>;
}
