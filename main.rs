mod info;
mod logs;
mod spawner;
mod policy;
use std::{
    env,
//    os
};


fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not Enought Argument !!! \n arg = {:?}", args);
    }
    match args[1].as_ref() {
        "test" => println!("matching Test concluant"),
        "get_pod" => info::pods()?,
        "get_namespaces" => info::namespace()?,
        "get_logs" => logs::get_logs(args[2].clone())?,
        "spawn_namespace" => spawner::spawn_namespace(args[2].clone())?,
        "spawn_pod" => spawner::spawn_pod(args[2].clone())?,
        "deploy" => spawner::deployment(args[2].clone(), args[3].clone())?,
        //"describe" => describe_node::describe()?,
        _ => println!("None Matched."),
    };
    Ok(())
}
