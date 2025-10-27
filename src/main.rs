use crate::errors::GameErr::IllegalMove;
use crate::errors::GameResult;
use crate::game::Game;

mod errors;
mod game;
mod tests;
mod ruleengine;

#[allow(unused)]
fn main() {
    println!("Welcome to the chess game!");
    println!("Type 'QUIT' to quit.");
    let mut g = Game::new();
    print(&g);
    read_user_input(g);
}

fn read_user_input(mut g: Game) {
    let mut input = String::from("");
    println!("Next move is: {}", &g.current_player.display());
    println!("Please enter a move in the format 'a1=>a2': ");
    std::io::stdin().read_line(&mut input).expect("Invalid input");
    let input_as_vec = input.trim().split("=>").collect::<Vec<&str>>();

    if (input.trim().to_uppercase() == "QUIT") {
        println!("Goodbye!");
        return;
    }
    let moved = match parse_input(input_as_vec) {
        Ok((from, to)) => {
            &g.move_piece((from.0, from.1 as i32), (to.0, to.1 as i32))
        },
        Err (e) => {
            println!("Error: {}", e);
            return read_user_input(g)
         }
    };
    match moved {
        Ok(_) => {
            print(&g);
            read_user_input(g);
        }
        Err(e) => {
            println!("Error: {}", e);
            read_user_input(g);
        },
    }
}

fn parse_input(input: Vec<&str>) -> Result<((char, i32),(char, i32)), &str> {
    if input.len() != 2 {
        return Err("Invalid input");
    }
    let from = input[0];
    let to = input[1];

    match (
        from.chars().nth(0),
        from.chars().nth(1),
        from.chars().nth(1).unwrap().to_digit(10),
        to.chars().nth(0),
        to.chars().nth(1),
        to.chars().nth(1).unwrap().to_digit(10)
    ) {
        (Some(from_col), Some(_), Some(from_row), Some(to_col), Some(_), Some(to_row)) =>
            Ok(((from_col, from_row as i32), (to_col, to_row as i32)))
        ,
        _ => Err("Invalid input"),
    }
}

pub fn print(g: &Game) {
    let mut str = String::from("");
    let mut board_rows: [[char; 8]; 8] = [[' '; 8];8];
    let mut row_index = 0;
    let mut col_index = 0;
    for i in (0..g.board.len()).rev(){

        if let Some(p) = g.board[i] {
            board_rows[row_index][col_index] = p.get_char_code();

        } else {
            board_rows[row_index][col_index] = '\u{25A1}';
        }
        if col_index == 7 {
            board_rows[row_index].reverse();
            row_index += 1;
            col_index = 0;
        } else {
            col_index += 1;
        }

    }
    str.push_str("\n\r");
    str.push_str(&format!("Score WHITE {} \n", &g.score_white));
    str.push_str(&format!("Score BLACK {} \n", &g.score_black));
    for (z, row) in board_rows.iter().enumerate() {
        for (i, col) in row.iter().enumerate() {
            if i == 0 {
                str.push_str(&format!("{}    {} " ,(8 - z), col.to_string().as_str()));
            } else {
                str.push_str(&format!(" {} " ,col.to_string().as_str()));
            }

        }
        str.push('\n');
    }
    str.push_str("\n\r");
    str.push_str("     a  b  c  d  e  f  g  h \n");

    println!("{}", str);

}
