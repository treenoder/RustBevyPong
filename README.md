# RustBevyPong

RustBevyPong is a simple Pong game created to learn the Bevy game engine using the Rust programming language.

## Features

- Basic Pong game mechanics
- Player paddles
- Ball movement and collision
- Simple game window setup

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/treenoder/RustBevyPong.git
    cd RustBevyPong
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the game:
    ```sh
    cargo run
    ```

## Project Structure

- `src/main.rs`: Entry point of the application, sets up the Bevy app and systems.
- `src/consts.rs`: Contains constant values used throughout the game.
- `src/components.rs`: Basic game components (Paddle, Ball, etc).
- `src/systems/mod.rs`: Module for game systems.
- `src/systems/basic.rs`: Basic game systems like spawning entities and handling input.

## Controls

- `Esc`: Exit the game
- `W`/`S`: Move left paddle up/down
- `Up Arrow`/`Down Arrow`: Move right paddle up/down

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgments

- [Bevy Game Engine](https://bevyengine.org/)
- Rust Programming Language
