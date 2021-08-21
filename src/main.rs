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
        info!("Status of server: {:?}", status);
    }
}

async fn ping_server() -> Vec<bool> {

    let servers = ["1.1.1.1:80", "1.1.1.1:443"];

    let mut status_list: Vec<bool> = Vec::new();

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