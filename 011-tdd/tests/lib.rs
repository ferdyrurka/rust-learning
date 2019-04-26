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

    #[test]
    #[should_panic(excepted = "Make this test fail")]
    fn it_should_panic() {
        // failed test by:
        // panic!("failed message");
        panic!("Make this test fail");
    }
}