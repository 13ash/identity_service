use crate::common::env::Environment;
use crate::stubs::auth::auth_client::AuthClient;
use crate::stubs::presence::presence_client::PresenceClient;
use crate::token::keys::AuthKeys;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub env: Environment,
    pub keys: AuthKeys,
    pub db: Pool,
    pub presence_client: Arc<Mutex<PresenceClient<Channel>>>,
    pub auth_client: Arc<Mutex<AuthClient<Channel>>>,
}
