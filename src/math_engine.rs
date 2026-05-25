use std::result;

use crate::ast::{Parser, evaluate};

// I will be using f64 for precision and to handl some future tests withfloating point i have in mind.
fn add(numbers: &[f64]) -> f64 {
    numbers.iter().map(|&n| n).sum()
}

fn subtract(numbers: &[f64]) -> f64 {
    // subtraction operation
    let first = numbers[0];
    numbers[1..].iter().fold(first, |acc, &n| acc - n)
}

fn multiply(numbers: &[f64]) -> f64 {
    numbers.iter().product() 
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

// ======= Number word conversion functions =======
pub fn word_value(word: &str) -> Option<f64>{
  match word {
        "zero" => Some(0.0),
        "one" => Some(1.0),
        "two" => Some(2.0),
        "three" => Some(3.0),
        "four" => Some(4.0),
        "five" => Some(5.0),
        "six" => Some(6.0),
        "seven" => Some(7.0),
        "eight" => Some(8.0),
        "nine" => Some(9.0),

        "ten" => Some(10.0),
        "eleven" => Some(11.0),
        "twelve" => Some(12.0),
        "thirteen" => Some(13.0),
        "fourteen" => Some(14.0),
        "fifteen" => Some(15.0),
        "sixteen" => Some(16.0),
        "seventeen" => Some(17.0),
        "eighteen" => Some(18.0),
        "nineteen" => Some(19.0),

        "twenty" => Some(20.0),
        "thirty" => Some(30.0),
        "forty" => Some(40.0),
        "fifty" => Some(50.0),
        "sixty" => Some(60.0),
        "seventy" => Some(70.0),
        "eighty" => Some(80.0),
        "ninety" => Some(90.0),

        "hundred" => Some(100.0),
        "thousand" => Some(1_000.0),
        "million" => Some(1_000_000.0),
        "billion" => Some(1_000_000_000.0),

        _ => None,
  }
}

pub fn word_to_number(word: &str) -> Option<f64> {
    let mut total = 0.0;
    let mut current = 0.0;

    for part in word.split_whitespace() {
        if let Some(value) = word_value(part)  {
            if value == 100.0 {
                current *= 100.0;
            } else if value >= 1000.0 {
                total += current * value;
                current = 0.0;
            } else {
                current += value;
            }
        } else {
            return None; // Return None if any part is not a valid number word
        }
    }

    total += current;
    Some(total) as Option<f64>
}

pub fn is_number_word(word: &str) -> bool {
    word_value(word).is_some() // returns true if the word is a valid number word, false otherwise
}

// ========= End of number word conversion functions =======

pub fn text_analyser(text : &str){
    let tokens = crate::ast::tokenize(text);
    println!("Tokens: {:?}", tokens);
    let mut parser = Parser::new(tokens);
    match parser.parse_expression(){
        Some(ast) => {
            println!("AST: {:?}", ast);
            match evaluate(&ast){
                Ok(result) => {
                    if result.fract() == 0.0 {
                        println!("Result: {}", result);
                    } else {
                        println!("Result: {:.4}", result);
                    }
                } Err(e) => println!("Error: {}", e),
            }
        }
        None => println!("Could not parse expression"),
    }
}