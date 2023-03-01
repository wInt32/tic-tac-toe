# tic-tac-toe
A simple 2 player tic tac toe game written in rust.

## Features
- 2 player gameplay, switching between players every turn
- only 164 LOC as of yet

## Usage
Use `cargo run` to run the program. Every turn it tells you which player is playing and it asks you for your turn.
You can enter a turn like this: `1 2` where the first number is the horizontal position (from right to left)
and the second is the vertical position (from top to bottom). After you enter your turn the game will print out
the new game board and ask the other player for a turn.
