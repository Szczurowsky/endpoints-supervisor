use tokio::net::TcpStream;

use tracing::{error};

use std::error::Error;

use color_eyre::Result;

pub async fn check_status(address: &String) -> Result<bool, Box<dyn Error>>{

    match TcpStream::connect("1.1.1.1:80").await{
        Ok(_) => {},
        Err(err) => {
            error!("Cannot reach cloudflare DNS server which means that you could have problem with internet connection. Error: {}", err);
            return Err("No internet connection".into());
        }
    }
    let server = TcpStream::connect(address).await;
    Ok(server.is_ok())

}
