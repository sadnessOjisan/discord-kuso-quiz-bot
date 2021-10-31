use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use serenity::{futures::lock::Mutex, model::id::ChannelId, prelude::TypeMapKey};

type QuestionID = i8;
#[derive(Debug, Clone)]
pub struct Question {
    id: QuestionID,
    pub content: String,
    pub answer: String,
}
#[derive(Debug, Clone)]
pub enum Mode {
    Init,
    WaitingUserAnswer(State),
    Finish(State),
    Error,
}

#[derive(Debug, Clone)]
pub struct BotState {
    pub mode: Mode,
}

pub struct AllBotState {
    pub states: HashMap<ChannelId, BotState>,
}

impl TypeMapKey for AllBotState {
    type Value = Arc<Mutex<AllBotState>>;
}

impl AllBotState {
    pub fn new() -> AllBotState {
        AllBotState {
            states: HashMap::new(),
        }
    }
}

impl BotState {
    pub fn new() -> BotState {
        BotState { mode: Mode::Init }
    }

    pub fn initialize_quiz(&mut self) {
        let questions = vec![
            Question {
                id: 1,
                content: "test".to_string(),
                answer: "hoge".to_string(),
            },
            Question {
                id: 2,
                content: "test2".to_string(),
                answer: "hoge2".to_string(),
            },
        ];
        let cursor = 0;
        let result = HashSet::new();
        self.mode = Mode::WaitingUserAnswer(State {
            questions,
            cursor,
            result,
        })
    }

    fn next_quiz(&mut self) {
        match &self.mode {
            Mode::WaitingUserAnswer(state) => {
                let next_cursor = state.cursor + 1;
                let next_state = State {
                    cursor: next_cursor,
                    questions: state.questions.clone(), // Q: string は copy できないから clone するは正しいか
                    result: state.result.clone(),
                };
                // Q: into できなかった
                if next_cursor as usize == state.questions.len() {
                    self.mode = Mode::Finish(next_state);
                    return;
                } else {
                    self.mode = Mode::WaitingUserAnswer(next_state);
                }
            }
            _ => {
                self.mode = Mode::Error;
            }
        }
    }

    fn update_result(&mut self, result: HashSet<QuestionID>) {
        match &self.mode {
            Mode::WaitingUserAnswer(state) => {
                let next_state = State {
                    cursor: state.cursor,
                    questions: state.questions.clone(),
                    result,
                };
                self.mode = Mode::WaitingUserAnswer(next_state)
            }
            _ => {
                self.mode = Mode::Error;
            }
        }
    }

    pub fn user_answer(&mut self, answer: String) -> Option<bool> {
        match &self.mode {
            Mode::WaitingUserAnswer(state) => {
                let current_quiz = state.questions[state.cursor as usize].clone();
                let current_q_answer = current_quiz.answer;
                let is_correct = current_q_answer == answer;
                let mut current_result = state.result.clone();
                if is_correct {
                    current_result.insert(current_quiz.id);
                }
                // Q: この中から直接state.resultに上書きたい
                self.update_result(current_result);
                self.next_quiz();
                Some(is_correct)
            }
            _ => {
                self.mode = Mode::Error;
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub questions: Vec<Question>,
    pub result: HashSet<QuestionID>,
    pub cursor: u8,
}
