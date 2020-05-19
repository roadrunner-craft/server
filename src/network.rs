use crate::player::PlayerId;
use crate::{IP, PORT};

use core::events::{ClientEvent, ServerEvent};
use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use uuid::Uuid;

const MAX_MESSAGE_SIZE: usize = 65535;

pub struct NetworkHandler {
    listener: UdpSocket,
    socket_to_player: HashMap<SocketAddr, PlayerId>,
    player_to_socket: HashMap<PlayerId, SocketAddr>,
}

impl NetworkHandler {
    pub fn new() -> io::Result<Self> {
        let listener = UdpSocket::bind(format!("{}:{}", IP.flag, PORT.flag))?;
        listener.set_nonblocking(true).unwrap();

        Ok(Self {
            listener,
            socket_to_player: HashMap::new(),
            player_to_socket: HashMap::new(),
        })
    }

    pub fn poll(&mut self) -> Option<(PlayerId, ClientEvent)> {
        let mut data = [0; MAX_MESSAGE_SIZE];

        let (_, socket) = self.listener.recv_from(&mut data).ok()?;

        let id = if let Some(id) = self.socket_to_player.get(&socket) {
            *id
        } else {
            let id = Uuid::new_v4().as_u128();
            self.socket_to_player.insert(socket, id);
            self.player_to_socket.insert(id, socket);
            id
        };

        bincode::deserialize(&data).map(|event| (id, event)).ok()
    }

    pub fn send(&self, player_id: &PlayerId, event: &ServerEvent) {
        if let Some(socket) = self.player_to_socket.get(player_id) {
            let buffer = bincode::serialize(event).unwrap();
            let _ = self.listener.send_to(&buffer, socket);
        }
    }

    pub fn broadcast_except(&self, player_id: &PlayerId, event: ServerEvent) {
        for player in self.player_to_socket.keys() {
            if *player != *player_id {
                self.send(player, &event)
            }
        }
    }

    pub fn broadcast(&self, event: ServerEvent) {
        for player in self.player_to_socket.keys() {
            self.send(player, &event)
        }
    }
}
