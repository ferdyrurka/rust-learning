/**
* Example test
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_failed() {
        // failed validate
        // panic!("Make this test fail");
        assert!(true);
    }
}