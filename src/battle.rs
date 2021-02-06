pub mod battle {
    use crate::enemy::enemy::Enemy;
    use crate::player::player::Player;
    use crate::action::Action;
    use rand::Rng;
    use std::io::stdin;

    pub struct Battle {
        player: Player,
        enemy: Enemy,
        playerVit: i32,
        enemyVit: i32
    }

    impl Battle {
        pub fn new(player: Player, level: i32) -> Battle {
            let enemy= Enemy::new(level);
            let enemyVitality = enemy.get_vitality();
            let playerVitality = player.get_vitality();
            Battle {
                player: player,
                enemy: enemy,
                playerVit: playerVitality,
                enemyVit: enemyVitality,
            }
        }
        pub fn fight(&mut self) -> bool {
            let mut turn:bool = true;
            if self.player.get_agility() < self.enemy.get_agility() {
                turn = false;
            }

            while self.are_both_alive() {
                let actions:[Action; 3];

                println!("Your health: {}", self.player.get_vitality());
                println!("Your health: {}", self.enemy.get_vitality());

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
                        if actions[i].get_side() == player_actions[i].get_side() {
                            curr_mismatch = curr_mismatch - 0.5;
                        }
                    }

                    if turn {
                        self.enemy.deal_damage(
                            self.player.get_strength() + self.player.get_weapon().get_damage()
                            ,player_actions[i].get_place() as usize);
                    } else {
                        self.player.deal_damage(
                            self.enemy.get_strength() + self.enemy.get_weapon().get_damage()
                            ,actions[i].get_place() as usize);
                    }
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

        fn are_both_alive(&self) -> bool {
            return self.player.get_vitality() > 0 && self.enemy.get_vitality() > 0
        }

        fn get_player_actions(&self) -> [Action; 3] {
            let mut input = String::new();
            let mut input2 = String::new();
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
            // }
            //
            // return player_actions.as;

            //action 1
            let mut input = String::new();
            let mut input2 = String::new();
            println!("Input place: (head, body, legs)");
            stdin().read_line(&mut input);
            println!("Input side: (right, left)");
            stdin().read_line(&mut input2);
            input.remove(input.len() - 1);
            input2.remove(input2.len() - 1);

            let action1 = Action::new(input.clone(), input2.clone());

            //action 2
            println!("Input place: (head, body, legs)");
            stdin().read_line(&mut input);
            println!("Input side: (right, left)");
            stdin().read_line(&mut input2);
            input.remove(input.len() - 1);
            input2.remove(input2.len() - 1);
            let action2 = Action::new(input.clone(), input2.clone());

            //action 3
            println!("Input place: (head, body, legs)");
            stdin().read_line(&mut input);
            println!("Input side: (right, left)");
            stdin().read_line(&mut input2);
            input.remove(input.len() - 1);
            input2.remove(input2.len() - 1);
            let action3 = Action::new(input.clone(), input2.clone());

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
                Action::new(random_side(),random_place()),
                Action::new(random_side(),random_place()),
                Action::new(random_side(),random_place()),
            ]
        }
    }

    fn random_side() -> String {
        let side = rand::thread_rng().gen_range(0..=1);
        if side == 0 {
            return String::from("right");
        } else {
            return String::from("left");
        }
    }

    fn random_place() -> String {
        let side = rand::thread_rng().gen_range(0..=2);
        if side == 0 {
            return String::from("head");
        } else if side == 1  {
            return String::from("body");
        } else {
            return String::from("legs");
        }
    }
}