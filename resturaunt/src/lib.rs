// Modules
// Parent modules can't see into child modules, but child modules can see parent modules.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    mod serving {
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

pub fn eat_at_resturaunt()
{
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}