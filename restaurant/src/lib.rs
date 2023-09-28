use crate::front_of_house::hosting;
mod front_of_house;

pub mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;
}