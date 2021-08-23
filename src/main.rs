#![windows_subsystem = "windows"]

mod json_handler;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

use tracing::{error, info};
use tracing_subscriber;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    json_handler::init();

    // Load config
    let config = json_handler::get_config();
    let load_realtime = config.get_load_realtime();
    let delay = config.get_delay();

    // Init mail
    json_handler::get_mail();

    // Preload all endpoints from file
    let mut loaded_servers = json_handler::get_endpoints().get_endpoints();

    let mut status: Vec<bool>;

    // Create vec with list of all nodes which died
    let mut sent_mails = Vec::new();

    loop{
        status = ping_server(load_realtime, loaded_servers.clone()).await;
        // If return is empty that means we weren't able to reach cloudflare DNS server
        // which means returned Vec is empty
        if !status.is_empty(){
            info!("Status of server: {:?}", status);
            // Check status
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
                        sent_mails = send_mail(sent_mails, loaded_servers.get(i).unwrap().to_string());
                    }
                }
                else{
                    if sent_mails.contains(&loaded_servers.get(i).unwrap().to_string()){
                        sent_mails.remove(sent_mails.iter().
                            position(|r| r == &loaded_servers.get(i)
                                .unwrap().to_string()).unwrap());
                    }
                }
                i = i + 1;
            }
        }
        info!("{:?}", sent_mails);
        sleep(Duration::from_secs(delay)).await;
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

fn send_mail(mut sent_mails: Vec<String>, address: String) -> Vec<String> {
    if !sent_mails.contains(&address){

        //Load mail
        let mail = json_handler::get_mail();
        let mail_username = mail.get_username();
        let mail_password = mail.get_password();
        let mail_port = mail.get_port();
        let mail_server = mail.get_server();

        let credentials = Credentials::new(String::from(mail_username), String::from(mail_password));
        let mailer = SmtpTransport::relay(mail_server)
            .unwrap()
            .credentials(credentials)
            .port(mail_port)
            .build();

        let email = Message::builder()
            .from(format!("Endpoints supervisor <{}>", mail_username).parse().unwrap())
            .to(format!("Customer <{}>", mail.get_destination_mail()).parse().unwrap())
            .subject("Endpoint Supervisor - Alert")
            .body(format!("Hi, our system detect that node with ip {} doesnt work properly. Check it ASAP", address))
            .unwrap();

        match mailer.send(&email){
            Ok(_) => info!("Sent mail"),
            Err(e) => error!("{}", e),
        }

        sent_mails.push(address);
        return sent_mails;
    }
    return sent_mails;
}