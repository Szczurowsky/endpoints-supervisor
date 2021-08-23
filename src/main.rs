mod json_handler;

use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    json_handler::init();

    // Load field realtime from config
    let load_realtime = json_handler::get_config().get_load_realtime();

    // Preload all endpoints from file
    let mut loaded_servers = json_handler::get_endpoints().get_endpoints();

    let mut status: Vec<bool>;

    loop{
        status = ping_server(load_realtime, loaded_servers.clone()).await;
        sleep(Duration::from_secs(3)).await;
        // If return is empty that means we weren't able to reach cloudflare DNS server
        // which means returned Vec is empty
        if !status.is_empty(){
            info!("Status of server: {:?}", status);
            // Collect all servers which doesn't respond
            let mut failed_servers = Vec::new();
            let mut i = 0;
            let mut servers_refreshed = false;
            while i < status.len(){
                if !status[i]{
                    if !servers_refreshed{
                        if load_realtime{
                            info!("Updated servers");
                            loaded_servers = json_handler::get_endpoints().get_endpoints();
                        }
                        servers_refreshed = true;
                    }
                    // If user will update json in wrong moment than updated servers would not
                    // contain servers which doesnt work
                    if loaded_servers.len() == status.len(){
                        failed_servers.push(loaded_servers.get(i).unwrap().to_string());
                    }
                }
                i = i + 1;
            }
            info!("{:?}", failed_servers);
        }
    }
}

async fn ping_server(load_realtime: bool, given_servers: Vec<String>) -> Vec<bool> {

    let servers: Vec<String>;

    if load_realtime{
       servers = json_handler::get_endpoints().get_endpoints();
    }
    else{
        servers = given_servers;
    }

    let mut status_list: Vec<bool> = Vec::new();

    // Check if cloudflare dns server works, if not than we don't have access to internet probably
    match TcpStream::connect("1.1.1.1:80").await{
        Ok(_) => {},
        Err(_) => {
            error!("Cannot reach cloudflare dns server. \
        That means you could have problem with internet connection");
            return status_list;
        }
    }

    for server_ip in servers.iter(){
        let server = TcpStream::connect(server_ip).await;
        if server.is_ok() {
            status_list.push(true);
        }
        else{
            status_list.push(false);
        }
    }

    return status_list;
}