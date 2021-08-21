mod config;

use directories::ProjectDirs;

use tracing::{info, error};

use std::fs;
use std::path::PathBuf;

pub fn init(){

    init_app_dir();

}

pub fn get_config() -> String {
    return config::read_config().unwrap();
}

fn init_app_dir(){
    if let Some(project_dirs)= ProjectDirs::from("pl", "szczurowsky", "endpoint-supervisor") {
        let path = PathBuf::from(project_dirs.config_dir());
        if !path.exists() {
            info!("Creating app directory on {:?}", &path);
            match fs::create_dir_all(&path){
                Ok(_) => {}
                Err(e) => error!("An error occurred when creating app directory: {:?}", e)
            }
        }
    }
}