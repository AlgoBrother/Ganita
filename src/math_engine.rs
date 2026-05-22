fn add(a: i32, b: i32) -> f64 {
    a as f64 + b as f64
}

fn subtract(a: i32, b: i32) -> f64 {
    a as f64 - b as f64
}

fn multiply(a: i32, b: i32) -> f64 {
    a as f64 * b as f64
}

fn divide(a: i32, b: i32) -> Option<f64> {
    if b != 0 {
        Some(a as f64 / b as f64)
    } else {
        None
    }
}

// ======= Number word conversion functions =======
fn word_value(word: &str) -> Option<i32>{
  match word {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),

        "ten" => Some(10),
        "eleven" => Some(11),
        "twelve" => Some(12),
        "thirteen" => Some(13),
        "fourteen" => Some(14),
        "fifteen" => Some(15),
        "sixteen" => Some(16),
        "seventeen" => Some(17),
        "eighteen" => Some(18),
        "nineteen" => Some(19),

        "twenty" => Some(20),
        "thirty" => Some(30),
        "forty" => Some(40),
        "fifty" => Some(50),
        "sixty" => Some(60),
        "seventy" => Some(70),
        "eighty" => Some(80),
        "ninety" => Some(90),

        "hundred" => Some(100),
        "thousand" => Some(1_000),
        "million" => Some(1_000_000),
        "billion" => Some(1_000_000_000),

        _ => None,
  }
}

pub fn word_to_number(word: &str) -> Option<i32> {
    let mut total = 0;
    let mut current = 0;

    for part in word.split_whitespace() {
        if let Some(value) = word_value(part) {
            if value == 100 {
                current *= 100;
            } else if value >= 1000 {
                total += current * value;
                current = 0;
            } else {
                current += value;
            }
        } else {
            return None; // Return None if any part is not a valid number word
        }
    }

    total += current;
    Some(total)
}

fn is_number_word(word: &str) -> bool {
    word_value(word).is_some() // returns true if the word is a valid number word, false otherwise
}

// ========= End of number word conversion functions =======

// fn operation_order(){
//     // function to see what teh text wants
//     // subtract x from y means y - x. we dont want to do x - y. 
//     // same for divide y by x means y / x. we dont want to do x / y.
//     // Add five and ten means 5 + 10. <tbh new function for to convert word into numbers




// }

pub fn text_analyser(text: &str){
    // This function will analyze the text and print the objective (add, subtract, multiply, divide) and the numbers involved.
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut numbers: Vec<i32> = Vec::new();
    let mut current_phrase = String::new(); // This will hold the current phrase being analyzed, which can be useful for handling multi-word number phrases like "twenty five"

    let mut operation = "";
    let mut reverse_order = false; // This will be used to determine if we need to reverse the order of numbers for subtraction and division "from" and "by" will be the key words to determine this. If we encounter "from", we know that the order of numbers should be reversed for subtraction. If we encounter "by", we know that the order of numbers should be reversed for division.

    // Analyze the words to determine the operation and extract numbers
    for word in words {    

        match word.to_lowercase().as_str(){
            "add" | "plus" => operation = "add",
            "subtract" | "minus" => operation = "subtract",
            "multiply" | "times" => operation = "multiply",
            "divide" | "over" => operation = "divide",
            "and" | "by" => {
                if !current_phrase.is_empty() {
                    if let Some(num) = word_to_number(&current_phrase.trim()) {
                        numbers.push(num);
                    }
                    current_phrase.clear(); // Clear the current phrase after processing
                }
             }

             "from" => {
                reverse_order = true;
                if !current_phrase.is_empty(){
                    if let Some(num) = word_to_number(&current_phrase.trim()) {
                        numbers.push(num);
                    }
                    current_phrase.clear(); // Clear the current phrase after processing
                }
             }

             _ => {
                if let Ok(num) = word.parse::<i32>() {
                    numbers.push(num);
                } else if is_number_word(word) {
                    current_phrase.push_str(word);
                    current_phrase.push(' '); // Add a space to separate words in the phrase
                }
             }
        }
    }

    if !current_phrase.is_empty() {

        if let Some(num) = word_to_number(current_phrase.trim()) {
            numbers.push(num);
        }
    }
    
    println!("Operation: {}", operation);
    println!("Numbers: {:?}", numbers); // {:?} is used to print the vector in a readable format

    if reverse_order && numbers.len() >= 2 {
        numbers.swap(0, 1); // Swap the first two numbers to reverse their order
    }

    let result = match operation {
        "add" if numbers.len() >= 2 => add(numbers[0], numbers[1]),
        "subtract" if numbers.len() >= 2 => subtract(numbers[0], numbers[1]),
        "multiply" if numbers.len() >= 2 => multiply(numbers[0], numbers[1]),
        "divide" if numbers.len() >= 2 => divide(numbers[0], numbers[1]).unwrap_or_else(|| {
            println!("Cannot divide by zero");
            0.0 // Return a default value in case of division by zero
        }),
        _ => {
            println!("Invalid operation or insufficient numbers");
            return; // Exit the function if the operation is invalid or there are not enough numbers
        }
    };

    if result.fract() == 0.0 {
        println!("Result: {}", result as i64);
    } else {
        println!("Result: {:.4}", result);
    }
  

    
}