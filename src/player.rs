pub mod player {
    use crate::armor::armor::Armor;
    use crate::weapon::weapon::Weapon;

    #[derive(Clone)]
    pub struct Player {
        level: i32,
        skill_points: i32,
        gold:i32,

        strength: i32,
        agility: i32,
        vitality: i32,

        weapon: Weapon,
        armor:[Armor; 3]
    }

    impl Player {
        pub fn new() -> Player {
            Player {
                level: 0,
                skill_points: 0,
                gold: 0,

                strength: 1,
                agility: 1,
                vitality: 30,

                weapon: Weapon::new(0),
                armor: [Armor::new(0),
                    Armor::new(0),
                    Armor::new(0)]
            }
        }

        pub fn get_level(&self) -> i32 {
            return self.level;
        }

        pub fn get_gold(&self) -> i32 {
            return self.gold;
        }

        pub fn get_skill_points(&self) -> i32 {
            return self.skill_points;
        }

        pub fn get_strength(&self) -> i32 {
            return self.strength;
        }

        pub fn get_agility(&self) -> i32 {
            return self.agility;
        }

        pub fn get_vitality(&self) -> i32 {
            return self.vitality;
        }

        pub fn get_armor(&self) -> &[Armor;3] {
            return &self.armor;
        }

        pub fn get_weapon(&self) -> &Weapon {
            return &self.weapon;
        }

        pub fn set_weapon(&mut self, new_weapon: Weapon) {
            self.weapon = new_weapon;
        }

        pub fn increase_level(&mut self) {
            self.level = self.level + 1;
        }

        pub fn increase_strength(&mut self) {
            self.strength = self.strength + 1;
        }

        pub fn increase_agility(&mut self) {
            self.agility = self.agility + 1;
        }

        // pub fn set_vitality(&mut self, vitality: i32) {
        //     return self.vitality = vitality;
        // }

        pub fn increase_vitality(&mut self) {
            self.vitality = self.vitality + 1;
        }

        pub fn add_gold(&mut self, gold: i32) {
            self.gold = self.gold + gold;
        }

        pub fn spend_gold(&mut self, gold: i32) {
            self.gold = self.gold - gold;
        }

        pub fn add_skillpoints(&mut self, points: i32) {
             self.skill_points = self.skill_points + points;
        }

        pub fn spend_skillpoint(&mut self) {
            self.skill_points = self.skill_points - 1;
        }

        pub fn set_armor(&mut self, place: usize, new_armor: Armor) {
            return self.armor[place] = new_armor;
        }

        pub fn deal_damage(&mut self, damage:i32, place:usize) {
            let new_damage = damage - self.armor[place].get_strength();

            if self.armor[place].get_strength() != 0{
                self.armor[place].deal_damage(damage);

                if self.armor[place].get_strength() < 0 {
                    self.armor[place].set_strength(0);
                }
            }

            self.vitality = self.vitality - new_damage;
        }

        pub fn is_dead(&self) -> bool {
            return if self.vitality <= 0 {
                true
            } else {
                false
            }
        }

    }

    // fn init_skill(level: i32, diff: i32) -> i32 {
    //     let mut skill:i32 = rand::thread_rng().gen_range(level - diff .. level + diff);
    //     if skill <= 0 {
    //         skill = 1;
    //     }
    //
    //     return skill;
    // }
}