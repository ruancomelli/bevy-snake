# Bevy Snake Game

A simple snake game built with Bevy by following [Marcus Buffett's excellent tutorial](https://mbuffett.com/posts/bevy-snake-tutorial/).

The goal of this project is to learn the basics of Bevy and improve my Rust skills by building a simple Snake game.

Some differences from the tutorial:

- I used the latest version of Bevy (0.15.3 at the time of writing)
- I split the game functionality into multiple Bevy plugins
- I addressed a couple of bugs mentioned in the tutorial
- I decided to go for a warping border instead of a wall
- I added [a quit plugin](src/plugins/quit.rs) to support multiple key bindings to exit a bevy game. This plugin is adapted from [Joan Antoni's `bevy_quit`](https://github.com/joanantonio/bevy_quit) to work with the latest version of Bevy.

## Running the game

```bash
cargo run
```

## Gameplay

Use the arrow keys to move the snake. The goal is to eat as much food as possible without hitting the snake's own body. If you hit the edge of the screen, you will wrap around to the other side.

## Controls

- Arrow keys (<kbd>←</kbd>, <kbd>→</kbd>, <kbd>↑</kbd>, <kbd>↓</kbd>) : Move the snake
- <kbd>Q</kbd> : Quit the game

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
