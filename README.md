# server

Game server for roadrunner, a voxel game. It owns the authoritative world
state and relays player events — connections, disconnections, and movement —
to every connected client over UDP, running a fixed 20 ticks-per-second game
loop.

## Quick start

The server depends on the `core` and `math` crates through path dependencies,
so the three repositories must be cloned side by side:

```sh
git clone git@github.com:roadrunner-craft/core.git
git clone git@github.com:roadrunner-craft/math.git
git clone git@github.com:roadrunner-craft/server.git
cd server
mise run run
```

## Usage

By default the server binds `0.0.0.0:25565`. Both parts are configurable:

```sh
mise run run -- --ip 127.0.0.1 --port 4000
```

| Flag | Description |
| --- | --- |
| `-s`, `--ip <IP>` | local address to bind, default `0.0.0.0` |
| `-p`, `--port <PORT>` | local port to bind, default `25565` |
| `-h`, `--help` | show the help message |

Clients that stop sending events are disconnected after two seconds of
inactivity.
