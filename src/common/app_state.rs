use crate::auth::keys::AuthKeys;
use crate::common::env::Config;
use crate::utils::publish;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type AMQPClient = publish::SharedAMPQConnection;

pub struct AppState {
    pub config: Config,
    pub keys: AuthKeys,
    pub db: Pool,
    pub amqp_client: AMQPClient,
}
