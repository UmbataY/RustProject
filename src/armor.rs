pub mod armor{
    use rand::Rng;

    #[derive(Clone)]
    pub struct Armor {
        strength: i32,
        price: i32
    }

    impl Armor {
        pub fn new(level: i32) -> Armor {
            let mut str: i32 = rand::thread_rng().gen_range(level - 2 .. level + 2);
            if str < 0 {
                str = 0;
            }
            Armor{
                strength: str,
                price: 100
            }
        }

        pub fn copyOf(other: &Armor) -> Armor {
            Armor{
                strength: other.strength.clone(),
                price: other.price.clone()
            }
        }

        pub fn get_strength(&self) -> i32 {
            self.strength
        }
        pub fn get_price(&self) -> i32 {
            self.price
        }
        pub fn set_strength(&mut self, strength: i32) {
            self.strength = strength;
        }

        pub fn deal_damage(&mut self, damage:i32) {
            self.strength = self.strength - damage;
        }
    }
}