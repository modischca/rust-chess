use crate::errors::GameErr::IllegalMove;
use crate::errors::GameResult;
use crate::game::Game;

mod errors;
mod game;
mod tests;
#[allow(unused)]
fn main() {
    let g = Game::new();
    print(&g).expect("Error");
}

pub fn print(g: &Game) -> GameResult<()> {
    let mut str = String::from("");
    g.board.iter().enumerate().for_each(|(i, x)| {
        if i % 8 == 0 {
            str.push_str("\n");
        }
        if let Some(x) = x {
            str.push_str(format!(" {},{} ", x.get_char_code(), i).as_str());

        } else {
            str.push_str(format!(" {} ", i + 1).as_str());

        }
    });

    for i in (0..str.len()).rev() {
        if let Some(p) = str.chars().nth(i) {
            print!("{}", p);
        }
    }
    Ok(())
}
