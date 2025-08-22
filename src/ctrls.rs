use std::{cmp::Ordering, env::args, fmt::{self, Display}, io::{stdout, Write}, ops::Neg};

use rand::{random_range, seq::IndexedRandom};
use text_io::{read, try_read, try_scan};

use crate::{Board, Player, WIN_CASES};

fn human_box() -> Box<dyn Controller> {
    Box::new(Human)
}

pub struct Controllers {
    pub x: Box<dyn Controller>,
    pub o: Box<dyn Controller>,
}
impl Controllers {
    pub fn new_from_env_args() -> Result<Self, &'static str> {
        let mut args = args()
            .skip(1)
            .map(|s| Ok(match &*s {
                "human" | "h" => human_box(),
                "random" | "r" => Box::new(Random),
                "randomsmart" | "rs" => Box::new(RandomSmart),
                "eval" | "e" => Box::new(Eval),
                _ => return Err("Unknown player type"),
            }));
        Ok(Self {
            x: args.next().unwrap_or_else(|| Ok(human_box()))?,
            o: args.next().unwrap_or_else(|| Ok(human_box()))?,
        })
    }
    pub fn get_move(&self, player: Player, board: &Board) -> usize {
        match player {
            Player::X => self.x.get_move(player, board),
            Player::O => self.o.get_move(player, board),
        }
    }
}

pub trait Controller {
    fn get_move(&self, player: Player, board: &Board) -> usize;
}

pub struct Human;
impl Controller for Human {
    fn get_move(&self, player: Player, _: &Board) -> usize {
        loop {
            print!("Player {}, place your marker: ", player.colourize());
            stdout().flush().expect("Could not flush stdout.");
            let input: String = read!("{}\n");

            match input.trim().parse::<usize>() {
                Ok(n @ 1..=9) => return n - 1,
                Ok(_) => println!("Outside the board.. 🤦‍"),
                Err(_) => println!("Invalid input."),
            }
        }
    }
}
fn possible_moves<'a>(board: &'a Board) -> impl Iterator<Item=usize> + use<'a> {
    board.board
        .iter()
        .enumerate()
        .filter_map(|(i, f)| f.is_none().then_some(i))
}
pub struct Random;
impl Controller for Random {
    fn get_move(&self, _: Player, board: &Board) -> usize {
        let choices: Vec<_> = possible_moves(board).collect();
        *choices.choose(&mut rand::rng()).unwrap()
    }
}
pub struct RandomSmart;
impl Controller for RandomSmart {
    fn get_move(&self, me: Player, board: &Board) -> usize {
        let mut danger = None;
        'danger: for (a, b, c) in WIN_CASES {
            let mut none_field = None;
            let (mut cme, mut cop) = (0, 0);
            for i in [a, b, c] {
                match board.board[i] {
                    Some(p) => if p == me { cme += 1 } else { cop += 1},
                    None if none_field.is_none() => none_field = Some(i),
                    // too many empty fields, move on
                    None => continue 'danger,
                }
            }
            if let Some(danger_spot) = none_field {
                if cme == 2 {
                    // We will win instantly if we take this
                    return danger_spot;
                } else if cop == 2 {
                    // There is a danger but don't return immediately in case we can win instead
                    danger = Some(danger_spot);
                }
                // otherwise it is a mixed spot and not a real danger spot
            }
        }
        // Avoid losing
        if let Some(danger_spot) = danger {
            return danger_spot;
        }

        Random.get_move(me, board)
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Evaluation {
    #[default]
    Draw,
    Loss(u8),
    Win(u8),
}
impl Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Evaluation::Draw => write!(f, "draw"),
            Evaluation::Loss(t) => write!(f, "loss in {t}"),
            Evaluation::Win(t) => write!(f, "win in {t}"),
        }
    }
}
impl PartialOrd for Evaluation {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Evaluation {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Evaluation::Win(_), Evaluation::Loss(_)) => Ordering::Greater,
            (Evaluation::Win(_), Evaluation::Draw) => Ordering::Greater,
            (Evaluation::Draw, Evaluation::Loss(_)) => Ordering::Greater,
            (Evaluation::Win(s), Evaluation::Win(o)) => o.cmp(s),
            (Evaluation::Draw, Evaluation::Draw) => Ordering::Equal,
            (Evaluation::Loss(s), Evaluation::Loss(o)) => s.cmp(o),
            (Evaluation::Draw, Evaluation::Win(_)) => Ordering::Less,
            (Evaluation::Loss(_), Evaluation::Draw) => Ordering::Less,
            (Evaluation::Loss(_), Evaluation::Win(_)) => Ordering::Less,
        }
    }
}
impl Neg for Evaluation {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Evaluation::Draw => Evaluation::Draw,
            Evaluation::Loss(t) => Evaluation::Win(t),
            Evaluation::Win(t) => Evaluation::Loss(t),
        }
    }
}
fn eval(me: Player, board: &Board, depth: u8) -> (usize, Evaluation) {
    'danger: for (a, b, c) in WIN_CASES {
        let mut none_field = None;
        let mut cme = 0;
        for i in [a, b, c] {
            match board.board[i] {
                Some(p) => if p == me { cme += 1 },
                None if none_field.is_none() => none_field = Some(i),
                // too many empty fields, move on
                None => continue 'danger,
            }
        }
        if let Some(win_spot) = none_field {
            if cme == 2 {
                // We will win instantly if we take this
                return (win_spot, Evaluation::Win(depth));
            }
        }
    }
    const FUZZ: u8 = 8;

    possible_moves(board).map(|i| {
        let mut new_board = *board;
        new_board.board[i] = Some(me);
        let e = -eval(!me, &new_board, depth + 1).1;
        #[cfg(feature = "show-eval")]
        if depth == 0 {
            eprintln!("{} ({e})", i + 1);
        }
        (i, e)
    }).max_by_key(|e| (e.1, random_range(0..FUZZ))).unwrap_or_default()
}
pub struct Eval;
impl Controller for Eval {
    fn get_move(&self, me: Player, board: &Board) -> usize {
        let (best_move, _evaluation) = eval(me, board, 0);
        #[cfg(feature = "show-eval")]
        eprintln!("Best: {} ({_evaluation})", best_move + 1);
        best_move
    }
}
