use k8s_openapi::api::core::v1::PersistentVolumeClaim;
use kube::{
    api::{Api, DeleteParams, PostParams, ResourceExt},
    Client,
};
use serde_yaml;
use tracing::*;

/// create pvc from yaml config
pub async fn create_pvc(config: serde_yaml::Value, name: String) -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    // Manage pvc
    let pvc: Api<PersistentVolumeClaim> = Api::namespaced(client, "pile");

    // Create pvc
    info!("Creating pvc instance");
    let mut p: PersistentVolumeClaim = serde_yaml::from_value(config)?;

    p.metadata.name = Some(name);

    let pp = PostParams::default();
    match pvc.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            info!("Created pvc: {}", name);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

/// delete pvc by its name
pub async fn delete_pvc(pvc_name: &String) -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    // Manage pods
    let pvc: Api<PersistentVolumeClaim> = Api::namespaced(client, "pile");

    let dp = DeleteParams::default();
    pvc.delete(&pvc_name[..], &dp)
        .await
        .unwrap()
        .map_left(|pdel| {
            assert_eq!(pdel.name_any(), &pvc_name[..]);
            info!("Deleting pvc : {:?}", pdel.name_any());
        });
    info!("PVC {} Deleted", pvc_name);
    Ok(())
}
