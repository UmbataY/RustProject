use std::io::{stdin, Read};
use std::thread;
use std::time::Duration;
use console::Term;
use crate::battle::battle::Battle;
use crate::player::player::Player;
use crate::armor::armor::Armor;
use crate::weapon::weapon::Weapon;
use std::borrow::BorrowMut;

pub struct Game {
    terminal:Term,
    player:Player,
    shopWeapon:[Weapon;3],
    shopArmor:[Armor;3]
}

impl Game {
    pub fn new() -> Game {
        Game {
            terminal: Term::stdout(),
            player: Player::new(),
            shopWeapon:[
                Weapon::new(1),
                Weapon::new(1),
                Weapon::new(1)],
            shopArmor:[
                Armor::new(1),
                Armor::new(1),
                Armor::new(1)]
        }
    }
    pub fn play(&mut self) {
        let mut battle:Battle;

        // let mut s = String::new();
        // stdin().read_line(&mut s);
        let mut result: bool = true;
        loop {
            self.shopWeapon = [
                Weapon::new(self.player.get_level()),
                Weapon::new(self.player.get_level()),
                Weapon::new(self.player.get_level())];
            self.shopArmor = [
                Armor::new(self.player.get_level()),
                Armor::new(self.player.get_level()),
                Armor::new(self.player.get_level())];

            if result {
                self.player.add_gold(100);
                self.player.add_skillpoints(3);
                self.player.increase_level();
            } else {
                self.player.add_gold(50);
                self.player.add_skillpoints(1);
            }

            self.print_main_menu();
            battle = Battle::new(self.player.clone(),self.player.get_level());
            result = battle.fight();

            // stdin().read_line(&mut s);
        }
    }
    fn print_main_menu(&mut self) {
        self.terminal.clear_screen();
        println!("Main Menu");
        println!();
        println!("Choose where to go:");
        println!("1 - Character skills");
        println!("2 - Armour shop");
        println!("3 - Weapon shop");
        println!("4 - Fight in the arena");

        let mut input: String = String::from("");
        stdin().read_line(&mut input);
        match input.chars().nth(0).unwrap() {
            '1' => {
                self.print_skill_menu()
            },
            '2' => self.print_armor_menu(),
            '3' => self.print_weapon_menu(),
            '4' => println!("FIGHT"),
            _ => {
                println!("SKILLSASA")
            },
        }
    }

    fn print_skill_menu(&mut self) {
        self.terminal.clear_screen();
        println!("Your skills");
        println!("Strength: {} (Increases attack damage and defence strength)", self.player.get_strength());
        println!("Agility: {} (If it is higher than your opponents agility you start first)", self.player.get_agility());
        println!("Vitality: {} (Your health)", self.player.get_vitality());

        println!();
        println!();

        println!("You have {} skill points to distribute", self.player.get_skill_points());
        if self.player.get_skill_points() > 0 {
            println!("1 - Increase strength");
            println!("2 - Increase agility");
            println!("3 - Increase vitality");
        }
        println!("4 - Back to main menu");

        let mut input: String = String::from("");
        stdin().read_line(&mut input);
        // while input.chars().nth(0).unwrap() != '4' {
        //
        // }
        match input.chars().nth(0).unwrap() {
            '1' => {
                self.increaseSkill('1');
                self.print_skill_menu();
            },
            '2' => {
                self.increaseSkill('2');
                self.print_skill_menu();
            },
            '3' => {
                self.increaseSkill('3');
                self.print_skill_menu();
            },
            '4' => {
                self.print_main_menu();
            },
            _ => (),
        }
    }

    fn increaseSkill(&mut self, skill: char) {
        if self.player.get_skill_points() > 0 {
            self.player.spend_skillpoint();
            match skill {
                '1' => self.player.increase_strength(),
                '2' => self.player.increase_agility(),
                '3' => self.player.increase_vitality(),
                _ => print!("skill number does not match")
            }
        } else {
            println!("You do not have enough skill points");
        }
    }

    fn print_armor_menu(&mut self) {
        self.terminal.clear_screen();
        println!("Armor shop");
        println!();
        println!("1(head) :{}, price: {}", self.shopArmor[0].get_strength(), self.shopArmor[0].get_price());
        println!("2(body) :{}, price: {}", self.shopArmor[1].get_strength(), self.shopArmor[1].get_price());
        println!("3(legs) :{}, price: {}", self.shopArmor[2].get_strength(), self.shopArmor[2].get_price());
        println!("You have {} gold", self.player.get_gold());
        println!();
        println!();

        println!("Your armor:");
        println!();
        println!();
        let playerArmor = self.player.get_armor();
        println!("head :{}", playerArmor[0].get_strength());
        println!("body :{}", playerArmor[1].get_strength());
        println!("legs :{}", playerArmor[2].get_strength());


        println!();
        println!("Choose where to go:");
        println!("4 - Back to main menu");

        let mut input: String = String::from("");
        stdin().read_line(&mut input);
        match input.chars().nth(0).unwrap() {
            '1' => {
                self.buy_armor(0);
                self.print_armor_menu();
            },
            '2' => {
                self.buy_armor(1);
                self.print_armor_menu();
            },
            '3' => {
                self.buy_armor(2);
                self.print_armor_menu();
            },
            '4' => self.print_main_menu(),
            _ => (),
        }
    }

    fn buy_armor(&mut self, armor: usize) {
        if self.shopArmor[armor].get_price() <=  self.player.get_gold() {
            self.player.set_armor(armor, Armor::copyOf(&self.shopArmor[armor]));
            self.player.spend_gold(self.shopArmor[armor].get_price());
        } else {
            println!("You do not have enough money");
        }
    }

    fn print_weapon_menu(&mut self) {
        self.terminal.clear_screen();
        println!("Weapon shop");
        println!("1 :{}, price: {}", self.shopWeapon[0].get_damage(), self.shopWeapon[0].get_price());
        println!("2 :{}, price: {}", self.shopWeapon[1].get_damage(), self.shopWeapon[0].get_price());
        println!("3 :{}, price: {}", self.shopWeapon[2].get_damage(), self.shopWeapon[0].get_price());
        println!("You have {} gold", self.player.get_gold());
        println!();
        println!();

        println!("Your Weapon:");
        println!();
        println!();
        let playerWeapon = self.player.get_weapon();
        println!("weapon: {}", playerWeapon.get_damage());
        println!("4 - Back to main menu");

        let mut input: String = String::from("");
        stdin().read_line(&mut input);
        match input.chars().nth(0).unwrap() {
            '1' => {
                self.buy_weapon(0);
                self.print_weapon_menu();
            },
            '2' =>{
                self.buy_weapon(1);
                self.print_weapon_menu();
            },
            '3' => {
                self.buy_weapon(2);
                self.print_weapon_menu();
            },
            '4' => self.print_main_menu(),
            _ => (),
        }
    }

    fn buy_weapon(&mut self, weapon: usize) {
        if self.shopWeapon[weapon].get_price() <=  self.player.get_gold(){
            self.player.set_weapon(Weapon::copyOf(&self.shopWeapon[weapon]));
            self.player.spend_gold(self.shopWeapon[weapon].get_price());
        } else {
            println!("You do not have enough money");
        }
    }
}

