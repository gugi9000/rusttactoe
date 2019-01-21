#[macro_use]
extern crate text_io;
use ansi_term::Colour;
use std::io::stdout;
use std::io::Write;

fn clear() {
    // TODO: Make this work on Windows #1
    // NOTE: Should work on Windows ?
    print!("{}[2J", 27 as char);
}

fn colourize(piece: &str) -> String {
    match piece {
        "X" => Colour::Blue.bold().paint(piece).to_string(),
        "O" => Colour::Yellow.bold().paint(piece).to_string(),
        _ => piece.to_string(),
    }
}

fn draw_board(board: [&str; 9]) {
    let tab = " ".repeat(9);
    println!("");
    println!(
        "{} {} | {} | {}",
        tab,
        colourize(&board[0]),
        colourize(&board[1]),
        colourize(&board[2])
    );
    println!("{}--- --- ---", tab);
    println!(
        "{} {} | {} | {}",
        tab,
        colourize(&board[3]),
        colourize(&board[4]),
        colourize(&board[5])
    );
    println!("{}--- --- ---", tab);
    println!(
        "{} {} | {} | {}",
        tab,
        colourize(&board[6]),
        colourize(&board[7]),
        colourize(&board[8])
    );
    println!("");
}

fn get_move(player: &str) -> usize {
    loop {
        print!("Player {}, place your marker: ", colourize(&player));
        stdout().flush().expect("Could not flush stdout.");
        let input: String = read!("{}\n");

        match input.trim().parse::<usize>() {
            Ok(n @ 1...9) => return n - 1,
            Ok(_) => println!("Outside the board.. 🤦‍"),
            Err(_) => println!("Invalid input."),
        }
    }
}

fn winner(board: [&str; 9]) -> bool {
    (board[0] == board[1]) && (board[1] == board[2])
        || (board[3] == board[4]) && (board[4] == board[5])
        || (board[6] == board[7]) && (board[7] == board[8])
        || (board[0] == board[3]) && (board[3] == board[6])
        || (board[0] == board[4]) && (board[4] == board[8])
        || (board[1] == board[4]) && (board[4] == board[7])
        || (board[2] == board[5]) && (board[5] == board[8])
        || (board[2] == board[4]) && (board[4] == board[6])
}

fn available(board: [&str; 9], placement: usize) -> bool {
    board[placement] != "X" && board[placement] != "O"
}

fn main() {
    let mut player = "X";
    let mut board = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut turns = 0;
    loop {
        if turns == 9 {
            println!("Turns out there are no available spots left.");
            println!("Game has tied.");
            break;
        }
        draw_board(board);
        let move_ = get_move(player);
        if available(board, move_) {
            turns += 1;
            board[move_] = player;
            if winner(board) {
                clear();
                println!("Player {} wins on move {}!!", player, turns);
                draw_board(board);
                break;
            }
            if player == "X" {
                player = "O";
            } else {
                player = "X"
            }
        } else {
            println!("Invalid move. Try again.");
        }
    }
    println!("Bye!");
}
