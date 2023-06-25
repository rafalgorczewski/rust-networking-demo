use super::messages::{MessageToClient, MessageToServer};

#[derive(Clone)]
pub struct SendMessageToServerEvent(pub MessageToServer);
#[derive(Clone)]
pub struct SendMessageToClientEvent(pub MessageToClient);
