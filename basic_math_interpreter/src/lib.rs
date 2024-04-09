
mod Parser;
use Parser::parse_perfect_input;
#[cfg(test)]
mod tests {
    // use std::result;

    use crate::parse_perfect_input;

    #[test]
    fn works() {
        let test_str = String::from("123+124");
        let result = parse_perfect_input(&test_str)[0];
        // println!("Running this test case");
        assert_eq!(result, 247);
    }
    #[test]
    fn multiplication_test() {
        let test_str = String::from("10*25");
        let result = parse_perfect_input(&test_str)[0];
        // println!("Running")
        assert_eq!(result, 250)
    }

    #[test]
    fn add_and_multiply() {
        let test_str = String::from("10+25*4");
        let result = parse_perfect_input(&test_str)[0];
        assert_eq!(result, 110);
    }
    #[test]
    fn add_and_multiply_and_paren() {
        let test_str = String::from("10+25*(10+2+(4*4))");
        let result = parse_perfect_input(&test_str)[0];
        assert_eq!(result, 710);
    }
    #[test]
    fn minus_perfect_test() {
        let test_str = String::from("25-10");
        let result = parse_perfect_input(&test_str)[0];
        assert_eq!(result, 15);
    }

    #[test]
    fn divide_perfect_test() {
        let test_str = String::from("150/10");
        let result = parse_perfect_input(&test_str)[0];
        assert_eq!(result, 15);
    }

    #[test]
    fn negative_number_result() {
        let test_str = String::from("25-100");
        let result = parse_perfect_input(&test_str)[0];
        assert_eq!(result, -75);
    }
    #[test]
    fn imperfect_input_addition_sub() {
        let test_str = String::from("25    - 100");
        let result = parse_perfect_input(&test_str)[0];
        assert_eq!(result, -75);
    } // fn works2
    #[test]
    fn imperfect_multiplication() {
        let test_str = String::from("25    * 100");
        let result = parse_perfect_input(&test_str)[0];
        assert_eq!(result, 2500);
    } // fn works2
    #[test]
    fn imperfect_parentheses() {
        let test_str = String::from("25    + 100+ (       2*2     )");
        let result = parse_perfect_input(&test_str)[0];
        assert_eq!(result, 129);
    }
    #[test]
    fn extra_shit() {
        let test_str = String::from("25    + 100+ (       2*2     ). 22");
        let result = parse_perfect_input(&test_str)[0];
        // let res_2 = parse_perfect_input(&test_str)[1];
        //this line ends up being out of bounds so gotta fix that shit
        assert_eq!(result, 129);
        // assert_eq!(res_2, 22);
    }
}
