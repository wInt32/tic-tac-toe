use scanf::scanf;

#[derive(Clone, Eq, PartialEq, Copy)]
enum GameTile {
    X,
    O,
    Empty,
}

enum GameResult {
    X,
    O,
    Draw,
}

impl From<GameTile> for GameResult {
    fn from(value: GameTile) -> Self {
        match value {
            GameTile::X => Self::X,
            GameTile::O => Self::O,
            GameTile::Empty => Self::Draw,
        }
    }
}

impl std::fmt::Display for GameTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Empty => write!(f, " "),
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}
fn print_game(rows: &[[GameTile; 3]; 3]) {
    for (i, row) in rows.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            print!(" {cell}");

            if j != 2 {
                print!(" |")
            }
        }
        println!();
        if i != 2 {
            for _n in 0..11 {
                print!("-");
            }
            println!();
        }
    }
}

fn validate_move(x: u8, y: u8, game: &[[GameTile; 3]; 3]) -> bool {
    if x > 3 || y > 3 || x < 1 || y < 1 {
        return false;
    }

    if game[Into::<usize>::into(y - 1)][Into::<usize>::into(x - 1)] != GameTile::Empty {
        return false;
    }

    true
}

fn update_game_state(game: &[[GameTile; 3]; 3]) -> Option<GameResult> {
    const X: GameTile = GameTile::X;
    const O: GameTile = GameTile::O;
    let winner = match game {
        // Rows
        [[X, X, X], [_, _, _], [_, _, _]] => X,
        [[O, O, O], [_, _, _], [_, _, _]] => O,
        [[_, _, _], [X, X, X], [_, _, _]] => X,
        [[_, _, _], [O, O, O], [_, _, _]] => O,
        [[_, _, _], [_, _, _], [X, X, X]] => X,
        [[_, _, _], [_, _, _], [O, O, O]] => O,
        // Columns
        [[X, _, _], [X, _, _], [X, _, _]] => X,
        [[O, _, _], [O, _, _], [O, _, _]] => O,
        [[_, X, _], [_, X, _], [_, X, _]] => X,
        [[_, O, _], [_, O, _], [_, O, _]] => O,
        [[_, _, X], [_, _, X], [_, _, X]] => X,
        [[_, _, O], [_, _, O], [_, _, O]] => O,
        // Diagonals
        [[X, _, _], [_, X, _], [_, _, X]] => X,
        [[O, _, _], [_, O, _], [_, _, O]] => O,
        [[_, _, X], [_, X, _], [X, _, _]] => X,
        [[_, _, O], [_, O, _], [O, _, _]] => O,

        g => {
            for row in g {
                for &cell in row {
                    if cell == GameTile::Empty {
                        return None;
                    }
                }
            }
            return Some(GameResult::Draw);
        }
    };
    Some(winner.into())
}

fn main() {
    let mut game = [[GameTile::Empty; 3]; 3];

    for i in 0.. {
        let x: usize;
        let y: usize;

        loop {
            let mut move_x = 0;
            let mut move_y = 0;

            print_game(&game);
            println!();

            if i % 2 == 0 {
                print!("X turn (x y): ");
            } else {
                print!("O turn (x y): ");
            }

            if scanf!("{} {}", move_x, move_y).is_ok() && validate_move(move_x, move_y, &game) {
                x = Into::<usize>::into(move_x) - 1;
                y = Into::<usize>::into(move_y) - 1;
                println!();
                break;
            }

            println!("Invalid input, try again.");
            println!();
        }

        if i % 2 == 0 {
            game[y][x] = GameTile::X;
        } else {
            game[y][x] = GameTile::O;
        }

        let result = update_game_state(&game);

        print_game(&game);
        println!();

        match result {
            None => continue,
            Some(GameResult::X) => println!("Congratulations! X has won the game."),
            Some(GameResult::O) => println!("Congratulations! O has won the game."),
            Some(GameResult::Draw) => println!("It's a draw!"),
        }
        break;
    }
}
