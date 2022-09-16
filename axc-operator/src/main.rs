use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client, Resource};
use log::info;
use log4rs;

#[tokio::main]
async fn main() {
    let client = Client::try_default().await.unwrap();

    let api: Api<Pod> = Api::namespaced(client, "ray-cluster");
    api.list(&ListParams::default())
        .await
        .unwrap()
        .items
        .iter()
        .map(|pod| pod.name())
        .for_each(|name| info!("{}", name));
}
