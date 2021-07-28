use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use futures_channel::mpsc::{UnboundedSender};
use futures_util::{future, pin_mut, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{trace, warn};
use url::Url;

use crate::types::entity::Entity;
use crate::types::events::{RateData, VoteData};
use crate::types::payload::{GatewayOp, Payload};

const GATEWAY_URL: &str = "wss://gateway.dlist.top";


#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_ready(&self, entity: &Entity) {}
    async fn on_disconnect(&self, reason: &str) {}
    async fn on_vote(&self, data: VoteData) {}
    async fn on_rate(&self, data: RateData) {}
}

error_chain! {
    /*types {
        MyError, MyErrorKind, MyResult;
    }*/
    foreign_links {
        Websocket(tungstenite::Error) #[doc = "Websocket error"];
        TrySendError(futures_channel::mpsc::TrySendError<Message>) #[doc = "Sending error"];
        Serde(serde_json::Error) #[doc = "JSON error"];
    }
    errors {
        CannotParsePayload(got: String, error: serde_json::Error) {
            description("Cannot parse payload JSON")
            display("Cannot parse payload. {:?} Got: {}", error, got)
        }
    }
}

pub struct Client<'a> {
    token: &'a str,
    handler: Arc<dyn EventHandler>,
    write_tx: Option<Arc<UnboundedSender<Message>>>,
    pub entity: Option<Entity>,
}

impl<'a> Client<'a> {
    pub fn new(token: &'a str, handler: impl EventHandler + 'static) -> Self {
        Client {
            token,
            handler: Arc::new(handler),
            entity: None,
            write_tx: None,
        }
    }

    pub async fn connect(&mut self) {
        let url = Url::parse(GATEWAY_URL).unwrap();

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();

        let (write_tx, write_rx) = futures_channel::mpsc::unbounded::<Message>();

        let sender = StreamExt::map(write_rx, Ok).forward(write);

        self.write_tx = Some(Arc::new(write_tx));

        let self_lock = RwLock::new(self);

        let reader = {
            read.for_each(|message| async {
                let mut that = self_lock.write().unwrap();
                if let Err(why) = that.on_message(message).await {
                    warn!("{}", why);
                }
            })
        };

        pin_mut!(sender, reader);
        future::select(sender, reader).await;
    }

    fn send(&self, msg: Message) -> Result<()> {
        self.write_tx.as_ref().unwrap().unbounded_send(msg)?;
        Ok(())
    }

    async fn on_message(&mut self, message: tungstenite::Result<Message>) -> Result<()> {
        let text = message?.into_text().unwrap();

        if text == "Connection closed" { return Ok(()); }

        let payload = serde_json::from_str::<Payload>(&text).map_err(|e| ErrorKind::CannotParsePayload(text, e))?;

        match payload.op {
            GatewayOp::Ready => {
                let entity: Entity = serde_json::from_value(payload.data)?;
                trace!("Ready. Connected to {:?}", entity);
                self.entity = Some(entity);
                self.handler.on_ready(&self.entity.as_ref().unwrap()).await;
            }
            GatewayOp::Hello => {
                let msg = payload.data.as_str().unwrap_or("");
                trace!("Connected with message: {}", msg);
                let data = serde_json::to_string(&Payload {
                    op: GatewayOp::Identify,
                    data: json!({
                                "token": self.token
                            }),
                    event: "".to_string(),
                }).unwrap();

                self.send(Message::Text(data))?;
                trace!("Identify packet sent");
            }
            GatewayOp::Event => {
                match payload.event.to_lowercase().as_str() {
                    "vote" => {
                        let data = serde_json::from_value(payload.data)?;
                        self.handler.on_vote(data).await;
                    }
                    "rate" => {
                        let data = serde_json::from_value(payload.data)?;
                        self.handler.on_rate(data).await;
                    }
                    _ => {}
                }
            }
            GatewayOp::Disconnect => {
                let why = payload.data.as_str().unwrap_or("");
                self.handler.on_disconnect(why).await;
            }
            _ => {}
        };

        Ok(())
    }
}