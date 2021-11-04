use std::collections::HashSet;

// (問題数、正答数)
type Summary = (usize, usize);

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
pub struct QuizState {
    pub mode: Mode,
}

impl QuizState {
    pub fn new() -> QuizState {
        QuizState { mode: Mode::Init }
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

impl State {
    pub fn new() -> Self {
        State {
            questions: vec![
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
            ],
            result: HashSet::new(),
            cursor: 0,
        }
    }

    pub fn get_current_question(&self) -> &Question {
        &self.questions[self.cursor as usize]
    }

    pub fn check_user_answer(&mut self, answer: &String) -> bool {
        let current_question = &self.questions[self.cursor as usize];
        let target_answer = &current_question.answer;
        // Q: 参照同士の比較でも大丈夫？
        let is_correct = target_answer.to_string() == answer.to_string();
        if is_correct {
            self.result.insert(current_question.id);
        }
        is_correct
    }

    pub fn next_question(&mut self) -> () {
        let next_cursor = self.cursor + 1;
        self.cursor = next_cursor;
    }

    pub fn is_last(&self) -> bool {
        // Q: into すると `type annotations needed cannot satisfy `usize: PartialEq<_>` と怒られるのどうにかしたい。
        self.questions.len() == (self.cursor + 1) as usize
    }

    pub fn summary_result(&self) -> Summary {
        (self.questions.len(), self.result.len())
    }
}
