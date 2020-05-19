use crate::network::NetworkHandler;
use crate::player::{Player, PlayerId};

use core::events::{ClientEvent, ServerEvent};
use core::world::World;
use std::collections::HashMap;
use std::io;
use std::thread;
use std::time::{Duration, Instant};

const SERVER_TICK_PER_SEC: u32 = 60;

pub struct Game {
    network: NetworkHandler,
    world: World,
    players: HashMap<PlayerId, Player>,
}

impl Game {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            network: NetworkHandler::new()?,
            world: World::new(),
            players: HashMap::new(),
        })
    }

    pub fn start(&mut self) {
        let mut last_time: Instant = Instant::now();

        loop {
            let elapsed = last_time.elapsed();

            let tick_duration = Duration::new(1, 0) / SERVER_TICK_PER_SEC;
            if let Some(cooldown) = tick_duration.checked_sub(elapsed) {
                thread::sleep(cooldown);
            }

            last_time = Instant::now();

            if let Some((id, event)) = self.network.poll() {
                self.handle_event(&id, &event);
            }

            //self.update();
        }
    }

    fn update(&mut self) {
        let positions = self.players.iter().map(|(_, n)| n.position).collect();
        self.world.load_around(positions);
    }

    fn handle_event(&mut self, id: &u128, event: &ClientEvent) {
        match event {
            ClientEvent::PlayerConnect => {
                let event = ServerEvent::PlayerList {
                    ids: self.players.keys().map(|n| *n).collect::<Vec<u128>>(),
                };
                self.network.send(id, &event);

                self.players.insert(*id, Player::new(*id));

                self.network
                    .broadcast_except(id, ServerEvent::PlayerConnected { id: *id });
            }
            ClientEvent::PlayerDisconnect => {
                self.players.remove(id);
                self.network
                    .broadcast_except(id, ServerEvent::PlayerDisconnected { id: *id });
            }
            ClientEvent::PlayerMove { position } => {
                if let Some(player) = self.players.get_mut(id) {
                    player.position = *position;
                    self.network.broadcast(ServerEvent::PlayerMoved {
                        id: *id,
                        position: *position,
                    });
                }
            }
        }
    }
}
