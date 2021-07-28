# DList.top Rust client
Official [dlist.top](https://dlist.top) gateway client for Rust.


## Installation

```toml
[dependencies]
dlist-top = "*"
```

## Setup

To get your token please refer to the [DList.top documentation](https://github.com/dlist-top/docs/wiki/Getting-started).


## Usage

```rust
use dlist_top::{Client, EventHandler};
use dlist_top::events::{VoteData, RateData};
use dlist_top::entity::Entity;
use async_trait::async_trait;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;

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
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let mut client = Client::new("TOKEN", DlistEvents);
    client.connect().await;
}

```