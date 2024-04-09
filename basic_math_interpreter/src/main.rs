// use "Parser.rs";

use std::char;
const CLOSING_PAREN: char = ')';
const OPENING_PAREN: char = '(';
const PLUS: char = '+';
const TIMES: char = '*';
const DIVIDE: char = '/';
const MINUS: char = '-';

static mut ACCUMULATOR: usize = 0;

fn main() {
    // let should_fail = String::from("Hello world");
    // let should_work = String::from("hhhh123j");
    // let should_work_array : Vec<char> = should_work.chars().collect();
    // let idx = 4;
    // let x = make_number(&should_work_array, idx);
    // println!("{x}");

    let input_string = String::from("10+20*(10+3)");
    let ret_val = parse_perfect_input(&input_string);
    println!("{}", ret_val[0]);
    // let mut x = 100;
    // next(&mut x);

    // unsafe{
    //     ACCUMULATOR += 10;
    //     print!("{ACCUMULATOR}");
    // }

    //    should_work: Vec<char> = should_work.chars().collect();
}

// fn parse(s: &String, idx: usize) -> Option<Vec<i32>>{
//     let mut result_vector:Option<Vec<i32>> = None;
//     let mut

//     while idx< s.len(){
//         if
//     }

//     result_vector

// }

// fn expression(s:&String, mut idx: usize){

// // }
// fn expression(){

// }

// fn parse(input_str: &String) -> Vec<isize> {
//     let ret_vec: Vec<isize> = Vec::new();
//     // let str_vec = input_str.chars().collect();
//     let str_vec: Vec<char> = input_str.chars().collect();

//     for character in str_vec {
//         if (!character.is_whitespace()){
//             expression()
//         }
//     }

//     ret_vec
// }

fn next(curr_idx: &mut usize) {
    *curr_idx += 1;
}

fn handle_whitespace_not_period(curr_idx: &mut usize) {
    next(curr_idx);
}

// fn handle_whitespace_period(curr_idx: &mut usize) -> bool

// fn check_and_handle_whitespace(char_vector: Vec<char>, curr_idx: &mut usize) {
//     // let curr_char = char_vector.get(*curr_idx);
//     let curr_char = char_vector[*curr_idx];
//     if curr_char.is_whitespace() {
//         if curr_char == '.' {
//             next(curr_idx);
//         } else {
//             handle_whitespace_not_period(curr_idx)
//         }
//     }
// }

fn match_chars(curr_char: &char, char_to_match: &char) {
    if curr_char != char_to_match {
        panic!("Syntax incorrect, this output is unreadable")
    }
}

fn parse_perfect_input(s: &String) -> Vec<usize> {
    let mut ret_vec: Vec<usize> = Vec::new();

    let char_vector: Vec<char> = s.chars().collect();
    let mut curr_idx = 0;
    let ret_val = expression_perfect(&char_vector, &mut curr_idx);

    ret_vec.push(ret_val);
    ret_vec
}

fn expression_perfect(char_vec: &Vec<char>, curr_idx: &mut usize) -> usize {
    let mut val: usize = term_perfect(char_vec, curr_idx);
    while *curr_idx < char_vec.len() && char_vec[*curr_idx] == PLUS {
        println!("Inside while ctrl flow with curr_idx val of {}", *curr_idx);
        next(curr_idx);
        val += term_perfect(char_vec, curr_idx);
    }

    val
}

fn term_perfect(char_vec: &Vec<char>, curr_idx: &mut usize) -> usize {
    let mut val: usize = factor_perfect(char_vec, curr_idx);
    while *curr_idx < char_vec.len() && char_vec[*curr_idx] == TIMES {
        next(curr_idx);
        val *= factor_perfect(char_vec, curr_idx);
    }

    val
}

fn factor_perfect(char_vec: &Vec<char>, curr_idx: &mut usize) -> usize {
    if char_vec[*curr_idx].is_numeric() {
        make_number(char_vec, curr_idx)
    } else if char_vec[*curr_idx] == OPENING_PAREN {
        next(curr_idx);
        let temp = expression_perfect(char_vec, curr_idx);
        match_chars(&char_vec[*curr_idx], &CLOSING_PAREN);
        temp
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

mod tests {
    use std::result;

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

    // fn works2
}
