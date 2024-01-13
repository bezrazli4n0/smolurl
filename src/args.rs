use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Server hostname.
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    /// Server port.
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Path to private key file.
    #[arg(long, default_value = "./private.pem")]
    pub private_key_path: PathBuf,

    /// Database URL.
    #[arg(short, long, default_value_t = String::from("postgres://postgres:mysecretpassword@localhost:5432/smolurl"))]
    pub db_url: String,
}
