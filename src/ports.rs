use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};
pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain{
    let socket_addresses : Vec<SocketAddr> = format!("{}:1024",subdomain.domain).to_socket_addrs()
        .expect("port scanner error")
        .collect();
    if socket_addresses.len() == 0{
        return subdomain
    }
    subdomain.open_ports = MOST_COMMON_PORTS_100.into_iter().map(|port| scan_port(socket_addresses[0],*port))
        .filter(|port|port.is_open)
        .collect();
    subdomain
}
fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_address.set_port(port);

    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok();

    Port { port, is_open }
}