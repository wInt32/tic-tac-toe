use scanf::scanf;

#[derive(Clone, Eq, PartialEq, Copy)]
enum Gamechar {
    Empty,
    X,
    O,
    WinnerNone,
}

impl std::fmt::Display for Gamechar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Empty => return write!(f, " "),
            Self::X => return write!(f, "X"),
            Self::O => return write!(f, "O"),
            Self::WinnerNone => return Err(std::fmt::Error),
        }
    }
}
fn print_game(rows: &Vec<Vec<Gamechar>>) {
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

fn validate_move(x: u8, y: u8, game: &Vec<Vec<Gamechar>>) -> bool {
    if x > 3 || y > 3 || x < 1 || y < 1 {
        return false;
    }

    if game[Into::<usize>::into(y - 1)][Into::<usize>::into(x - 1)] != Gamechar::Empty {
        return false;
    }

    return true;
}

fn who_won(game: &Vec<Vec<Gamechar>>) -> Gamechar {
    let top_left = game[0][0];
    let top_middle = game[0][1];
    let top_right = game[0][2];

    let middle_left = game[1][0];
    let center = game[1][1];
    let middle_right = game[1][2];

    let bottom_left = game[2][0];
    let bottom_middle = game[2][1];
    let bottom_right = game[2][2];

    if center == middle_right && center == middle_left {
        return center;
    }

    if center == top_middle && center == bottom_middle {
        return center;
    }

    if center == top_right && center == bottom_left {
        return center;
    }

    if center == top_left && center == bottom_right {
        return center;
    }

    if top_middle == top_left && top_middle == top_right {
        return top_middle;
    }

    if bottom_middle == bottom_left && bottom_middle == bottom_right {
        return bottom_middle;
    }

    if middle_left == top_left && middle_left == bottom_left {
        return middle_left;
    }

    if middle_right == top_right && middle_right == bottom_right {
        return middle_right;
    }

    for row in game {
        for cell in row {
            if *cell == Gamechar::Empty {
                return Gamechar::Empty;
            }
        }
    }

    return Gamechar::WinnerNone;
}

fn main() {
    let mut game = vec![vec![Gamechar::Empty; 3]; 3];

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
            game[y][x] = Gamechar::X;
        } else {
            game[y][x] = Gamechar::O;
        }

        let winner = who_won(&game);

        if winner == Gamechar::X {
            print_game(&game);
            println!();
            println!("Congratulations! X has won the game.");
            break;
        } else if winner == Gamechar::O {
            print_game(&game);
            println!();
            println!("Congratulations! O has won the game.");
            break;
        } else if winner == Gamechar::WinnerNone {
            print_game(&game);
            println!();
            println!("It's a draw!");
            break;
        }
    }
}
