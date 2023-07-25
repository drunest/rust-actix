use actix_web::{guard, web, App, HttpServer, HttpResponse};

mod api;
mod error;
mod model;
mod prelude;
mod repository;
mod utils;

use repository::surrealdb_repo::SurrealDBRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = match SurrealDBRepo::init().await {
        Ok(surreal) => {
            println!("✅ Connection to the database is successful!");
            surreal
        }
        Err(e) => {
            println!("🔥 Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        }
    };

    let db_data = web::Data::new(surreal);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(api::config)
            .service(
              web::resource("/api").route(
                web::route()
                  .guard(guard::Get())
                  .guard(guard::Header("content-type", "application/json"))
                  .to(|| HttpResponse::Ok())
              )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
