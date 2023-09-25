use crate::common::app_state::{AppState, Pool};
use crate::common::env::Environment;
use crate::routes::users::{user_login, user_logout, user_register};
use crate::stubs::presence::presence_client::PresenceClient;
use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::Pg;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

mod common;
mod models;
mod routes;
mod schema;
mod stubs;
mod token;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment variables, init log, init config
    dotenv::dotenv().ok();
    env_logger::init();
    let config = Environment::from_env();

    // generate JWT auth keys
    let token_keys = token::keys::generate_token_keys().await;

    // create postgresql connection pool
    let manager = ConnectionManager::<PgConnection>::new(&config.database_uri);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool.");

    let mut connection = pool
        .get()
        .expect("Failed to get a connection from the pool");

    run_migrations(&mut connection).unwrap();

    let presence_client = Arc::new(Mutex::new(
        PresenceClient::connect("http://presence_service:8001")
            .await
            .unwrap(),
    ));

    let auth_client = Arc::new(Mutex::new(AuthClient::connect("http://auth_service:8003")
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
                env: config.clone(),
                keys: token_keys.clone(),
                db: pool.clone(),
                presence_client: Arc::clone(&presence_client),
                auth_client: Arc::clone(&auth_client)
            }))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v1")
                    .service(web::resource("/register").route(web::post().to(user_register)))
                    .service(web::resource("/login").route(web::get().to(user_login)))
                    .service(web::resource("/logout").route(web::get().to(user_logout))),
            )
    })
    .bind((
        Environment::from_env().host_ip,
        Environment::from_env().host_port,
    ))?
    .run()
    .await;

    result
}
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::stubs::auth::auth_client::AuthClient;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
