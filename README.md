# Jump and Run Platform

This is a platform game ("endless runner") developed in the Rust programming language using the [Bevy](https://bevyengine.org/) game engine.

## About the Game
In this game, you control a character that automatically runs to the right through a continuously generated world. Your task is to jump over obstacles (such as logs and cacti) to survive and achieve a high score.

The game features:
- Procedurally generated terrain.
- Scoreboard system.
- Increasing difficulty (speed increases over time).

## Micro:bit Integration
The game is designed to be controlled via an external **BBC Micro:bit** connected via USB cable. It works by listening to the computer's Serial Port.

### Micro:bit Setup
To play, your Micro:bit must be programmed to send data over USB.
1. **Baud rate:** 115200.
2. **Trigger:** When you perform an action (e.g., press Button A or shake the device).
3. **Data:** It must send the text `"JUMP"` followed by a newline (`\n`).

## How to Run

### 1. Prerequisites
- [Rust and Cargo](https://www.rust-lang.org/tools/install) must be installed.
- The source code for this project.

### 2. Port Configuration
By default, the game looks for the Micro:bit on **COM8**. If your Micro:bit is on a different port:
1. Find your device's port (e.g., in Device Manager on Windows).
2. Open the file `src/microbit.rs`.
3. Edit line 18: `serialport::new("COM8", 115200)` to match your port (e.g., `"COM3"`).

### 3. Running the Game
Open a terminal in the project folder and run:

```bash
cargo run --target x86_64-pc-windows-msvc
```

*(Note: We use `--target x86_64-pc-windows-msvc` to ensure the game runs on your PC, as the `Cargo.toml` file contains settings for embedded cross-compilation which would otherwise attempt to deploy to the Micro:bit itself).*

If you have removed the `[build]` section from `Cargo.toml`, you can simply run:
```bash
cargo run
```

---
> Don't mind the errors, I'm just a silly boy.
