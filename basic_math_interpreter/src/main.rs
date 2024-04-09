// use "Parser.rs";

use std::char;
const CLOSING_PAREN: char = ')';
const OPENING_PAREN: char = '(';
const PLUS: char = '+';
const TIMES: char = '*';
const DIVIDE: char = '/';
const MINUS: char = '-';
const PERIOD: char = '.';


fn main() {

    let input_string = String::from("10+20*(10+3)");
    let ret_val = parse_perfect_input(&input_string);
    println!("{}", ret_val[0]);

}

fn next(curr_idx: &mut usize) {
    *curr_idx += 1;
}


fn match_chars(curr_char: &char, char_to_match: &char) {
    if curr_char != char_to_match {
        panic!("Syntax incorrect, this output is unreadable")
    }
}

fn parse_perfect_input(s: &String) -> Vec<isize> {
    let mut ret_vec: Vec<isize> = Vec::new();

    let char_vector: Vec<char> = s.chars().collect();
    let mut curr_idx = 0;
    let ret_val = expression_perfect(&char_vector, &mut curr_idx);
    println!("{ret_val}");
    while char_vector.len() > curr_idx && char_vector[curr_idx] == PERIOD {
        next(&mut curr_idx);
        expression_perfect(&char_vector, &mut curr_idx);
    }

    ret_vec.push(ret_val);
    ret_vec
}

fn expression_perfect(char_vec: &Vec<char>, curr_idx: &mut usize) -> isize {
    let mut val: isize = term_perfect(char_vec, curr_idx) as isize;
    while *curr_idx < char_vec.len()
        && char_vec[*curr_idx] != PERIOD
        && (char_vec[*curr_idx] == PLUS
            || char_vec[*curr_idx] == MINUS
            || char_vec[*curr_idx].is_whitespace())
    {
        println!("Inside while ctrl flow with curr_idx val of {}", *curr_idx);

        // val += term_perfect(char_vec, curr_idx);
        if char_vec[*curr_idx] == PLUS {
            next(curr_idx);
            val += term_perfect(char_vec, curr_idx) as isize;
        } else if char_vec[*curr_idx] == MINUS {
            next(curr_idx);
            val -= term_perfect(char_vec, curr_idx) as isize;
        } else {
            // panic!("TF bro! Bad input prob white space or smth fr fr idk how u ended up here");
            next(curr_idx);
        }
    }

    val
}

fn term_perfect(char_vec: &Vec<char>, curr_idx: &mut usize) -> usize {
    let mut val: usize = factor_perfect(char_vec, curr_idx);
    while *curr_idx < char_vec.len()
        && (char_vec[*curr_idx] == TIMES
            || char_vec[*curr_idx] == DIVIDE
            || char_vec[*curr_idx].is_whitespace())
    {
        if char_vec[*curr_idx] == TIMES {
            next(curr_idx);
            val *= factor_perfect(char_vec, curr_idx);
        } else if char_vec[*curr_idx] == DIVIDE {
            next(curr_idx);
            val /= factor_perfect(char_vec, curr_idx)
        } else {
            next(curr_idx);
        }
    }

    val
}

fn factor_perfect(char_vec: &Vec<char>, curr_idx: &mut usize) -> usize {
    while char_vec.len() > *curr_idx && char_vec[*curr_idx].is_whitespace() {
        next(curr_idx);
    }
    if char_vec[*curr_idx].is_numeric() {
        make_number(char_vec, curr_idx)
    } else if char_vec[*curr_idx] == OPENING_PAREN {
        next(curr_idx);
        let temp = expression_perfect(char_vec, curr_idx);
        while char_vec.len() > *curr_idx && char_vec[*curr_idx].is_whitespace() {
            next(curr_idx);
        }

        // if (*curr_idx)
        match_chars(&char_vec[*curr_idx], &CLOSING_PAREN);
        temp.try_into().unwrap()
        // next(curr_idx);
        //at this point the function should properly return the value that it has received
        // I will implement this functionality later
    } else {
        panic!("Invalid input m8");
    }
}

fn make_number(char_vec: &Vec<char>, curr_idx: &mut usize) -> usize {
    let mut ret_num = 0;
    while *curr_idx < char_vec.len() && char_vec[*curr_idx].is_numeric() {
        ret_num *= 10;
        ret_num += char_vec[*curr_idx]
            .to_digit(10)
            .expect("Value out of bounds");
        next(curr_idx);
    }

    println!("About to return the number {ret_num}");
    ret_num as usize
}
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
        let res_2 = parse_perfect_input(&test_str)[1];
        //this line ends up being out of bounds so gotta fix that shit
        assert_eq!(result, 129);
        assert_eq!(res_2, 22);
    }
}
