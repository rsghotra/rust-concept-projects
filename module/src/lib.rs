
mod front_of_house;
mod back_of_house;

use crate::front_of_house::hosting;
use crate::front_of_house::serving;


pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_the_table();
    serving::take_order();
    let order1 = back_of_house::Breakfast::summer("Wheat");
    let order2 = back_of_house::Appetizer::Salad;
    let order3 = back_of_house::Appetizer::Soup;
    
    back_of_house::cook_order();
    serving::serve_order();
    serving::take_payment();

    println!("{:?}", order1.toast);


}
