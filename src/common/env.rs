use std::env;

use tonic::transport::Endpoint;

#[derive(Debug, Clone)]
pub struct Environment {
    pub host_ip: String,
    pub host_port: u16,
    pub database_uri: String,
    pub rust_log: String,
    pub local_salt: String,
    pub presence_service_endpoint: Endpoint,
}

impl Environment {
    pub fn from_env() -> Environment {
        Environment {
            host_ip: env::var("HOST_IP").expect("HOST_IP variable not found."),
            host_port: env::var("HOST_PORT")
                .expect("HOST_PORT variable not found.")
                .parse()
                .expect("HOST_PORT variable malformed."),
            database_uri: env::var("DATABASE_URI").expect("DATABASE_URI variable not found."),
            rust_log: env::var("RUST_LOG").expect("RUST_LOG variable not found."),
            local_salt: env::var("LOCAL_SALT").expect("LOCAL_SALT variable not found."),
            presence_service_endpoint: Endpoint::from_static("http://presence_service:8001"),
        }
    }
}
