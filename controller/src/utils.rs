use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
// use std::io::prelude::*;
use std::io::Write;

/// Read yaml config from 'pod_config.yaml' to create pod
pub fn read_pod_config() -> serde_yaml::Value {
    let f = std::fs::File::open("config/pv/kube/pod_config.yaml").unwrap();
    let config: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();
    config
}

/// Read yaml config from 'pvc_config.yaml' to create pvc
pub fn read_pvc_config() -> serde_yaml::Value {
    let f = std::fs::File::open("config/pv/kube/pvc_config.yaml").unwrap();
    let config: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();
    config
}

pub fn write_pvc(path: &String, info: &JGraph) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(serde_json::to_string(&info).unwrap().as_bytes())
        .unwrap();
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JGraph {
    main: String,
    model: Vec<String>,
    datamodule: Vec<String>,
    dataset: Vec<String>,
}

// pub trait TokenCheck {
//     pub async fn check_token<T>(&self) -> Option<&T>;
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct PileResult {
    pub result: JGraph,
    pub access_token: Option<String>,
}

impl PileResult {
    pub async fn check_token(&self) -> Option<&PileResult> {
        if self.access_token == Some("abc".to_string()) {
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: Option<String>,
}

impl Token {
    pub async fn check_token(&self) -> Option<&Token> {
        if self.access_token == Some("abc".to_string()) {
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Deserialize)]
pub struct PileParams {
    pub id: u64,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PodConfig {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    pub containers: Container,
    pub restartPolicy: String,
    pub volumes: Volumes,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub command: Vec<String>,
    pub volumeMounts: VolumeMounts,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeMounts {
    pub name: String,
    pub mountPath: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Volumes {
    pub name: String,
    pub persistentVolumeClaim: PersistentVolumeClaim,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PersistentVolumeClaim {
    pub claimName: String,
}
