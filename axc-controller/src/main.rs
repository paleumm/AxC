use k8s_openapi::api::core::v1::Pod;
// use serde_json::json;
// use tracing::*;

// use kube::{
//     api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
//     runtime::wait::{await_condition, conditions::is_pod_running},
//     Client,
// };

use kube::{Api, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    // axc_controller::create_runtime(&pods).await.unwrap();
    axc_controller::delete_pod("blog", pods).await;

    Ok(())
}
