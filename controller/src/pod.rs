use futures::{StreamExt, TryStreamExt};
// use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{
        Api, AttachParams, /*AttachedProcess,*/ DeleteParams, ListParams, PostParams,
        ResourceExt, WatchEvent,
    },
    Client,
};
use serde_yaml;
use tracing::*;

// use crate::utils::PodConfig;

/// create pod from yaml config
pub async fn create_pod(
    config: serde_yaml::Value,
    name: String,
    pvc_name: String,
) -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    // Manage pods
    let pods: Api<Pod> = Api::namespaced(client, "pile");

    // Create Pod blog
    info!("Creating Runtime instance");
    let mut p: Pod = serde_yaml::from_value(config)?;

    p.metadata.name = Some(name);
    p.spec.volumes.persistent_volume_claim.claim_name = Some(pvc_name);

    let pp = PostParams::default();
    match pods.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            info!("Created pod: {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => return Err(e.into()),                        // any other case is probably bad
    }

    Ok(())
}

/// delete pod by its name
pub async fn delete_pod(podname: &String) -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    // Manage pods
    let pods: Api<Pod> = Api::namespaced(client, "pile");

    let dp = DeleteParams::default();
    pods.delete(&podname[..], &dp)
        .await
        .unwrap()
        .map_left(|pdel| {
            assert_eq!(pdel.name_any(), &podname[..]);
            info!("Deleting pod : {:?}", pdel.name_any());
        });
    info!("Pod {} Deleted", podname);
    Ok(())
}

/// exec pod by its name
pub async fn exec_pod(podname: &String) -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    // Manage pods
    let pods: Api<Pod> = Api::namespaced(client, "pile");

    pods.exec(
        &podname,
        vec!["bash", "-c", "/data/generator/runtime.sh"],
        &AttachParams::default().stderr(false),
    )
    .await?;
    info!("Pod {} Executed", podname);
    Ok(())
}

pub async fn _runtime(config: serde_yaml::Value) -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    // Manage pods
    let pods: Api<Pod> = Api::namespaced(client, "pile");

    // Create Pod blog
    info!("Creating Runtime instance");
    let p: Pod = serde_yaml::from_value(config)?;

    let pp = PostParams::default();
    match pods.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            info!("Created pod: {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => return Err(e.into()),                        // any other case is probably bad
    }

    let lp = ListParams::default()
        .fields("metadata.name=test-user")
        .timeout(10);
    let mut stream = pods.watch(&lp, "0").await?.boxed();
    while let Some(status) = stream.try_next().await? {
        match status {
            WatchEvent::Added(o) => {
                info!("Added {}", o.name_any());
            }
            WatchEvent::Modified(o) => {
                let s = o.status.as_ref().expect("status exists on pod");
                if s.phase.clone().unwrap_or_default() == "Running" {
                    info!("Ready to attach to {}", o.name_any());
                    break;
                }
            }
            _ => {}
        }
    }

    let podname = "test-user".to_string();
    pods.exec(
        &podname[..],
        vec!["bash", "-c", "/data/generator/runtime.sh"],
        &AttachParams::default().stderr(false),
    )
    .await?;
    info!("Pod {} Executed", podname);

    Ok(())
}
