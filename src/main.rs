use crate::common::app_state::{AppState, Pool};
use crate::common::env::Config;
use crate::routes::users::{user_login, user_logout, user_register};
use crate::stubs::presence::presence_client::PresenceClient;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use std::sync::Arc;
use tokio::sync::Mutex;


mod auth;
mod common;
mod models;
mod routes;
mod schema;
mod stubs;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment variables, init log, init config
    dotenv::dotenv().ok();
    env_logger::init();
    let config = Config::from_env();

    // generate JWT auth keys
    let auth_keys = auth::keys::generate_auth_keys().await;

    // create postgresql connection pool
    let manager = ConnectionManager::<PgConnection>::new(&config.database_uri);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool.");
    let presence_client = Arc::new(Mutex::new(
        PresenceClient::connect("http://127.0.0.1:8001")
            .await
            .unwrap(),
    ));

    // initialize http server
    println!(
        "HTTP Server starting at http://{}:{}",
        &config.host_ip, &config.host_port
    );
    let result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                config: config.clone(),
                keys: auth_keys.clone(),
                db: pool.clone(),
                presence_client: Arc::clone(&presence_client),
            }))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v1")
                    .service(web::resource("/register").route(web::post().to(user_register)))
                    .service(web::resource("/login").route(web::get().to(user_login)))
                    .service(web::resource("/logout").route(web::get().to(user_logout))),
            )
    })
    .bind((Config::from_env().host_ip, Config::from_env().host_port))?
    .run()
    .await;

    result
}
