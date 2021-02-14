pub mod battle {
    use crate::enemy::enemy::Enemy;
    use crate::player::player::Player;
    use crate::action::Action;
    use rand::Rng;
    use std::io::stdin;

    pub struct Battle {
        player: Player,
        enemy: Enemy
    }

    impl Battle {
        pub fn new(player: Player, level: i32) -> Battle {
            let enemy= Enemy::new(level);
            Battle {
                player,
                enemy
            }
        }
        pub fn fight(&mut self) -> bool {
            let mut turn:bool = true;
            if self.player.get_agility() < self.enemy.get_agility() {
                turn = false;
            }

            while self.are_both_alive() {
                let actions:[Action; 3];

                self.print_characters_info();

                if turn {
                    println!("You attack");
                    actions = self.generate_enemy_defence()
                } else {
                    println!("The enemy attacks");
                    actions = self.generate_enemy_attacks()
                }

                let player_actions:[Action; 3];
                player_actions = self.get_player_actions();
                // let mut mismatches = 0;

                for i in 0..=2 as usize {
                    let mut curr_mismatch = 1.0;
                    if actions[i].get_place() == player_actions[i].get_place() {
                        curr_mismatch = curr_mismatch - 0.5;
                        if actions[i].get_side() != player_actions[i].get_side() {
                            curr_mismatch = curr_mismatch - 0.5;
                        }
                    }

                    if turn {
                        let full_damage = (self.player.get_strength() + self.player.get_weapon().get_damage()) as f32;
                        let damage_to_deal = full_damage * curr_mismatch;
                        // println!("place: {}", player_actions[i].get_place());
                        // println!("deal Damage - enemy - part: {}", player_actions[i].get_place() as usize);
                        self.enemy.deal_damage(
                            damage_to_deal as i32
                            ,player_actions[i].get_place() as usize);
                    } else {
                        let full_damage = (self.enemy.get_strength() + self.enemy.get_weapon().get_damage()) as f32;
                        let damage_to_deal = full_damage * curr_mismatch;
                                            // *curr_mismatch;
                        self.player.deal_damage(
                            damage_to_deal as i32
                            ,actions[i].get_place() as usize);
                    }
                }

                println!("Enemy actions: ");
                for action in actions.iter() {
                    println!("place: {}, side {}", action.get_place_string() ,action.get_side_string());
                }

                turn = !turn;
            }

            let mut result = true;
            if self.player.get_vitality() <= 0 {
                result = false;
            }

            // self.player.set_vitality(self.playerVit);

            return result;
        }

        fn print_characters_info(&self) {
            print!("Your health: {}", self.player.get_vitality());
            print!("          ");
            println!("Enemy health: {}", self.enemy.get_vitality());

            print!("weapon: {}", self.player.get_weapon().get_damage());
            print!("          ");
            println!("weapon: {}", self.enemy.get_weapon().get_damage());

            print!("head armor: {}", self.player.get_armor()[0].get_strength());
            print!("          ");
            println!("head armor: {}", self.enemy.get_armor()[0].get_strength());

            print!("body armor: {}", self.player.get_armor()[1].get_strength());
            print!("          ");
            println!("body armor: {}", self.enemy.get_armor()[1].get_strength());

            print!("legs armor: {}", self.player.get_armor()[2].get_strength());
            print!("          ");
            println!("legs armor: {}", self.enemy.get_armor()[2].get_strength());
        }

        fn are_both_alive(&self) -> bool {
            return !self.player.is_dead() && !self.enemy.is_dead()
        }

        fn get_player_actions(&self) -> [Action; 3] {
            println!("Input your actions");
            println!();
            // let mut player_actions:Vec<Action> = Vec::new();

            // for i in 0..=2 {
            //     stdin().read_line(&mut input);
            //     stdin().read_line(&mut input2);
            //     input.remove(input.len() - 1);
            //     input2.remove(input2.len() - 1);
            //
            //     player_actions.push(Action::new(input.clone(), input2.clone()));
            //     // player_actions[i] = ;
            // }4

            //
            // return player_actions.as;

            //action 1
            let mut input = String::new();
            let mut input2 = String::new();
            println!("Input place: (head, body, legs)");
            if stdin().read_line(&mut input).is_err() {
                panic!("Input failed");
            }
            println!("Input side: (right, left)");
            stdin().read_line(&mut input2);
            input.remove(input.len() - 1);
            input2.remove(input2.len() - 1);

            let action1 = Action::new(input.clone(), input2.clone());
            input.clear();
            input2.clear();

            //action 2
            println!("Input place: (head, body, legs)");
            stdin().read_line(&mut input);
            println!("Input side: (right, left)");
            stdin().read_line(&mut input2);
            input.remove(input.len() - 1);
            input2.remove(input2.len() - 1);

            let action2 = Action::new(input.clone(), input2.clone());
            input.clear();
            input2.clear();


            //action 3
            println!("Input place: (head, body, legs)");
            stdin().read_line(&mut input);
            println!("Input side: (right, left)");
            stdin().read_line(&mut input2);
            // println!("Input string: {}", input);
            input.remove(input.len() - 1);
            input2.remove(input2.len() - 1);

            let action3 = Action::new(input.clone(), input2.clone());
            input.clear();
            input2.clear();

            return [
                action1,
                action2,
                action3,
            ]
        }

        fn generate_enemy_attacks(&self) -> [Action; 3] {
            return self.generate_actions();
        }

        fn generate_enemy_defence(&self) -> [Action; 3] {
            return self.generate_actions();
        }

        fn generate_actions(&self) -> [Action; 3] {
            return [
                Action::new(random_place(),random_side()),
                Action::new(random_place(),random_side()),
                Action::new(random_place(),random_side()),
            ]
        }
    }

    fn random_side() -> String {
        let side = rand::thread_rng().gen_range(0..=1);
        // println!("{}",side);
        if side == 0 {
            return String::from("right");
        } else {
            return String::from("left");
        }
    }

    fn random_place() -> String {
        let place = rand::thread_rng().gen_range(0..=2);
        // println!("{}",place);
        if place == 0 {
            return String::from("head");
        } else if place == 1  {
            return String::from("body");
        } else {
            return String::from("legs");
        }
    }
}