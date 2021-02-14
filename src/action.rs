// pub mod action {
    pub struct Action {
        place: i32,
        side: i32
    }

    impl Action {
        pub fn new(place: String, side: String) -> Action {
            Action {
                place: make_actual_place(place),
                side: make_actual_side(side),
            }
        }

        pub fn get_side(&self) -> i32 {
            return self.side;
        }

        pub fn get_side_string(&self) -> String {
            match self.place {
                0 => String::from("right"),
                1 => String::from("left"),
                _ => String::from("right")
            }
        }

        pub fn get_place(&self) -> i32 {
            return self.place;
        }

        pub fn get_place_string(&self) -> String {
            match self.side {
                0 => String::from("head"),
                1 => String::from("body"),
                2 => String::from("legs"),
                _ => String::from("head")
            }
        }
    }

    fn make_actual_place(place: String) -> i32{
        match place.as_str() {
            "head" => 0,
            "body" => 1,
            "legs" => 2,
            _ => {
                // println!("No such Place");
                0
            }
        }
    }

    fn make_actual_side(side: String) -> i32{
        match side.as_str() {
            "right" => 0,
            "left" => 1,
            _ => {
                // println!("No such Side");
                0
            }
        }
    }
// }