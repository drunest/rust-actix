use actix_web::{guard, middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;

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

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(api::config)
            .service(
                web::resource("/api").route(
                    web::route()
                        .guard(guard::Get())
                        .guard(guard::Header("content-type", "application/json"))
                        .to(|| HttpResponse::Ok()),
                ),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
