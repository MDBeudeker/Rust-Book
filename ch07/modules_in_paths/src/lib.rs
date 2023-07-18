mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();  // eat_at_restaurant can only directly access its sibling, front of house
                                                        // in order to access the sub items of front of house, we need to make the module 'hosting' public, as well as the function add_to_waitlist
    
    //relative path
    front_of_house::hosting::add_to_waitlist();

    //Order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //change our mind about the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);
    
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");


    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


fn deliver_order() {}

mod back_of_house{
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast{ // the struct is made public, but we still need to decide the scope of its contents implicitly
        pub toast: String, // anyone can see toast on the menu
        seasonal_fruit: String, //only the Chef can see this
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

//using 'super' = '..'