#![windows_subsystem = "windows"]

mod status;

use tracing::{error, info};
use tracing_subscriber;

use crate::status::Methods;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let address = String::from("1.1.1.1:80");
    let mut address_list = Vec::new();
    let mut methods = Vec::new();
    address_list.push(address);
    methods.push(Methods::Tcp);

   info!("{:?}",  check_servers(&address_list, &methods).await)

}

async fn check_servers(servers: &Vec<String>, methods: &Vec<status::Methods>) -> Vec<bool>{

    let mut status_list:Vec<bool> = Vec::new();
    let mut i = 0;

    for server in servers.iter(){
        let method = methods.get(i).unwrap();
        let server_status = status::check_status(method, server).await;

        match server_status{
            Ok(status) => status_list.push(status),
            // If function returns error that means we cannot reach cf DNS which means no internet
            Err(_) => return Vec::new(),
        }

        i = i +1;
    }

    return status_list;

}