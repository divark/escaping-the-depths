# Escaping the Depths
## Description
Escaping the Depths is a dungeon exploration-based game tailored for Twitch whose only input is clicks on the screen.

This project is implemented in Rust using the Bevy game engine because
- Rust brings a lot of runtime errors as compile-time errors, increasing the efficiency of development.
- Bevy provides a lot of built-in functionality to do rendering and input handling.
- It is one of the few game engines that makes event handling easy, which is the basis of this game's design.

## How to Build and Run
1. Install Rust if you have not already.
2. (Linux only) Install Bevy's dependencies depending on the distribution [here.](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
3. Run `cargo test` to ensure everything is working as intended.
4. If all tests pass, run `cargo run` to start the game.

## License
The source code of this project uses the GPLv3 license. For more information, check out the LICENSE file.
