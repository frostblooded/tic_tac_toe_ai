use std::fmt;
use std::io::stdin;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum State {
    Player,
    Enemy,
}

#[derive(Debug, Clone)]
struct Board {
    data: Vec<Vec<Option<State>>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-----------\n")?;

        for i in 0..Board::BOARD_SIZE {
            for j in 0..Board::BOARD_SIZE {
                let val = &self.data[i][j];

                if let Some(v) = val {
                    if *v == State::Player {
                        write!(f, "|X| ")?;
                    } else {
                        write!(f, "|O| ")?;
                    }
                } else {
                    write!(f, "| | ")?;
                }
            }

            write!(f, "\n-----------\n")?;
        }

        Ok(())
    }
}

impl Board {
    const BOARD_SIZE: usize = 3;
    const MAX_SOLUTION_DEPTH: u32 = Board::BOARD_SIZE as u32 * Board::BOARD_SIZE as u32 + 1;

    fn new() -> Self {
        Board {
            data: vec![vec![None; Board::BOARD_SIZE]; Board::BOARD_SIZE],
        }
    }

    fn set_player(&mut self, x: usize, y: usize) {
        self.data[x][y] = Some(State::Player);
    }

    fn set_enemy(&mut self, x: usize, y: usize) {
        self.data[x][y] = Some(State::Enemy);
    }

    fn generate_children(&self, maximize: bool) -> Vec<(Board, usize, usize)> {
        let mut res = vec![];

        for i in 0..Board::BOARD_SIZE {
            for j in 0..Board::BOARD_SIZE {
                if self.data[i][j].is_none() {
                    let mut cloned_board = self.clone();

                    if maximize {
                        cloned_board.set_player(i, j);
                    } else {
                        cloned_board.set_enemy(i, j);
                    }

                    res.push((cloned_board, i, j));
                }
            }
        }

        res
    }

    fn check_row_same(&self, idx: usize, state: State) -> bool {
        for i in 0..Board::BOARD_SIZE {
            if self.data[idx][i] != Some(state) {
                return false;
            }
        }

        true
    }

    fn check_col_same(&self, idx: usize, state: State) -> bool {
        for i in 0..Board::BOARD_SIZE {
            if self.data[i][idx] != Some(state) {
                return false;
            }
        }

        true
    }

    fn check_main_diag_same(&self, state: State) -> bool {
        for i in 0..Board::BOARD_SIZE {
            if self.data[i][i] != Some(state) {
                return false;
            }
        }

        true
    }

    fn check_secondary_diag_same(&self, state: State) -> bool {
        for i in 0..Board::BOARD_SIZE {
            if self.data[i][Board::BOARD_SIZE - i - 1] != Some(state) {
                return false;
            }
        }

        true
    }

    fn is_full(&self) -> bool {
        for i in 0..Board::BOARD_SIZE {
            for j in 0..Board::BOARD_SIZE {
                if self.data[i][j].is_none() {
                    return false;
                }
            }
        }

        true
    }

    fn get_value(&self) -> i32 {
        if self.is_full() {
            return 2;
        }

        for i in 0..Board::BOARD_SIZE {
            if self.check_row_same(i, State::Player) {
                return 1;
            }

            if self.check_row_same(i, State::Enemy) {
                return -1;
            }

            if self.check_col_same(i, State::Player) {
                return 1;
            }

            if self.check_col_same(i, State::Enemy) {
                return -1;
            }
        }

        if self.check_main_diag_same(State::Player) {
            return 1;
        }

        if self.check_main_diag_same(State::Enemy) {
            return -1;
        }

        if self.check_secondary_diag_same(State::Player) {
            return 1;
        }

        if self.check_secondary_diag_same(State::Enemy) {
            return -1;
        }

        0
    }

    fn move_enemy_helper(
        &self,
        maximize: bool,
        depth: usize,
        mut alpha: Option<i32>,
        mut beta: Option<i32>,
    ) -> (i32, Option<(usize, usize)>) {
        let children = self.generate_children(maximize);
        let value = self.get_value();

        if value != 0 || children.len() == 0 {
            if value == 2 {
                return (0, None);
            }

            let used_value = if value == 1 {
                Board::MAX_SOLUTION_DEPTH as i32 - depth as i32
            } else {
                -(Board::MAX_SOLUTION_DEPTH as i32 - depth as i32)
            };

            //println!("Returning value {}", used_value);
            return (used_value, None);
        }

        let mut best_value = None;
        let mut best_move = None;

        for (child_board, x, y) in children {
            //println!(
            //"Depth {}, alpha {:?}, beta {:?}: Entering {:?} after placement of ({}, {})",
            //depth, alpha, beta, child_board, x, y
            //);

            let (value, _) = child_board.move_enemy_helper(!maximize, depth + 1, alpha, beta);

            if best_value.is_none() {
                best_value = Some(value);
                best_move = Some((x, y));
            } else if maximize {
                if value > best_value.unwrap() {
                    best_value = Some(value);
                    best_move = Some((x, y));
                }
            } else {
                if value < best_value.unwrap() {
                    best_value = Some(value);
                    best_move = Some((x, y));
                }
            }

            if maximize {
                if beta.is_some() && best_value.unwrap() >= beta.unwrap() {
                    //println!("Returning because child is higher than beta");
                    return (best_value.unwrap(), best_move);
                }

                if alpha.is_some() {
                    alpha = Some(alpha.unwrap().max(best_value.unwrap()));
                } else {
                    alpha = best_value;
                }
            } else {
                if alpha.is_some() && best_value.unwrap() <= alpha.unwrap() {
                    //println!("Returning because child is lower than alpha");
                    return (best_value.unwrap(), best_move);
                }

                if beta.is_some() {
                    beta = Some(beta.unwrap().min(best_value.unwrap()));
                } else {
                    beta = best_value;
                }
            }
        }

        (best_value.unwrap(), best_move)
    }

    fn move_enemy(&mut self) {
        let (value, coords) = self.move_enemy_helper(false, 0, None, None);
        let unwrapped_coords = coords.unwrap();

        println!("Best value: {:?}", (value, coords));
        self.set_enemy(unwrapped_coords.0, unwrapped_coords.1);
    }
}

fn main() {
    let mut buf = String::new();
    let mut board = Board::new();

    println!("Who is first? (0 - player, 1 - AI)");
    buf.clear();
    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from input");

    let first_player_arg: u8 = buf.trim().parse().expect("Couldn't parse input as number");

    if first_player_arg == 1 {
        board.move_enemy();
    }

    loop {
        println!("{}", board);

        buf.clear();
        stdin()
            .read_line(&mut buf)
            .expect("Couldn't read from input");

        let coords: Vec<usize> = buf
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().expect("Couldn't read input as numbers"))
            .collect();

        if coords.len() != 2 {
            println!("Error: input coordinates should be two");
            continue;
        }

        board.set_player(coords[0], coords[1]);

        // Check if player won or there is a tie.
        let value = board.get_value();

        if value == 1 {
            println!("You win!");
            println!("{}", board);
            break;
        } else if value == 2 {
            println!("Tie!");
            println!("{}", board);
            break;
        }

        board.move_enemy();

        // Check if AI won or there is a tie.
        let value = board.get_value();

        if value == -1 {
            println!("AI wins!");
            println!("{}", board);
            break;
        } else if value == 2 {
            println!("Tie!");
            println!("{}", board);
            break;
        }
    }
}
