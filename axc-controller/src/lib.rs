use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
    runtime::wait::{await_condition, conditions::is_pod_running},
    Client,
};
use serde_json::json;
use tracing::*;
//pods: &Api<Pod>
pub async fn create_runtime() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    // Create Pod blog
    info!("Creating Pod instance blog");
    let p: Pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": "blog" },
        "spec": {
            "containers": [{
              "name": "blog",
              "image": "clux/blog:0.1.0"
            }],
        }
    }))?;

    let pp = PostParams::default();
    match pods.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            info!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => return Err(e.into()),                        // any other case is probably bad
    }

    Ok(())
}

async fn create_pod(pod: Pod, pods: Api<Pod>) -> anyhow::Result<()> {
    let pp = PostParams::default();
    match pods.create(&pp, &pod).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(pod.name_any(), name);
            info!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => return Err(e.into()),                        // any other case is probably bad
    }
    Ok(())
}
//podname: &str, pods: Api<Pod>
pub async fn delete_pod() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    let podname = "blog";
    let dp = DeleteParams::default();
    pods.delete(podname, &dp).await.unwrap().map_left(|pdel| {
        assert_eq!(pdel.name_any(), podname);
        info!("Deleting blog pod started: {:?}", pdel);
    });
    Ok(())
}
