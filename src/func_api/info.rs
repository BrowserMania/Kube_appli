use anyhow;
use k8s_openapi::api::core::v1::{Namespace, Pod};
use k8s_openapi::api::networking::v1::NetworkPolicy;
use kube::api::ListParams;
use kube::{Api, Client};
//use std::error::Error;

pub async fn pods() -> Result<Vec<String>, anyhow::Error> {
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

    //let list_params = ListParams::default().labels("browser");
    let list_params = ListParams::default();

    let pod_list = pods.list(&list_params).await?;
    let mut pod_list_json = Vec::new();

    for pod in pod_list {
        if let Some(name) = pod.metadata.name {
            println!("Found Pod: {}", name);
            pod_list_json.push(name);
        }
    }
    Ok(pod_list_json)
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


/*
 Pour le moment on work avec des network policy, pas des cilium le CRD plante, 
 */

pub async fn networking_rule() -> Result<Vec<String>, anyhow::Error> {
    let client = Client::try_default().await?;
    let policy: Api<NetworkPolicy> = Api::all(client);

    let policy_list = policy.list(&ListParams::default()).await?;
    let mut vec_policy = Vec::new();
    
    for element in policy_list {
        if let Some(name) = element.metadata.name {
            println!("Policy name: {}", name);
            vec_policy.push(name);
        }
    }
    Ok(vec_policy)
}