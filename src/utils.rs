use jwt_simple::prelude::*;
use std::{fs, path::PathBuf};

pub fn load_keypair_from_file(path: PathBuf) -> Result<RS384KeyPair, anyhow::Error> {
    let private_key_file_content = fs::read_to_string(path)?;
    let keypair = RS384KeyPair::from_pem(&private_key_file_content)?;

    Ok(keypair)
}
