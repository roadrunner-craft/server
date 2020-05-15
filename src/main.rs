mod game;
mod network;
mod player;

use crate::game::Game;

use std::io;

gflags::define! {
    /// local address to bind, default: 0.0.0.0
    -s, --ip <IP> = "0.0.0.0"
}

gflags::define! {
    /// local port to bind, default: 25565
    -p, --port <PORT>: u16 = 25565
}

gflags::define! {
    /// show this help message
    -h, --help = false
}

fn main() -> io::Result<()> {
    gflags::parse();

    if HELP.flag {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        gflags::print_help_and_exit(0);
    }

    let mut game = Game::new()?;
    game.start();
    Ok(())
}
