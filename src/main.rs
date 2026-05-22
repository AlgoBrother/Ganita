
mod math_engine;
use math_engine::{text_analyser, word_to_number};

fn main(){
    let text = [
        "Add 5 and 10",
        "Add -15 and 10",
        "Subtract 20 from 50",
        "Multiply 4 by 6",
        "Divide 100 by 5",
        "Divide 10 by 0", // This will test division by zero handling
        "Add five hundred and twenty three and one hundred and seventy seven", // This will test multi-word number handling
        "Subtract one million from five million", // This will test larger numbers
        "Multiply twenty by thirty", // This will test multi-word numbers without "and"
        "Divide one hundred by twenty five", // This will test multi-word numbers with "by"
        "divide 20 by 40", // This will test missing operation handling
    ];

     for text in text {
         println!("Analyzing: {}", text);
         text_analyser(text);
         println!("-----------------------------");
     }

    println!("Result: {}", word_to_number("five million three thousand nine hundred ninety").unwrap_or_else(|| {
        println!("Failed to convert word to number");
        0
    }));
}