use std::net::TcpListener;
use zero2prod::{startup::run, configuration::get_configuration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get config file.");
    let address = format!("127.0.0.1:{}",configuration.application_port);
    let address = TcpListener::bind(address)?;
    run(address)?.await
}
