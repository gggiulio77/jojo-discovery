use std::net::{Ipv4Addr, TcpListener};

use anyhow::bail;
use local_ip_address::local_ip;
use log::*;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub async fn init_broadcast<A: ToSocketAddrs>(
    bind_address: A,
    server_port: u16,
) -> anyhow::Result<()> {
    let socket = UdpSocket::bind(bind_address).await.unwrap();
    socket.set_broadcast(true).unwrap();

    info!(
        "[broadcast]: listening UDP on {:?}",
        socket.local_addr().unwrap()
    );

    let task = tokio::spawn(async move {
        let mut buffer = [0; 512];
        loop {
            // TODO: think a way to make this more secure, maybe encrypt payload with a date or something and encrypt/decrypt in both ends
            let (len, addr) = socket.recv_from(&mut buffer).await.unwrap();
            info!("[broadcast]: {:?} bytes received from {:?}", len, addr);

            let len = socket
                .send_to(&server_port.to_string().as_bytes(), addr)
                .await
                .unwrap();
            info!("{:?} bytes sent", len);
        }
    });

    task.await?
}

pub fn get_available_port(local_ip: Ipv4Addr) -> Option<u16> {
    (3000..9000).find(|port| port_is_available(local_ip, *port))
}

pub fn port_is_available(local_ip: Ipv4Addr, port: u16) -> bool {
    match TcpListener::bind((local_ip, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_local_ip() -> anyhow::Result<Ipv4Addr> {
    match local_ip()? {
        std::net::IpAddr::V4(ip) => Ok(ip),
        _ => {
            bail!("[init]: local IpV4 not found")
        }
    }
}

// TODO: implement the multicast version
