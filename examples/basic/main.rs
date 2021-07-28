use dlist_top::{Client, EventHandler};
use dlist_top::events::{VoteData, RateData};
use dlist_top::entity::Entity;
use async_trait::async_trait;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;

mod config;

struct DlistEvents;

#[async_trait]
impl EventHandler for DlistEvents {
    async fn on_ready(&self, _entity: &Entity) {
        println!("DList.top ready!");
    }
    async fn on_disconnect(&self, reason: &str) {
        println!("DList.top disconnected with reason: {}", reason)
    }
    async fn on_vote(&self, data: VoteData) {
        println!("vote! {:?}", data);
    }
    async fn on_rate(&self, data: RateData) {
        println!("rate! {:?}", data);
    }
}


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_env_filter("dlist_top=trace")
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let mut client = Client::new(config::TOKEN, DlistEvents);
    client.connect().await;

    println!("test");
}

