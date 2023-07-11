// api/mod.rs
use actix_web::web;
pub mod item_api;

pub fn config(conf: &mut web::ServiceConfig) {
  let scope = web::scope("/api")
      .service(item_api::create_item)
      .service(item_api::get_item)
      .service(item_api::get_items)
      .service(item_api::update_item)
      .service(item_api::delete_item);

  conf.service(scope);
}