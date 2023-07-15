// api/mod.rs
use actix_web::web;
pub mod item_api;
pub mod user_api;

pub fn config(conf: &mut web::ServiceConfig) {
  let scope = web::scope("/api")
      .service(item_api::create_item)
      .service(item_api::get_item)
      .service(item_api::get_items)
      .service(item_api::update_item)
      .service(item_api::delete_item)
      .service(item_api::search_items_by_ids)
      .service(user_api::create_user)
      .service(user_api::get_user)
      .service(user_api::get_users)
      .service(user_api::update_user)
      .service(user_api::delete_user)
      .service(user_api::search_users_by_ids);

  conf.service(scope);
}