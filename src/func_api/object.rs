use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct User {
    pub id: String,
}

//enum dir {
//    Egress,
//    Ingress,
//    DenyIngress,
//    DenyEgress,
//}

//#[derive(Deserialize, Debug)]
//struct Policie{
//    name: String,
//    direction: String,       // egress ou ingress
//    ports: Some<Vec<int>>,   //plage ou liste de ports
//    pods: Some<Vec<String>>, //pods où on applique le filtre
//    host: Some<Vec<String>>  //host filtre l7
//}

//#[derive(Deserialize, Debug)]
//struct ListRules{
//    rules: Vec<Policie>
//}
