// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

// fn seat_at_table() {}
//     }
// }

// fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    // pub struct Breakfast {
    //     pub toast: String,
    //     seasonal_fruit: String,
    // }

    // impl Breakfast {

    //     pub fn summer(toast: &str) -> Breakfast {
    //         Breakfast {
    //             toast: String::from(toast),
    //             seasonal_fruit: String::from("peaches"),
    //         }
    //     }
    // }
    // fn fix_incorrect_order() {
    //     cook_order();
    //     super::deliver_order();
    // }

    // fn cook_order() {}
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // let mut meal = back_of_house::Breakfast::summer("Rye");
    // meal.toast = String::from("Wheat");
    // println!("I'd like {} from toast please", meal.toast);
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();
}

// mod serving {
//     fn take_order() {}

//     fn serve_order() {}

//     fn take_payment() {}
// }
