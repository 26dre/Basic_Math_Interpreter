// use "Parser.rs";

use std::char;
const CLOSING_PAREN: char = ')';
const OPENING_PAREN: char = '(';
const PLUS: char = '+';
const TIMES: char = '*';

fn main() {
    println!("Hello, world!");
    println!("Lets see if this appears in the git file");

    let x = String::from("Hello bitches");

    let thing = x.as_bytes();

    let x_array: Vec<char> = x.chars().collect();
    // let x_len = x_array.len();

    // parse(&x);
    let mut i = 0;

    while i < x_array.len() {
        println!("{i}: {}", x_array[i]);
        next(&mut i);
    }

    let characters = x.char_indices();
    for c in characters {
        println!("{}, {}", c.0, c.1);
    }

    let j = thing[2];
    println!("{}", j as char);
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

fn parse_perfect_input(s: &String) -> Vec<isize> {
    let ret_vec: Vec<isize> = Vec::new();

    let char_vector: Vec<char> = s.chars().collect();

    ret_vec
}

fn expression_perfect(char_vec: &Vec<char>, mut curr_idx: usize) {
    term_perfect(char_vec, curr_idx);
    while (char_vec[curr_idx] == PLUS) {
        next(&mut curr_idx);
        term_perfect(char_vec, curr_idx);
    }
}

fn term_perfect(char_vec: &Vec<char>, mut curr_idx: usize) {
    factor_perfect(char_vec, curr_idx);
    while (char_vec[curr_idx] == TIMES) {
        next(&mut curr_idx);
        factor_perfect(char_vec, curr_idx);
    }
}
fn factor_perfect(char_vec: &Vec<char>, mut curr_idx: usize) {
    if char_vec[curr_idx].is_numeric() {
        make_number(char_vec, curr_idx);
    } else if char_vec[curr_idx] == OPENING_PAREN {
        next(&mut curr_idx);
        expression_perfect(char_vec, curr_idx);
        match_chars(&char_vec[curr_idx], &CLOSING_PAREN);
        next(&mut curr_idx);
        //at this point the function should properly return the value that it has received
        // I will implement this functionality later
    } else {
        panic!("Invalid input m8");
    }
}

fn make_number(char_vec: &Vec<char>, mut curr_idx: usize) -> usize {
    let mut ret_num = 0;
    while char_vec[curr_idx].is_numeric() {
        ret_num *= 10;
        ret_num += char_vec[curr_idx]
            .to_digit(10)
            .expect("Value out of bounds");
        next(&mut curr_idx);
    }

    ret_num as usize
}
