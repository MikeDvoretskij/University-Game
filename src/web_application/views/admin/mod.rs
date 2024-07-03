pub mod question_form;
mod structs;

use actix_web::web::{ServiceConfig, get, post, scope};

pub fn admin_views_factory(app: &mut ServiceConfig){
    app.route("/", get().to(question_form::clear_question))
        .route("/", post().to(question_form::form_hanler));
}