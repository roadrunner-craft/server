use crate::network::NetworkHandler;
use crate::player::{Player, PlayerId};

use core::events::{ClientEvent, ServerEvent};
use core::utils::logging;
use core::utils::sleep;
use core::world::World;
use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

const SERVER_TICK_PER_SEC: u32 = 20;
const INACTIVITY_TIMEOUT: u64 = 2;

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
        let expected_tick_duration = Duration::new(1, 0) / SERVER_TICK_PER_SEC;

        loop {
            let start = Instant::now();

            // maybe this function should have a timeout if something makes it run for too long the
            // server will become unresponsive
            self.update();

            // poll as many events as possible
            while start.elapsed() < expected_tick_duration {
                match self.network.poll() {
                    Some((id, event)) => self.handle_event(&id, &event),
                    None => break,
                }
            }

            // ms per tick
            let mspt = start.elapsed().as_secs_f64() * 1000.0;

            if let Some(cooldown) = expected_tick_duration.checked_sub(start.elapsed()) {
                sleep(cooldown);
            }

            // tick per second
            let tps = 1.0 / start.elapsed().as_secs_f64();

            // TODO: make a struct for ServerStats
            logging::info!(
                "mspt: {:.3}, tps: {:.1}, players: {}",
                mspt,
                tps,
                self.players.len()
            );
        }
    }

    fn update(&mut self) {
        self.disconnect_inactive_players();

        let positions = self.players.iter().map(|(_, n)| n.position()).collect();
        self.world.load_around(positions);
    }

    fn handle_event(&mut self, id: &u128, event: &ClientEvent) {
        match event {
            ClientEvent::PlayerConnect => {
                let event = ServerEvent::ServerInfo {
                    seed: self.world.seed().0,
                    player_ids: self.players.keys().map(|n| *n).collect::<Vec<u128>>(),
                };
                self.network.send(id, &event);

                self.players.insert(*id, Player::new(*id));

                self.network
                    .broadcast_except(id, ServerEvent::PlayerConnected { id: *id });
            }
            ClientEvent::PlayerDisconnect => self.disconnect_player(id),
            ClientEvent::PlayerMove { position } => {
                if let Some(player) = self.players.get_mut(id) {
                    player.set_position(*position);
                    self.network.broadcast(ServerEvent::PlayerMoved {
                        id: *id,
                        position: *position,
                    });
                }
            }
        }
    }

    fn disconnect_inactive_players(&mut self) {
        let inative_players = self
            .players
            .iter_mut()
            .filter(|(_, p)| p.last_update().elapsed().as_secs() >= INACTIVITY_TIMEOUT)
            .map(|(_, p)| p.id())
            .collect::<Vec<u128>>();

        for id in inative_players {
            self.disconnect_player(&id);
        }
    }

    fn disconnect_player(&mut self, id: &u128) {
        self.players.remove(id);
        self.network
            .broadcast(ServerEvent::PlayerDisconnected { id: *id });
    }
}
