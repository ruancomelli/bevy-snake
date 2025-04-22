# Bevy Snake Game

A simple snake game built with Bevy by following [Marcus Buffett's excellent tutorial](https://mbuffett.com/posts/bevy-snake-tutorial/).

The goal of this project is to learn the basics of Bevy and improve my Rust skills by building a simple Snake game.

Some differences from the tutorial:

- I used the latest version of Bevy (0.15.3 at the time of writing)
- I separated game functionality into Bevy plugins
- I addressed a couple of bugs mentioned in the tutorial
- I decided to go for a warping border instead of a wall

## Running the game

```bash
cargo run
```

## Gameplay

Use the arrow keys to move the snake. The goal is to eat as much food as possible without hitting the snake's own body. If you hit the edge of the screen, you will wrap around to the other side.

## Controls

- Arrow keys (←, →, ↑, ↓) : Move the snake
- Q: Quit the game

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
