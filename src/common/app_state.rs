use crate::auth::keys::AuthKeys;
use crate::common::env::Config;
use crate::stubs::presence::presence_client::PresenceClient;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub config: Config,
    pub keys: AuthKeys,
    pub db: Pool,
    pub presence_client: Arc<Mutex<PresenceClient<Channel>>>,
}
