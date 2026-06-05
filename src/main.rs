mod ast;
mod math_engine;
mod tests; 
mod trignometry;
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
        "Add 5 10 and 76 and -20", // This will test multiple numbers handling
        "Add 5 and 10 subtract 3 and 2", // This will test multiple operations in one line

        "Add 1 2 3 4 5",
        "Subtract 100 20 5 5",
        "Multiply 2 3 4 5",
        "Divide 100 2 5",

        "Add zero and zero",
        "Multiply 0 by 999",
        "Divide 0 by 10",

        "Subtract -10 from -50",
        "Multiply -5 by -10",
        "Multiply -5 by 10",

        "Add one hundred twenty three and four hundred fifty six",
        "Add twenty one and thirty two",
        "Subtract one thousand from ten thousand",

        "Add 5.5 and 2.5",
        "Divide 5 by 2",

        "Add five hundred thousand and two million",
        "Subtract one billion from two billion",

        "Add five and apples",
        "Multiply hello by world",

        "Divide 999999999 by 3",
        "Multiply 99999 by 99999",

        "Add five hundred and twenty three and seven",
        "Subtract fifty from twenty",

        "Add 5 and -10 and 15 and -20",
        "Multiply 1 1 1 1 1 1",
        "2 + 5 * 3",
        "10 + 20 - 5 * 3 / 2",
        "1 + 2",
        "Multiply 4 by result of 7 * 7",

        "Divide 1 by 3",
        "Divide 1000 by 0.5",
        "Add six to the result of subtracting ten from twenty, then multiply by three unless the result is negative",
        "Add five to the result of subtracting thirty from twenty, then multiply by three unless the result is negative",
        "10 / 2 / 5",
        "Add 5 and 10 and 15 and 20 and 25",
        "2 ^ 3 ^ 4",
        "10 ^ 2",
        "10^2",
        "2 to the power of 10",
        "(1 + 2) * (3 + (4 * 3))",
        "3 = 3",
        "2 is greater than 4",
        "Add sixty seven to the result of subtracting hundred from thousand, then multiply it by three if the result is greater than  or equal to 900.",
        "subtract -5 from 10",
        "Add -15 and 10",
        "Subtract -10 from -50",
        "Subtract twenty from ten then multiply by three unless the result is negative, then multiply by -1 if the result is less than zero",
        "add 5 5 5 5 5 5 5 then subtract the sum with 3 then subtract with 38",
        "sine of 30 degrees",
        "cosine of 45 degrees",
        "tangent of 60 degrees",
        "cosecant of 30 degrees",
        "secant of 60 degrees",
        "cotangent of 45 degrees",
        "inverse sine of 0.5",
        "inverse cosine of 0.5",
        "inverse tangent of 1",
        "sin 30 degrees",
        "cos 45 degrees",
        "sin 30",
        "sin 30 radians",
        "sine of add 5 5 5 5 5 5 5 then subtract the sum with 3 then subtract with 38 degrees",
        "sin 30 ^ 2 + cos 30 ^ 2",
        "inverse sine of 0.5 then multiply by two unless the result is greater than one hundred",


    ];

     for text in text {
         println!("Analyzing: {}", text);
         text_analyser(text);
         println!("-----------------------------");
     }

    println!("Result: {}", word_to_number("five million three thousand nine hundred ninety").unwrap_or_else(|| {
        println!("Failed to convert word to number");
        0.0
    }));
}