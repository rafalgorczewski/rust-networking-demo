use std::net::SocketAddrV4;

pub struct HostRequestedEvent;
pub struct ConnectRequestedEvent(pub SocketAddrV4);
pub struct GameStartRequestedEvent;
