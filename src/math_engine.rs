use crate::ast::{Parser, evaluate, evaluate_with_context};
use std::collections::HashMap;
use lazy_static::lazy_static;

// ======= Number word conversion functions =======
lazy_static!{
    static ref NUMBER_WORDS: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        m.insert("zero", 0.0);
        m.insert("one", 1.0);
        m.insert("two", 2.0);
        m.insert("three", 3.0);
        m.insert("four", 4.0);
        m.insert("five", 5.0);
        m.insert("six", 6.0);
        m.insert("seven", 7.0);
        m.insert("eight", 8.0);
        m.insert("nine", 9.0);

        m.insert("ten", 10.0);
        m.insert("eleven", 11.0);
        m.insert("twelve", 12.0);
        m.insert("thirteen", 13.0);
        m.insert("fourteen", 14.0);
        m.insert("fifteen", 15.0);
        m.insert("sixteen", 16.0);
        m.insert("seventeen", 17.0);
        m.insert("eighteen", 18.0);
        m.insert("nineteen", 19.0);

        m.insert("twenty", 20.0);
        m.insert("thirty", 30.0);
        m.insert("forty", 40.0);
        m.insert("fifty", 50.0);
        m.insert("sixty", 60.0);
        m.insert("seventy", 70.0);
        m.insert("eighty", 80.0);
        m.insert("ninety", 90.0);

        m.insert("hundred", 100.0);
        m.insert("thousand", 1_000.0);
        m.insert("million", 1_000_000.0);
        m.insert("billion", 1_000_000_000.0);

        m
    };
}

pub fn word_value(word: &str) -> Option<f64>{
    NUMBER_WORDS.get(word).copied() // we use the HashMap to get the value of the word, and return None if the word is not found
}

pub fn word_to_number(word: &str) -> Option<f64> {
    let mut total = 0.0;
    let mut current = 0.0;

    for part in word.split_whitespace() {
        if let Some(value) = word_value(part) {
            if value == 100.0 {
                if current == 0.0 { current = 1.0; } // "hundred" alone
                current *= 100.0;
            } else if value >= 1000.0 {
                if current == 0.0 { current = 1.0; } // "thousand" alone
                total += current * value;
                current = 0.0;
            } else {
                current += value;
            }
        } else {
            return None;
        }
    }

    total += current;
    Some(total)
}

pub fn is_number_word(word: &str) -> bool {
    word_value(word).is_some() // returns true if the word is a valid number word, false otherwise
}

// ========= End of number word conversion functions =======

// ========= Normaliser =======
pub fn normalise(text: &str) -> String {
    let mut result = text.to_string();

    // normalize english phrases
    result = result.replace("to the power of", "^");
    result = result.replace("to power of", "^");
    // add spaces around operators
    for op in ["*", "/", "^", "(", ")"] {
        result = result.replace(op, &format!(" {} ", op));
    }

    // collapse duplicate spaces
    result = result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    result
}

// ========= Main compute function and text analyser for debugging =======

// Inner function to compute the result and also return the AST for debugging purposes. This is used by both the main text_analyser and the compute function (we ignore the ast formation in tests/test results))
fn compute_inner(text: &str) -> (Option<crate::ast::Expression>, Result<f64, String>) {
    let normalised_text = &normalise(text);
    let tokens = crate::ast::tokenize(normalised_text);
    let mut parser = Parser::new(tokens);
    match parser.parse_expression() {
        Some(ast) => {
            let result = evaluate(&ast);
            (Some(ast), result)
        }
        None => (None, Err("Could not parse expression".to_string())),
    }
}

#[cfg(test)]
// This is the main compute function that will be used in tests directory. It just returns the result and ignores the AST.
pub fn compute(text: &str) -> Result<f64, String> {
    // compute_with_context(text, &mut HashMap::new())
    let (_, result) = compute_inner(text); // we ignore the AST and just return the result
    result
}


pub type VarContext = HashMap<String, f64>;

// for main.rs shwoing the whole AST and tokens. Compute is made for test cases where we just want the result and not the AST or tokens.
pub fn text_analyser(text: &str) { 
    let normalised_text = &normalise(text);
    println!("Tokens: {:?}", crate::ast::tokenize(normalised_text));
    let (ast, result) = compute_inner(text);
    if let Some(ast) = ast {
        println!("AST: {:?}", ast);
    }
    match result {
        Ok(r) => {
            if r.fract() == 0.0 { println!("Result: {}", r); }
            else { println!("Result: {:.4}", r); }
        }
        Err(e) => println!("Error: {}", e),
    }
}