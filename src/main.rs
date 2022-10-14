//game to play tic tac toe
use std::fmt::Display;
use std::io::{stdin, stdout};


#[derive(Eq, PartialEq, Copy, Clone)]

enum Player {
    X,
    O,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
            }
        )
    }
}

#[derive(Copy, Clone)]//create struct for tic tac toe
struct Game {
    board: [[Option<Player>; 3]; 3],
    turn: Player,
    winner : Option<Player>,
}
impl Game {
    fn new() -> Game {
        Game {
            board: [[None; 3]; 3],
            turn: Player::X,
            winner: None,
        }
    }

    fn check_winner(&self, p:Player, row:usize, column:usize) -> Option<Player> {

        //check if row is a win
        if self.board[row][0] == Some(p) && self.board[row][1] == Some(p) && self.board[row][2] == Some(p) {
            return Some(p);
        }
        //check if column is win
        if self.board[0][column] == Some(p) && self.board[1][column] == Some(p) && self.board[2][column] == Some(p) {
            return Some(p);
        }

        //if center is player, check diagonals
        if self.board[1][1] == Some(p) {
            if self.board[0][0] == Some(p) && self.board[1][1] == Some(p) && self.board[2][2] == Some(p) {
                return Some(p);
            }
            if self.board[0][2] == Some(p) && self.board[1][1] == Some(p) && self.board[2][0] == Some(p) {
                return Some(p);
            }
        }
        
        None
    }


}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

    
        Ok(
            for row in self.board.iter() {
                for cell in row.iter() {
                    write!(f, "|")?;
                    write!(f, "{}", match cell {
                        Some(Player::X) => "X",
                        Some(Player::O) => "O",
                        None => " ",
                    })?;
                }
                writeln!(f, "|")?;
            }
        )


    }
}
fn get_pos(input:String) -> (usize,usize) {
    let mut input = input.split_whitespace();
    let row = input.next().unwrap().parse::<usize>().unwrap();
    let col = input.next().unwrap().parse::<usize>().unwrap();
    return (row-1,col-1);
}

fn new_turn (game:Game) {
    println!();
    println!("////////////////////////////////");
    println!();
    println!("{}", game);

}

fn main() {
    let mut game = Game::new();
    loop {
        new_turn(game);
        println!("Player {}'s turn", game.turn);
        println!("Enter a row and column (e.g. 1 2): ");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        
        let pos = get_pos(input);
        game.board[pos.0][pos.1] = Some(game.turn);
        game.winner = game.check_winner(game.turn, pos.0, pos.1);
        if game.winner != None {
            new_turn(game);
            println!("{} wins!", game.winner.unwrap());
            break;
        }
        game.turn = match game.turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        
    }
}
