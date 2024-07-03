mod database;
mod web_application;
mod content_loader;

use actix_web::{web, App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let pool = database::create_pool().await;
    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(web_application::views_factory);
        return app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

