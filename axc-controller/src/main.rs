// use k8s_openapi::api::core::v1::Pod;
// use serde_json::json;
// use tracing::*;

// use kube::{
//     api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
//     runtime::wait::{await_condition, conditions::is_pod_running},
//     Client,
// };

use kube::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let client = Client::try_default().await?;

    axc_controller::create_runtime(client).await.unwrap();

    Ok(())
}
