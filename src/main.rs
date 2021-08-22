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

    loop{
        sleep(Duration::from_secs(1)).await;
        let status = ping_server().await;

        // If return is empty that means we weren't able to reach cloudflare DNS server
        // which means returned Vec is empty
        if !status.is_empty(){
            info!("Status of server: {:?}", status);
        }
    }
}

async fn ping_server() -> Vec<bool> {

    let servers = ["1.1.1.1:80", "1.1.1.1:443"];

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