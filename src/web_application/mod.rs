pub mod views;

use actix_web::web::{ServiceConfig};

pub fn views_factory(app: &mut ServiceConfig) {
    views::admin::admin_views_factory(app);
}