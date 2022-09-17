use kube::Client;
use serde_json::{Result, Value};

#[tokio::main]
async fn main() {
    let client = Client::try_default().await.unwrap();
}
