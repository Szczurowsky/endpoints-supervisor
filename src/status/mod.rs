mod tcp;

use std::error::Error;

use color_eyre::Result;

#[derive(PartialEq)]
pub enum Methods{
    Tcp,
}

pub async fn check_status(method: &Methods, address: &String) -> Result<bool, Box<dyn Error>>{
    if &Methods::Tcp == method{
        return tcp::check_status(address).await;
    }
    Ok(false)
}
