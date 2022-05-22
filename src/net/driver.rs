use std::io;
use std::net::UdpSocket;
use std::net::IpAddr;
use std::net::SocketAddr;
use crate::net::message::Message;
use bincode::{config, Encode, Decode};

const PORT: u16 = 8888;

#[derive(Debug)]
struct Driver {
    peers: Vec<IpAddr>,
    local_socket: UdpSocket,
}

impl Driver {
    pub fn new(peers: Vec<IpAddr>, local_address: IpAddr) -> Self {
        Driver{peers, local_socket: UdpSocket::bind(SocketAddr::new(local_address, PORT))? }
    }

    pub fn send(&self, message: &Message, peer: IpAddr) -> io::Result<usize>{
        self.local_socket.send_to(bincode::encode_to_vec(&message, bincode::config::standard())?.as_slice(), peer)
    }

    pub fn broadcast(&self, message: &Message) {
        for peer in self.peers {
            self.send(message, peer).expect("fail to send message");
        }
    }
}