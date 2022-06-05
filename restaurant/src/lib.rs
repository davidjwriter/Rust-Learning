#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                super::server_order();
            }

            fn cook_order() {}
        }

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaces"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This would not compile
    // meal.seasonal_fruit = String::from("blueberries");
}
