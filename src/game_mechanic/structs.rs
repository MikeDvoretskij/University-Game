use std::collections::HashMap;

#[derive(Debug)]
struct Question_One {
    pub question: String,
    pub answer: String,
    pub answers: HashMap<i8, String>
}

struct Player {
    user_name: String,
    score: f32,
}

impl Player {
    fn new(user_name:String, score:f32) -> Player{
        Player{
            user_name: user_name,
            score: score
        }
    }
}