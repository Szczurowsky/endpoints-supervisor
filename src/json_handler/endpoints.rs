use serde::{Deserialize, Serialize};

use tracing::{info, error};

use color_eyre::eyre::Result;

use directories::ProjectDirs;

use std::fs;
use std::path::PathBuf;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Endpoints {
    endpoints: Vec<String>,
}

impl Endpoints{

    pub fn get_endpoints(self) -> Vec<String> {
        return self.endpoints;
    }

}

pub fn read_endpoints () -> Result<String, Box<dyn Error>> {

    if let Some(project_dirs)= ProjectDirs::from("pl", "szczurowsky", "endpoint-supervisor"){

        let mut path = PathBuf::from(project_dirs.config_dir());
        path = path.join("endpoints");
        path.set_extension("json");

        if !path.exists(){

            info!("Creating endpoints file on {:?}", &path);

            let mut default = Vec::new();
            default.push("1.1.1.1:80".to_string());
            default.push("1.1.1.1:443".to_string());

            let endpoints = Endpoints {
                endpoints: default,
            };

            let file = serde_json::to_string_pretty(&endpoints)?;

            match fs::write(&path, &file){
                Ok(_) => {}
                Err(e) => {
                    error!("An error occurred when creating endpoints file {}", e);
                    return Err("Cannot create endpoints file".into());
                }
            }

            return Ok(file);

        }
        else{
            // load endpoints
            let file = fs::read_to_string(&path).unwrap();

            return Ok(file);
        }
    }
    Err("Cannot create project dir naming".into())
}