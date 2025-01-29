use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct User {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub enum Auto {
    Egress,
    Ingress,
    DenyIngress,
    DenyEgress,
}

#[derive(Deserialize, Debug)]
pub struct Policie {
    pub name: String,
    pub description: String,
    pub direction: Auto,           // egress ou ingress
    pub ports: Option<Vec<u16>>,   //plage ou liste de ports
    pub pods: Option<Vec<String>>, //pods où on applique le filtre
    pub host: Option<Vec<String>>, //host filtre l7
}

#[derive(Deserialize, Debug)]
pub struct ListRules {
    pub rules: Vec<Policie>,
}
