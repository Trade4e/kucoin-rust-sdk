
pub extern crate serde;
pub extern crate serde_json;

#[macro_use]
pub extern crate serde_derive;

#[macro_use]
pub extern crate thiserror;

pub mod error;
pub mod utils;
pub mod client;
pub mod market;
pub mod others;

#[cfg(test)]
mod tests {
    use crate::utils::get_time_as_millis;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn time() {
        let now = get_time_as_millis();
        println!("Time {:?}", now);
    }
}
