use std::{collections::HashSet, sync::Arc};

use serenity::{futures::lock::Mutex, prelude::TypeMapKey};

struct Quiz {
    id: u8,
    content: String,
    answer: String,
}

pub struct ChannelState {
    quizzes: Vec<Quiz>,
    currentIdx: usize,
    result: HashSet<u8>
}

// 同じ crate でしか impl できない
impl TypeMapKey for ChannelState {
    type Value = Arc<Mutex<ChannelState>>;
}


impl ChannelState {
   pub fn new() -> Self {
       ChannelState {
           quizzes: vec![
               Quiz {
                   id: 1,
                   content: "aaa".to_string(),
                   answer: "bbb".to_string()
               }
           ],
           currentIdx: 0,
           result: HashSet::new()
       }
    }
   

}