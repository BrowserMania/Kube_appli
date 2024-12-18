//use kube::{Client, Api};
use kube::{
    api::{Api, PostParams},
    Client,
};
use k8s_openapi::api::apps::v1::{
    Deployment,
    DeploymentSpec
};
use k8s_openapi::api::core::v1::{
    Container,
    Namespace,
    Pod,
    PodSpec,
    PodTemplateSpec
};
use anyhow;

//gestion des username fait par badr côté bdd
#[tokio::main]
pub async fn spawn_namespace(mut user: String) ->  anyhow::Result<()> {
    user = format!("{user}-browser");
    let client = Client::try_default().await?;
    println!("Votre user est {}", user);

    let namespace = Namespace {
        metadata: kube::api::ObjectMeta {
            name: Some(user.to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let namespaces: Api<Namespace> = Api::all(client);
    
    let result = namespaces.create(&PostParams::default(), &namespace).await?;
    println!("Nouveau namespace : {}", result.metadata.name.unwrap());
    Ok(())
    /*Ok(result.metadata.name.unwrap())
    */
}

#[tokio::main]
pub async fn spawn_pod(mut user: String) ->  anyhow::Result<()> {
    user = format!("{user}-browser");
    let client = Client::try_default().await?;
    println!("Votre user est {}", user);

    let pod = Pod {
        metadata: kube::api::ObjectMeta {
            name : Some(user.to_string()),
            ..Default::default()
        },
        spec: Some(PodSpec {
            containers: vec![Container {
                name: user.to_string(),
                image: Some("runner".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let pods: Api<Pod> = Api::default_namespaced(client);
    let result = pods.create(&PostParams::default(), &pod).await?;
    println!("Nouveau pod crée, {}", result.metadata.name.unwrap());
    Ok(())   
}


#[tokio::main]
pub async fn deployment(mut user: String, mut nm: String) ->  anyhow::Result<()> {
    user = format!("{user}-browser");
    nm = format!("{nm}-browser");
    let client = Client::try_default().await?;
    println!("Votre user est {}", user);

    let deployment = Deployment {
        metadata: kube::api::ObjectMeta {
            name: Some(user.to_string()),
            namespace: Some(nm.to_string()),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            template: PodTemplateSpec {
                
            } 
            ,
            ..Default::default()
        }),
        ..Default::default()
    };

    let deployments: Api<Deployment> = Api::all(client);
    let result = deployments.create(&PostParams::default(), &deployment).await?;

    println!("le résultat est m {}", result.metadata.name.unwrap());
    Ok(())    
}