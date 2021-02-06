pub mod weapon{
    use rand::Rng;

    #[derive(Clone)]
    pub struct Weapon {
        name: String,
        damage: i32,
        price: i32
    }

    impl Weapon {
        pub fn new(level: i32) -> Weapon {
            let mut dam: i32 = rand::thread_rng().gen_range(level - 2 .. level + 2);
            if dam <= 0 {
                dam = 1;
            }

            Weapon{
                name: String::from(""),
                damage: dam,
                price: 100
            }
        }

        pub fn copyOf(other: &Weapon) -> Weapon {
            Weapon{
                name: other.name.clone(),
                damage: other.damage.clone(),
                price: other.price.clone()
            }
        }

        pub fn get_damage(&self) -> i32 {
            self.damage
        }
        pub fn get_price(&self) -> i32 {
            self.price
        }
    }
}