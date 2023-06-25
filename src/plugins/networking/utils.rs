use bevy::prelude::*;
use bevy_renet::renet::{
    ChannelConfig, ClientAuthentication, ReliableChannelConfig, RenetClient, RenetConnectionConfig,
    RenetError, RenetServer, ServerAuthentication, ServerConfig,
};
use std::net::{SocketAddrV4, UdpSocket};
use std::time::{Duration, SystemTime};
use strum::{EnumCount, EnumIter};

use super::constants::{LOCAL_ADDRESS_INGOING, LOCAL_ADDRESS_OUTGOING, MAX_CLIENTS, PROTOCOL_ID};
use super::messages::{MessageToClient, MessageToServer};
use super::types::ClientId;

/// Remember to add relevant entries to the channels config below.
#[derive(Copy, Clone, EnumIter, EnumCount)]
pub enum FromServerChannel {
    LobbyMessages,
    Command,
}

/// Remember to add relevant entries to the channels config below.
#[derive(Copy, Clone, EnumIter, EnumCount)]
pub enum FromClientChannel {
    LobbyMessages,
    Input,
    Command,
}

impl FromServerChannel {
    fn channels_config() -> Vec<ChannelConfig> {
        let config = vec![
            ReliableChannelConfig {
                channel_id: Self::LobbyMessages as u8,
                message_resend_time: Duration::from_millis(200),
                ..Default::default()
            }
            .into(),
            ReliableChannelConfig {
                channel_id: Self::Command as u8,
                message_resend_time: Duration::ZERO,
                ..Default::default()
            }
            .into(),
        ];
        debug_assert!(config.len() == FromServerChannel::COUNT);
        config
    }
}

impl FromClientChannel {
    fn channels_config() -> Vec<ChannelConfig> {
        let config = vec![
            ReliableChannelConfig {
                channel_id: Self::LobbyMessages as u8,
                message_resend_time: Duration::from_millis(200),
                ..Default::default()
            }
            .into(),
            ReliableChannelConfig {
                channel_id: Self::Input as u8,
                message_resend_time: Duration::ZERO,
                ..Default::default()
            }
            .into(),
            ReliableChannelConfig {
                channel_id: Self::Command as u8,
                message_resend_time: Duration::ZERO,
                ..Default::default()
            }
            .into(),
        ];
        debug_assert!(config.len() == FromClientChannel::COUNT);
        config
    }
}

pub fn send_message_to_client(
    client_id: Option<ClientId>,
    message: MessageToClient,
    server: &mut RenetServer,
) {
    let serialization_result = bincode::serialize(&message);
    match serialization_result {
        Ok(serialized_message) => {
            if let Some(ClientId::Id(user_id)) = client_id {
                server.send_message(user_id, message.as_channel() as u8, serialized_message)
            } else {
                server.broadcast_message(message.as_channel() as u8, serialized_message)
            };
        }
        Err(error) => {
            error!("Couldn't serialize a message! Error: {}", error);
        }
    }
}

pub fn send_message_to_server(message: MessageToServer, client: &mut RenetClient) {
    let serialization_result = bincode::serialize(&message);
    match serialization_result {
        Ok(serialized_message) => {
            client.send_message(message.as_channel() as u8, serialized_message);
        }
        Err(error) => {
            error!("Couldn't serialize a message! Error: {}", error);
        }
    }
}

pub(super) fn construct_new_server() -> Result<RenetServer, std::io::Error> {
    let server_addr = LOCAL_ADDRESS_INGOING;
    let socket = UdpSocket::bind(server_addr)?;

    let connection_config = RenetConnectionConfig {
        send_channels_config: FromServerChannel::channels_config(),
        receive_channels_config: FromClientChannel::channels_config(),
        ..Default::default()
    };
    let server_config = ServerConfig::new(
        MAX_CLIENTS,
        PROTOCOL_ID,
        server_addr.into(),
        ServerAuthentication::Unsecure,
    );
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    RenetServer::new(current_time, server_config, connection_config, socket)
}

pub(super) fn construct_new_client(server_addr: SocketAddrV4) -> Result<RenetClient, RenetError> {
    let socket = UdpSocket::bind(LOCAL_ADDRESS_OUTGOING)?;

    let connection_config = RenetConnectionConfig {
        send_channels_config: FromClientChannel::channels_config(),
        receive_channels_config: FromServerChannel::channels_config(),
        ..Default::default()
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr: server_addr.into(),
        user_data: None,
    };

    RenetClient::new(current_time, socket, connection_config, authentication)
}
