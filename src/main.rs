use jojo_discovery::{
    discovery::{get_available_port, get_local_ip},
    initialize,
};
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let my_local_ip = get_local_ip().unwrap();

    let available_port = match get_available_port(my_local_ip) {
        Some(port) => {
            info!("[init]: port found {:?}", port);
            port
        }
        None => panic!("[init]: cannot find an available port on machine"),
    };

    let _ = initialize(available_port).await?;

    Ok(())
}
