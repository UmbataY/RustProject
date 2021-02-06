mod battle;
mod enemy;
mod armor;
mod player;
mod weapon;
mod game;
mod action;

use std::io::{stdin, Read};
use std::thread;
use std::time::Duration;
use console::Term;
use crate::battle::battle::Battle;
use crate::player::player::Player;
use crate::armor::armor::Armor;
use crate::weapon::weapon::Weapon;
use crate::game::Game;

// use crate::player::player::Player;
//
// use battle::battle::*;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref player1:Player = player::player::Player::new();
    static ref terminal:Term = Term::stdout();
    // static ref mut shopWeapon:[Weapon;3] = [
    //         Weapon::new(1),
    //         Weapon::new(1),
    //         Weapon::new(1)];
    // static ref mut shopArmor:[Armor;3] = [];

}


// static player:&Player = &player::player::Player::new();
// static mut terminal:Term = Term::stdout();
fn main() {
    let mut level = 0;
    let mut gold = 100;
    let mut skill_points = 3;

    // let player = &player::player::Player::new();
    // let terminal = Term::stdout();
    let mut battle:Battle;

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
