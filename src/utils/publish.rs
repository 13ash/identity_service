use crate::common::app_state::AMQPClient;
use crate::models::status::StatusModel;
use actix_web::web::Json;
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions};
use lapin::publisher_confirm::Confirmation;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Connection, ExchangeKind};
use std::sync::Arc;

pub struct SharedAMPQConnection {
    inner: Arc<Connection>,
}

impl SharedAMPQConnection {
    pub fn new(conn: Connection) -> Self {
        Self {
            inner: Arc::new(conn),
        }
    }

    pub fn get(&self) -> Arc<Connection> {
        self.inner.clone()
    }
}

impl Clone for SharedAMPQConnection {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

async fn publish_status(
    amqp_client: &AMQPClient,
    routing_key: &str,
    body: Json<StatusModel>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the channel
    let channel = amqp_client.get().create_channel().await?;

    // Declare an exchange
    let exchange = "presence";
    channel
        .exchange_declare(
            exchange,
            ExchangeKind::Direct,
            ExchangeDeclareOptions {
                passive: false,
                durable: true,
                auto_delete: false,
                internal: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await?;

    // Prepare the message payload
    let payload = serde_json::to_string(&*body)?;

    // Publish the message
    let confirm = channel
        .basic_publish(
            exchange,
            routing_key,
            BasicPublishOptions::default(),
            &*payload.as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .await?;

    match confirm.await? {
        Confirmation::NotRequested => {
            println!("Message published without confirmation");
        }
        Confirmation::Nack(info) => {
            println!("Message was not acknowledged: {:?}", info);
        }
        Confirmation::Ack(info) => {
            println!("Message was acknowledged: {:?}", info);
        }
    }

    Ok(())
}

pub async fn publish_login(
    amqp_client: &AMQPClient,
    body: Json<StatusModel>,
) -> Result<(), Box<dyn std::error::Error>> {
    publish_status(amqp_client, "login", body).await
}

pub async fn publish_logout(
    amqp_client: &AMQPClient,
    body: Json<StatusModel>,
) -> Result<(), Box<dyn std::error::Error>> {
    publish_status(amqp_client, "logout", body).await
}
