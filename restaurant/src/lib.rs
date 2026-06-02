mod back_of_the_house;
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_the_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

}

use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting;
    pub fn throw_customer() {
        hosting::add_to_waitlist();
    }
}

mod second_customer {

    pub fn throw_customer() {
        super::hosting::add_to_waitlist(); //used super to access crate
        
        let mut map = super::HashMap::new();
        map.insert(1, 2); 
    }
}

pub fn eat_at_restaurant() {
        //absolute path
        hosting::add_to_waitlist();

        //relative path
        hosting::add_to_waitlist();

        let mut meal = back_of_the_house::back_of_the_house::Breakfast::summer("Rye");

        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast);

        //meal.seasonal_fruit = String::from("blueberries");

        let order1 = back_of_the_house::back_of_the_house::Appetizer::Soup;
        let order2 = back_of_the_house::back_of_the_house::Appetizer::Salad;
}


