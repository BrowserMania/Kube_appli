mod func_api {
    pub mod delete;
    pub mod info;
    pub mod object;
    pub mod policy;
    pub mod spawner;
}

use axum::{
    extract,
    http::StatusCode,
    response::IntoResponse,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{
    Deserialize,
    //    Serialize,
};
use serde_json::json;

//async fn get_session() -> &'static str{
//    "Get_session de toto"
//}
//
//async fn list_user() {
//    todo!()
//}

#[derive(Deserialize, Debug)]
struct NewUser {
    name: String,
    password: String,
    mail: String,
    // autres paramètres ?
}

async fn create_user(cred: extract::Json<NewUser>) -> impl IntoResponse {
    let my_cred = &cred.0;
    let user = &cred.name;
    let passwd = &cred.password;
    let mail = &cred.mail;
    println!(
        "info : {:?}\ntu est {user} \nTon mdp vaut : {passwd} \nton mais est {mail}",
        my_cred
    );
    (StatusCode::OK, "Correct connection en cours");
}

#[derive(Deserialize, Debug)]
struct ListUser {
    users: Vec<NewUser>,
}

async fn penis(cred: extract::Json<ListUser>) -> impl IntoResponse {
    for element in &cred.users {
        println!(
            "info : {:?}\ntu est {} \nTon mdp vaut : {} \nton mais est {}",
            cred, element.name, element.password, element.mail
        )
    }
    (StatusCode::OK, "Correct connection en cours");
}

async fn create_session(cred: extract::Json<func_api::object::User>) -> impl IntoResponse {
    let user = &cred.id;
    let _ = func_api::spawner::spawn_namespace(user.to_string()).await;
    let _ = func_api::spawner::spawn_pod(user.to_string()).await;
    (StatusCode::OK, "Correct connection en cours");
}

async fn delete_session(cred: extract::Json<func_api::object::User>) -> impl IntoResponse {
    let user = &cred.id;
    let _ = func_api::delete::del_namespace(user.to_string()).await;
    let _ = func_api::delete::del_pod(user.to_string()).await;
    (StatusCode::OK, "Correct connection en cours");
}

async fn list_pods() -> impl IntoResponse {
    //let _ = func_api::info::pods().await;
    //(StatusCode::OK, "Correct connection en cours");
    match func_api::info::pods().await {
        Ok(pods) => {
            let response = json!({ "pods": pods });
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

async fn list_policy() -> impl IntoResponse {
    //let _ = func_api::info::pods().await;
    //(StatusCode::OK, "Correct connection en cours");
    match func_api::info::networking_rule().await {
        Ok(policy) => {
            let response = json!({ "pods": policy });
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

#[tokio::main]
async fn main() { 
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users/list", post(penis))
        .route("/session/new", post(create_session))
        .route("/session/del", post(delete_session))
        .route("/user/new", post(create_user))
        .route("/pods/list", get(list_pods))
        .route("/policy/list", get(list_policy))
        .route("/session/list", {
            list_pods().await;
            get(())
        });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
