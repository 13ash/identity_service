use crate::auth::keys::AuthKeys;
use crate::common::env::Config;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub config: Config,
    pub keys: AuthKeys,
    pub db: Pool,
}
