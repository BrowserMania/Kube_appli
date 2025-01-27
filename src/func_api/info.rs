use k8s_openapi::api::core::v1::{Namespace, Pod};
use kube::api::ListParams;
use kube::{Api, Client};
use anyhow;
//use std::error::Error;

pub async fn pods() -> anyhow::Result<()> {
//    let config = kube::Config::from_kubeconfig(&kube::config::KubeConfigOptions::default()).await?;
//    let client = Client::try_from(config)?;
//
//    // Work with Kubernetes API.
//    let pods: Api<Pod> = Api::default_namespaced(client);
//    let lp = ListParams::default();
//
//    for p in pods.list(&lp).await? {
//        println!("Found Pod: {}", p.metadata.name.unwrap_or_default());
//    }
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::all(client);
    
    let list_params = ListParams::default().labels("browser");
    
    let pod_list = pods.list(&list_params).await?;
    
    for pod in pod_list {
        if let Some(name) = pod.metadata.name {
            println!("Found Pod: {}", name);
        }
    }
    Ok(())
}

#[tokio::main]
pub async fn namespace() -> anyhow::Result<()> {
    let config = kube::Config::from_kubeconfig(&kube::config::KubeConfigOptions::default()).await?;
    let client = Client::try_from(config)?;

    let namespaces: Api<Namespace> = Api::all(client);
    match namespaces.list(&Default::default()).await {
        Ok(list) => {
            println!("Namespaces dans le cluster:");
            for ns in list.items {
                if let Some(name) = ns.metadata.name {
                    println!("- {}", name);
                }
            }
        }
        Err(_e) => todo!(),
    }
    Ok(())
}
