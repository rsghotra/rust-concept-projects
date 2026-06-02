#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast { 
            toast: String::from(toast), 
            seasonal_fruit: String::from("Blie"), 
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}

pub fn cook_order() {
    
}