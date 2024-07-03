use actix_web::web::Form;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QuestionForm{
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub answers: String
}