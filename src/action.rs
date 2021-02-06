// pub mod action {
    pub struct Action {
        place: i32,
        side: i32
    }

    impl Action {
        pub fn new(side: String, place: String) -> Action {
            Action {
                place: make_actual_place(place),
                side: make_actual_side(side),
            }
        }

        pub fn get_side(&self) -> i32 {
            return self.side;
        }

        pub fn get_place(&self) -> i32 {
            return self.place;
        }
    }

    fn make_actual_place(place: String) -> i32{
        match place.as_str() {
            "head" => 0,
            "body" => 1,
            "legs" => 2,
            _ => 0
        }
    }

    fn make_actual_side(side: String) -> i32{
        match side.as_str() {
            "right" => 0,
            "left" => 1,
            _ => 0
        }
    }
// }