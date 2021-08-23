use serde::{Deserialize, Serialize};

use tracing::{info, error};

use color_eyre::eyre::Result;

use directories::ProjectDirs;

use std::fs;
use std::path::PathBuf;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    load_realtime: bool,
    delay: u64
}

impl Config{

    pub fn get_load_realtime(&self) -> bool {
         return self.load_realtime;
    }

    pub fn get_delay(&self) -> u64{
        return self.delay;
    }

}

pub fn read_config () -> Result<String, Box<dyn Error>> {

    if let Some(project_dirs)= ProjectDirs::from("pl", "szczurowsky", "endpoint-supervisor"){

        let mut path = PathBuf::from(project_dirs.config_dir());
        path = path.join("config");
        path.set_extension("json");

        if !path.exists(){

            info!("Creating config file on {:?}", &path);

            let config = Config {
                load_realtime: false,
                delay: 10
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