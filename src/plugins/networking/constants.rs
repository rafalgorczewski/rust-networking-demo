use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

pub const LOCAL_ADDRESS_INGOING: SocketAddrV4 =
    SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5000);
pub const LOCAL_ADDRESS_OUTGOING: SocketAddrV4 =
    SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0);
pub const MAX_CLIENTS: usize = 5;
pub const PROTOCOL_ID: u64 = 7;

pub const CONNECTION_RETRY_INTERVAL: Duration = Duration::from_secs(1);
pub const CONNECTION_RETRY_TIMEOUT: Duration = Duration::from_secs(10);
