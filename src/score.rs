use crate::frame::{Drawable, Frame};

pub struct Score {
    x: usize,
    y: usize,
    score: u32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            score: 0,
        }
    }

    pub fn score(&mut self) {
        self.score += 1;
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        let score = self.score.to_string();
        let formated_score = format!("Score: {}", score);
        frame[self.x][self.y] = formated_score;
    }
}
