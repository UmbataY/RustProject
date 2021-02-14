mod battle;
mod enemy;
mod armor;
mod player;
mod weapon;
mod game;
mod action;
mod test;

// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     static ref player1:Player = player::player::Player::new();
//     static ref terminal:Term = Term::stdout();
// }

fn main() {
    // let mut level = 0;
    // let mut gold = 100;
    // let mut skill_points = 3;

    // let a = 23;
    //
    // let b = a as f32 * 0.5;
    //
    // println!("Test: {}",b as i32);

    // let player = &player::player::Player::new();
    // let terminal = Term::stdout();
    // let mut battle:Battle;

    // terminal = Term::stdout();
    // terminal.write_line("Hello World!");
    // thread::sleep(Duration::from_millis(2000));


    // let mut s = String::new();
    // stdin().read_line(&mut s);
    // s.remove(s.len() - 1);
    // print!("{}", s);
    // print!("s")


    let mut game = game::Game::new();
    game.play();
}
