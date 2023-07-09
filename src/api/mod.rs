// api/mod.rs
use actix_web::web;
pub mod todo_api;

pub fn config(conf: &mut web::ServiceConfig) {
  let scope = web::scope("/api")
      .service(todo_api::create_todo)
      .service(todo_api::get_todo)
      .service(todo_api::get_todos)
      .service(todo_api::update_todo)
      .service(todo_api::delete_todo);

  conf.service(scope);
}