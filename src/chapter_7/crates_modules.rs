mod front_of_house;
use crate::crates_modules::front_of_house::hosting;

// use self::front_of_house::hosting; // relative path; for be deprecated

pub fn eat_at_restaurant() {
    // crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist(); // because the use (import part)
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("from complex chapter_7 {:#?}", meal);
}

fn serve_order() {}
mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrent_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
