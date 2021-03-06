use mio::{self, tcp::*};
use std::net::SocketAddr;

mod client;
mod security;
mod server;

fn main() {
    let address = "0.0.0.0:4200".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();
    let mut event_loop: mio::EventLoop<server::WebSocketServer> = mio::EventLoop::new().unwrap();

    let mut server = server::WebSocketServer::new(server_socket);

    event_loop
        .register(
            &server.socket,
            mio::Token(0),
            mio::EventSet::readable(),
            mio::PollOpt::edge(),
        )
        .unwrap();

    event_loop.run(&mut server).unwrap();
}
