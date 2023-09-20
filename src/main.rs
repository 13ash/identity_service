use crate::common::app_state::{AppState, Pool};
use crate::common::env::Config;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

mod common;
mod models;
mod schema;
mod routes;
mod utils;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let auth_keys = auth::keys::generate_auth_keys().await;
    let config = Config::from_env();
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool.");

    println!("Server starting at http://{}:{}", &config.host_ip, &config.host_port);
    let result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                config: config.clone(),
                keys: auth_keys.clone(),
                db: pool.clone(),
            }))
            .wrap(middleware::Logger::default())
            .service(web::scope("/v1").service(web::resource("/auth")))
    })
    .bind((Config::from_env().host_ip, Config::from_env().host_port))?
    .run()
    .await;

    result
}
