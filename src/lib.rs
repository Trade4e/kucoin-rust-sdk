pub mod utils;
pub mod client;

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
