use serde::{Deserialize, Serialize};

use tracing::{info, error};

use color_eyre::eyre::Result;

use directories::ProjectDirs;

use std::fs;
use std::path::PathBuf;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    database_host: String,
    database_port: i128,
    database_user: String,
    database_password: String,
}

pub fn read_config () -> Result<String, Box<dyn Error>> {

    if let Some(project_dirs)= ProjectDirs::from("pl", "szczurowsky", "endpoint-supervisor"){

        let mut path = PathBuf::from(project_dirs.config_dir());
        path = path.join("config");
        path.set_extension("json");

        if !path.exists(){

            info!("Creating config file on {:?}", &path);

            let config = Config {
                database_host: "localhost".to_string(),
                database_port: 3306,
                database_user: "root".to_string(),
                database_password: "".to_string(),
            };

            let file = serde_json::to_string_pretty(&config)?;

            match fs::write(&path, &file){
                Ok(_) => {}
                Err(e) => {
                    error!("An error occurred when creating config file {}", e);
                    return Err("Cannot create config file".into());
                }
            }

            return Ok(file);

        }
        else{
            // load config
            let file = fs::read_to_string(&path).unwrap();

            return Ok(file);
        }
    }
    Err("Cannot create project dir naming".into())
}