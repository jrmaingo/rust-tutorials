use std::io;

// Board consistes of 3x3 grid
struct Board {
    spaces: Vec<Vec<i32>>,
}

// player structure
struct Player {
    num: i32,
}

// methods for Board
impl Board {
    // creates new board
    fn new_board() -> Board {
        Board { spaces: vec![ vec![0;3] ;3] }
    }

    // prints out the current state of the board
    fn print_board(&self) {
        let mut output = String::new();
        for i in 0..3 {
            for j in 0..3 {
                let symbol = match self.spaces[i][j] {
                    0 => " ",
                    1 => "X",
                    2 => "O",
                    _ => "?",
                };
                output.push_str(symbol);
                if j < 2 {
                    output.push_str("|");
                }
            }
            output.push_str("\n");
            if i < 2 {
                output.push_str("-----\n");
            }
        }
        println!("{}", output);
    }

    // assigns value ot space if empty
    fn set_space(& mut self, x: usize, y: usize , p: &Player) {
        match self.spaces[y][x] {
            0 => self.spaces[y][x] = p.num,
            _ => self.spaces[y][x] = 3,
        };
    }

    // check if there is a winner
    fn has_winner(& self) -> bool {
        false
    }
}

fn main() {
    let mut board = Board::new_board();
    let p1 = Player { num: 1 };
    let p2 = Player { num: 2 };
    board.set_space(1, 1, &p1);
    board.set_space(0, 0, &p2);
    let mut curr_player = &p1;

    while !board.has_winner() {
        board.print_board();
        println!("Current player: {}", curr_player.num);

        let mut input = String::new();
        println!("Input: ");
        io::stdin().read_line(&mut input);
        print!("{}", input);

        curr_player = if curr_player.num == p1.num { &p2 } else { &p1 };
    }
}
