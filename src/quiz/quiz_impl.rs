use indexmap::{indexmap, indexset, IndexMap, IndexSet};

// (問題数、正答数)
type Summary = (usize, usize);

#[derive(Debug, Clone)]
pub struct Question {
    pub content: String,
    pub answer: String,
}

#[derive(Debug, Clone)]
pub struct Quiz {
    questions: IndexMap<usize, Question>,
}

impl Default for Quiz {
    fn default() -> Self {
        Self::new()
    }
}

impl Quiz {
    pub fn new() -> Self {
        Self {
            questions: indexmap! {
                0 => Question {
                    content: "test".to_string(),
                    answer: "hoge".to_string(),
                },
                1 => Question {
                    content: "test2".to_string(),
                    answer: "hoge2".to_string(),
                },

            },
        }
    }

    pub fn iter_mut(&mut self) -> QuizIterator<'_> {
        QuizIterator {
            quiz: self,
            cursor: 0,
            summary: indexset! {},
        }
    }
}

pub struct QuizIterator<'a> {
    quiz: &'a Quiz,
    cursor: usize,
    summary: IndexSet<usize>,
}

impl QuizIterator<'_> {
    pub fn check(&mut self, answer: &str) -> &'static str {
        let current_question = &self.quiz.questions[self.cursor];
        let target_answer = &current_question.answer;
        if target_answer == answer {
            self.summary.insert(self.cursor);
            "正解です。"
        } else {
            "不正解です。"
        }
    }

    pub fn summary_result(&self) -> Summary {
        (self.quiz.questions.len(), self.summary.len())
    }
}

impl Iterator for QuizIterator<'_> {
    type Item = Question;

    fn next(&mut self) -> Option<Self::Item> {
        let old_cursor = self.cursor;
        self.cursor += 1;
        (self.quiz.questions.len() > old_cursor)
            .then(|| self.quiz.questions[old_cursor].clone())
    }
}
