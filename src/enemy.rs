pub mod enemy {
    use rand::Rng;
    use crate::armor::armor::Armor;
    use crate::weapon::weapon::Weapon;

    pub struct Enemy {
        strength: i32,
        agility: i32,
        vitality: i32,
        weapon: Weapon,
        armor:[Armor; 3]
    }

    impl Enemy {
        pub fn new(level: i32) -> Enemy {
            let new_strength:i32 = init_skill(level, 2);
            let new_agility:i32 = init_skill(level, 2);
            let mut new_vitality:i32 = 30 + init_skill(level, 2);
            if new_vitality < 30 {
                new_vitality = 30;
            }

            Enemy {
                strength: new_strength,
                agility: new_agility,
                vitality: new_vitality,
                weapon: Weapon::new(level),
                armor: [Armor::new(level),
                        Armor::new(level),
                        Armor::new(level)]
            }
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

        pub fn get_weapon(&self) -> &Weapon {
            return &self.weapon;
        }

        pub fn get_armor(&self) -> &[Armor;3] {
            return &self.armor;
        }

        pub fn deal_damage(&mut self, damage:i32, place: usize) {
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

    fn init_skill(level: i32, diff: i32) -> i32 {
        let mut skill:i32 = rand::thread_rng().gen_range((level - diff)..(level + diff));
        if skill <= 0 {
            skill = 1;
        }

        return skill;
    }
}