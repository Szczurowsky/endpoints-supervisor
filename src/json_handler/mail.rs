use serde::{Deserialize, Serialize};

use tracing::{info, error};

use color_eyre::eyre::Result;

use directories::ProjectDirs;

use std::fs;
use std::path::PathBuf;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Mail {
    username: String,
    password: String,
    server: String,
    port: u16,
    destination_mail: String,
}

impl Mail{

    pub fn get_port(&self) -> u16{
        return self.port;
    }

    pub fn get_username(&self) -> &String {
        return &self.username;
    }

    pub fn get_password(&self) -> &String {
        return &self.password;
    }

    pub fn get_server(&self) -> &String {
        return &self.server;
    }
    
    pub fn get_destination_mail(&self) -> &String{
        return &self.destination_mail;
    }

}

pub fn read_mail () -> Result<String, Box<dyn Error>> {

    if let Some(project_dirs)= ProjectDirs::from("pl", "szczurowsky", "endpoint-supervisor"){

        let mut path = PathBuf::from(project_dirs.config_dir());
        path = path.join("mail");
        path.set_extension("json");

        if !path.exists(){

            info!("Creating mail file on {:?}", &path);

            let mail = Mail {
                username: "J".to_string(),
                password: "D".to_string(),
                server: "localhost".to_string(),
                port: 25,
                destination_mail: "some@mail.com".to_string()
            };

            let file = serde_json::to_string_pretty(&mail)?;

            match fs::write(&path, &file){
                Ok(_) => {}
                Err(e) => {
                    error!("An error occurred when creating mail file {}", e);
                    return Err("Cannot create mail file".into());
                }
            }

            return Ok(file);

        }
        else{
            // load mail
            let file = fs::read_to_string(&path).unwrap();

            return Ok(file);
        }
    }
    Err("Cannot create project dir naming".into())
}