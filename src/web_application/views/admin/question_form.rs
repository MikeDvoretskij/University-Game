use super::structs::QuestionForm;
use actix_web::{HttpResponse, web::Form, Responder, web::Data};
use sqlx::{Pool, Sqlite};
use crate::content_loader::read_file;
use std::path::PathBuf;
use std::env;


pub async fn clear_question() -> impl Responder{
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let page_path: PathBuf = [current_dir.to_str().unwrap(), "templates", "html", "admin_question.html"].iter().collect();
    let page_path = page_path.to_str().unwrap();
    let page = read_file(page_path);
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(page)
}

pub async fn form_hanler(form: Form<QuestionForm>, db_pool: Data<Pool<Sqlite>>) -> impl Responder{
    let current_dir = env::current_dir().expect("Failed to get current directory");
    {let query_path: PathBuf = [current_dir.to_str().unwrap(), "database", "requests", "insert", "insert_questions.sql"].iter().collect();
    let query_path = query_path.to_str().unwrap();

    let id = form.id.to_string();
    let question =  form.question.as_str();
    let answer =  form.answer.to_string();
    let answers = form.answers.to_string();

    let query = read_file(query_path)
        .replace("id", &id)
        .replace("'question'", &format!("'{}'", &question).to_string())
        .replace("answers", &answers)
        .replace("answer", &answer);

    sqlx::query(&query)
        .execute(&*db_pool.get_ref())
        .await
        .expect("Error with query");}

    let page_path: PathBuf = [current_dir.to_str().unwrap(), "templates", "html", "admin_question.html"].iter().collect();
    let page_path = page_path.to_str().unwrap();
    let page = read_file(page_path);
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(page)

}