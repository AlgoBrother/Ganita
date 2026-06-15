use std::collections::HashMap;

use crate::{math_engine::{VarContext, is_number_word, word_to_number}, trignometry::trigo::{TrigonometricFunction, compute_trigo_func}, variable_solving::variable_solve::{solve_linear, solve_numerically}};
use crate::trignometry::trigo::{AngleType, PI};
use crate::utils::functions::factorial;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Variable(String), // for future extension with variables, e.g. "Let x be 5, then add x and 10"

    Assign { // for future extension with variable assignment, e.g. "Let x be 5"
        name: String,
        value: Box<Expression>,
    },

    UnaryOp {
        op: UnaryOperation,
        operand: Box<Expression>,
    },

    BinOp {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    TrigoOp{ // for trigno
        func: TrigonometricFunction,
        operand: Box<Expression>,
        unit: AngleType,
    },

    Conditional {
        base: Box<Expression>,
        condition: Condition,
        guarded_op: Option<Operation>,
        guarded_val: Option<Box<Expression>>, // For future extension: allow conditions like "unless the result is less than 10"
    },

    Comparison {
        op: CompareOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    Convert {
        from: AngleType,
        to: AngleType,
        operand: Box<Expression>,
    },

    Solve {
        var: String,
        equation: Box<Expression>,
    }
}

#[derive(Debug, Clone)]
pub enum Condition {
    IsNegative,
    IsPositive,
    IsZero,
    Comparison{
        op: CompareOp,
        threshold: f64, // the threshold value for the comparison 
        polarity: bool // true for "greater than", false for "less than"
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power, // Exponent
}

#[derive(Debug, Clone)]
pub enum UnaryOperation{
        Factorial, 
}

#[derive(Debug, Clone)]
pub enum CompareOp {
    GreaterThan,
    LessThan,
    EqualTo,
    NotEqualTo,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
}

#[derive(Debug, Clone)]
pub enum Token {
    Op(Operation),
    SingleDigitOp(UnaryOperation),
    Cmp(CompareOp),
    Number(f64),
    From,
    By,
    And,
    To,
    Then,
    The,
    Result,
    Of,
    Unless,
    Is,
    If,
    In,
    Convert,
    Let,
    Be,

    Solve,

    Negative,
    Positive,

    LParenthesis,
    RParenthesis,

    Sin, 
    Cos,
    Tan,
    Cosec,
    Sec,
    Cot,
    Arcsin,
    Arccos,
    Arctan,

    Radians,
    Degrees,

    Variable(String) // allows for future extension with variables, e.g. "Let x be 5, then add x and 10"
}


// TOKENIZER

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let raw: Vec<&str> = input.split_whitespace().collect();

    let mut i = 0;
    while i < raw.len() {
        let word_raw = raw[i];
        let word_lower = word_raw.to_lowercase();


        // ── Step 1: check raw word as a symbol or keyword first ──────────

        
        match word_lower.as_str() {
            // if block for checking text like 5! factorial operation support
            _ if word_lower.ends_with('!') && word_lower.len() > 1 => {
                let num_part = &word_lower[..word_lower.len()-1];
                if let Ok(num) = num_part.parse::<f64>() {
                    tokens.push(Token::Number(num));
                    tokens.push(Token::SingleDigitOp(UnaryOperation::Factorial));
                    i += 1;
                    continue;
                }
            }

            _ if word_lower.ends_with('x') && word_lower.len() > 1 => {
                let num_part = &word_lower[..word_lower.len()-1]; // strip the trailing 'x' to support inputs like "5x" as "5 * x"
                if let Ok(num) = num_part.parse::<f64>() {
                    tokens.push(Token::Number(num));
                    tokens.push(Token::Variable("x".to_string()));
                    i += 1;
                    continue;
                }
            }

            "+" => { tokens.push(Token::Op(Operation::Add));      i += 1; continue; }
            "-" => { tokens.push(Token::Op(Operation::Subtract)); i += 1; continue; }
            "*" => { tokens.push(Token::Op(Operation::Multiply)); i += 1; continue; }
            "/" => { tokens.push(Token::Op(Operation::Divide));   i += 1; continue; }
            "^" => { tokens.push(Token::Op(Operation::Power));    i += 1; continue; }
            "(" | "[" | "{" => { tokens.push(Token::LParenthesis); i += 1; continue; }
            ")" | "]" | "}" => { tokens.push(Token::RParenthesis); i += 1; continue; }
            "<"  => { tokens.push(Token::Cmp(CompareOp::LessThan)); i += 1; continue; }
            ">"  => { tokens.push(Token::Cmp(CompareOp::GreaterThan));i += 1; continue; }
            "="  => { tokens.push(Token::Cmp(CompareOp::EqualTo)); i += 1; continue; }
            "!=" => { tokens.push(Token::Cmp(CompareOp::NotEqualTo)); i += 1; continue; }
            "<=" => { tokens.push(Token::Cmp(CompareOp::LessThanOrEqualTo)); i += 1; continue; }
            ">=" => { tokens.push(Token::Cmp(CompareOp::GreaterThanOrEqualTo)); i += 1; continue; }
            "!" => {
                if raw.get(i + 1) == Some(&"=") {
                    tokens.push(Token::Cmp(CompareOp::NotEqualTo));
                    i += 2;
                    continue;
                } else {
                    tokens.push(Token::SingleDigitOp(UnaryOperation::Factorial));
                    i += 1;
                    continue;
                }
            }
            _ => {}
        }

        // ── Step 2: strip punctuation (commas, periods at end, etc.)
        //    but keep '-' for negative numbers and '.' for decimals
        let word: String = word_lower
            .trim_matches(|c: char| c == ',' || c == '!' || c == '?')
            .to_string();

        match word.as_str() {
            "add"  | "plus"  | "adding"              => tokens.push(Token::Op(Operation::Add)),
            "subtract" | "subtracting"  => tokens.push(Token::Op(Operation::Subtract)),
            "minus" => {
                // peek: is next word "from"? If so, it's a subtraction, not a negative number
                if let Some(next) = raw.get(i + 1) {
                    let next_lower = next.trim_matches(|c: char| c == ',' || c == '?').to_lowercase();
                    if let Ok(num) = next_lower.parse::<f64>() {  
                        tokens.push(Token::Number(-num));
                        i += 2;
                        continue;
                    } else if is_number_word(&next_lower) {
                        if let Some(num) = word_to_number(&next_lower) {
                            tokens.push(Token::Number(-num));
                            i += 2;
                            continue;
                        }
                    }
                }
                tokens.push(Token::Op(Operation::Subtract));
            },
            "multiply" | "multiplying" | "times"     => tokens.push(Token::Op(Operation::Multiply)),
            "divide" | "dividing" | "over"           => tokens.push(Token::Op(Operation::Divide)),
            "power" | "exponent"                     => tokens.push(Token::Op(Operation::Power)),
            "from"     => tokens.push(Token::From),
            "by"       => tokens.push(Token::By),
            "and"      => tokens.push(Token::And),
            "to"       => tokens.push(Token::To),
            "then"     => tokens.push(Token::Then),
            "the"      => tokens.push(Token::The),
            "result"   => tokens.push(Token::Result),
            "of"       => tokens.push(Token::Of),
            "unless"   => tokens.push(Token::Unless),
            "is"       => tokens.push(Token::Is),
            "solve" | "find" | "determine"  => tokens.push(Token::Solve),
            "negative" => tokens.push(Token::Negative),
            "positive" => tokens.push(Token::Positive),
            "if"  | "when"    => tokens.push(Token::If),
            "in"      => tokens.push(Token::In),
            "convert" => tokens.push(Token::Convert),
            "let"     => tokens.push(Token::Let),
            "be"      => tokens.push(Token::Be),
            "sin" | "sine"     => tokens.push(Token::Sin),
            "cos" | "cosine"   => tokens.push(Token::Cos),
            "tan" | "tangent"  => tokens.push(Token::Tan),
            "cosec" | "csc" | "cosecant"   => tokens.push(Token::Cosec),
            "sec" | "secant"   => tokens.push(Token::Sec),
            "cot" | "cotangent" => tokens.push(Token::Cot),
            "arcsin" | "sin^-1" => tokens.push(Token::Arcsin),
            "arccos" | "cos^-1" => tokens.push(Token::Arccos),
            "arctan" | "tan^-1"  => tokens.push(Token::Arctan),
            "inverse" => {
                if let Some(next) = raw.get(i + 1) {
                    match next.to_lowercase().as_str() {
                        "sine" | "sin"     => { tokens.push(Token::Arcsin); i += 2; continue; }
                        "cosine" | "cos"   => { tokens.push(Token::Arccos); i += 2; continue; }
                        "tangent" | "tan"  => { tokens.push(Token::Arctan); i += 2; continue; }
                        _ => {}
                    }
                }
                // unknown "inverse X", skip
                i += 1; continue;
            }


            "radian" | "radians" => tokens.push(Token::Radians),
            "degree" | "degrees" => tokens.push(Token::Degrees),


            "greater"  | "more" => {
                // peek: is next word "than"?
                if raw.get(i + 1).map(|s| s.to_lowercase()) == Some("than".to_string()) {
                    // peek again: is it ">= " (greater than or equal to)?
                    if raw.get(i + 2).map(|s| s.to_lowercase()) == Some("or".to_string()){
                        tokens.push(Token::Cmp(CompareOp::GreaterThanOrEqualTo));
                        i += 5; // skip "than or equal to"
                    } else{
                        tokens.push(Token::Cmp(CompareOp::GreaterThan));
                        i += 2; // skip "than"
                    }
                } else{
                    tokens.push(Token::Cmp(CompareOp::GreaterThan));
                    i += 1;
                }
                continue;
            },

            "less" | "fewer" | "smaller" => {
                if raw.get(i  +1).map(|s| s.to_lowercase()) == Some("than".to_string()){
                    if raw.get(i + 2).map(|s| s.to_lowercase()) == Some("or".to_string()){
                        tokens.push(Token::Cmp(CompareOp::LessThanOrEqualTo));
                        i += 5; // skip "than or equal to"
                    }else{
                        tokens.push(Token::Cmp(CompareOp::LessThan));
                        i += 2; // skip "than"
                    }
                } else{
                    tokens.push(Token::Cmp(CompareOp::LessThan));
                    i += 1;
                }
                continue;
            },

            "equal" | "equals" => {
                // skip optional "to"
                let skip = if raw.get(i + 1).map(|s| s.to_lowercase()) == Some("to".to_string()) { 2 } else { 1 };
                tokens.push(Token::Cmp(CompareOp::EqualTo));
                i += skip;
                continue;
            },

            "not" => {
                // "not equal to"
                if raw.get(i + 1).map(|s| s.to_lowercase()) == Some("equal".to_string()) {
                    tokens.push(Token::Cmp(CompareOp::NotEqualTo));
                    i += if raw.get(i + 2).map(|s| s.to_lowercase()) == Some("to".to_string()) { 3 } else { 2 };
                } else {
                    i += 1; // unknown "not", skip
                }
                continue;
            },

            "factorial" => tokens.push(Token::SingleDigitOp(UnaryOperation::Factorial)),

            "pi" | "π" => tokens.push(Token::Number(std::f64::consts::PI)),

            "where" | "such" | "that" | "what" | "value" => {} // equation introducers, silently skip

            _ => {
                if let Ok(num) = word.parse::<f64>() {
                    tokens.push(Token::Number(num));
                } else {
                    // for number words ("twenty-one" -> "twenty one"), we replace hyphens with spaces and check if it's a valid number word or phrase
                    let word = word.replace('-', " ");
                    let word = word.trim();

                    if is_number_word(word) || word.contains(' ') {
                        let mut phrase = word.to_string();

                        loop {
                            if i + 1 >= raw.len() { break; }

                            let next = raw[i + 1]
                                .trim_matches(|c: char| c == ',' || c == '!' || c == '?')
                                .to_lowercase();

                            if !is_number_word(&next) {
                                break;
                            }

                            let candidate = format!("{} {}", phrase, next);

                            if word_to_number(&candidate).is_some() {
                                phrase = candidate;
                                i += 1;
                            } else {
                                break;
                            }
                        }

                        if let Some(num) = word_to_number(&phrase) {
                            tokens.push(Token::Number(num));
                            i += 1; // skip the last word of the phrase
                            continue;
                        }
                    }

                    if word.len() == 1 && word.chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false){
                        tokens.push(Token::Variable(word.to_string()));
                    } 
                }
                // unknown words silently skipped
            }
        }
        i += 1;
    }
    tokens
}

// ──────────────────────────────────────────────
// PARSER — recursive descent
//
// Grammar (simplified):
//   program      → expression (Unless condition)?
//   expression   → additive (Then additive)*
//   additive     → multiplicative ((Add|Sub) rhs)*
//   multiplicative → primary ((Mul|Div) primary)*
//   primary      → NUMBER
//                | Op subtract_args        (prefix operator)
//                | The Result Of additive  (sub-expression grouping)
//   rhs          → (The Result Of)? additive   (allows "the result of X")
//
// Key design decision: "then" re-enters the TOP of additive rather than
// being handled as a loop-continue inside parse_primary. This means
// "Add 5 to <sub>, then multiply by 3" builds:
//   Mul(Add(5, sub), 3)   ← correct, multiply wraps the whole thing
// ──────────────────────────────────────────────

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    // ── Top-level entry: parse expression then check for Unless ──────────
    pub fn parse_expression(&mut self) -> Option<Expression> {
        // First check if the input STARTS with Solve token
        // "find x where/such/that ..." — prefix form
        if matches!(self.peek(), Some(Token::Solve)) {
            self.consume();
            self.skip_fillers();
            if let Some(Token::Variable(var_name)) = self.peek().cloned() {
                self.consume();
                self.skip_fillers(); // eats "where", "such", "that", "in"
                let equation = self.parse_comparison_then_conditional()?;
                return Some(Expression::Solve {
                    var: var_name,
                    equation: Box::new(equation),
                });
            }
        }

        // Parse the full expression normally (handles all existing cases)
        let mut expr = self.parse_comparison_then_conditional()?;

        // After the main expression is parsed, check for solve patterns
        // "x + 9 = 0, find x" — suffix form
        // Comma is stripped by tokenizer, so we just check for Solve token
        if matches!(self.peek(), Some(Token::Solve)) {
            // existing suffix solve handling
            self.consume();
            self.skip_fillers();
            if let Some(Token::Variable(var_name)) = self.peek().cloned() {
                self.consume();
                expr = Expression::Solve { var: var_name, equation: Box::new(expr) };
            }
        } else if matches!(self.peek(), Some(Token::If)) {
            // "x if x + 5 = 10" — the variable is already in expr as Variable("x")
            if let Expression::Variable(var_name) = &expr {
                let var_name = var_name.clone();
                self.consume(); // eat If
                self.skip_fillers();
                let equation = self.parse_comparison_then_conditional()?;
                expr = Expression::Solve { var: var_name, equation: Box::new(equation) };
            }
        }
        Some(expr)
    }

    // Seperated core conditional/then logic from parse_expression and made a seperate functionn.
    fn parse_comparison_then_conditional(&mut self) -> Option<Expression> {
        let mut expr = self.parse_comparison()?;

        loop {
            let mut then_op: Option<Operation> = None;
            let mut then_val: Option<Expression> = None;

            while matches!(self.peek(), Some(Token::Then)) {
                self.consume();
                self.skip_fillers();
                if let Some(Token::Op(op)) = self.peek().cloned() {
                    self.consume();
                    self.skip_fillers();
                    let val = self.parse_multiplicative()?;

                    if let (Some(prev_op), Some(prev_val)) = (then_op, then_val) {
                        expr = Expression::BinOp {
                            op: prev_op,
                            left: Box::new(expr),
                            right: Box::new(prev_val),
                        };
                    }
                    then_op = Some(op);
                    then_val = Some(val);
                }
            }

            // Save the state
            let state_before_condition = self.pos; // in case we need to backtrack if condition parsing fails (example: What is the value of x if x + 5 = 10, here x is unknown whihc will result in variable not found error.)
            let is_if_condition = matches!(self.peek(), Some(Token::If));
            let is_unless_condition = matches!(self.peek(), Some(Token::Unless));

            if !is_if_condition && !is_unless_condition {
                if let (Some(op), Some(val)) = (then_op, then_val) {
                    expr = Expression::BinOp {
                        op,
                        left: Box::new(expr),
                        right: Box::new(val),
                    };
                }
                break;
            }

            let polarity = is_if_condition; // true for "if", false for "unless"
            self.consume(); // Consume the If / Unless token



        //     let polarity = match self.peek() {
        //         Some(Token::If)     => { self.consume(); true }
        //         Some(Token::Unless) => { self.consume(); false }
        //         _ => {
        //             if let (Some(op), Some(val)) = (then_op, then_val) {
        //                 expr = Expression::BinOp {
        //                     op, left: Box::new(expr), right: Box::new(val)
        //                 };
        //             }
        //             break;
        //         }
        //     };

            while matches!(self.peek(),
                Some(Token::The) | Some(Token::Result) |
                Some(Token::Is)  | Some(Token::Of)
            ) { self.consume(); }

            let condition_opt = match self.peek() {
                Some(Token::Negative) => { self.consume(); Some(Condition::IsNegative) }
                Some(Token::Positive) => { self.consume(); Some(Condition::IsPositive) }
                Some(Token::Cmp(op)) => {
                    let op = op.clone();
                    self.consume();
                    self.skip_fillers();
                    if let Some(Expression::Number(threshold)) = self.parse_primary() {
                        Some(Condition::Comparison { op, threshold, polarity })
                    } else {
                        None
                    }
                }
                _ => None // It's either an equation or invalid, abort conditional parsing
            };

            if let Some(condition) = condition_opt{
                expr = Expression::Conditional { 
                    base: Box::new(expr),
                    condition,
                    guarded_op: then_op,
                    guarded_val: then_val.map(Box::new) 
                };
            } else{
                self.pos = state_before_condition;
            
                if let (Some(op), Some(val)) = (then_op, then_val) {
                    expr = Expression::BinOp {
                        op, left: Box::new(expr), right: Box::new(val)
                    };
                }
                break;
            }
            // let condition = match self.peek() {
            //     Some(Token::Negative) => { self.consume(); Condition::IsNegative }
            //     Some(Token::Positive) => { self.consume(); Condition::IsPositive }
            //     Some(Token::Cmp(_)) => {
            //         if let Some(Token::Cmp(op)) = self.peek().cloned() {
            //             self.consume();
            //             self.skip_fillers();
            //             if let Some(Expression::Number(threshold)) = self.parse_primary() {
            //                 Condition::Comparison { op, threshold, polarity }
            //             } else {
            //                 if let (Some(op), Some(val)) = (then_op, then_val) {
            //                     expr = Expression::BinOp {
            //                         op, left: Box::new(expr), right: Box::new(val)
            //                     };
            //                 }
            //                 break;
            //             }
            //         } else { break; }
            //     }
        //         _ => {
        //             if let (Some(op), Some(val)) = (then_op, then_val) {
        //                 expr = Expression::BinOp {
        //                     op, left: Box::new(expr), right: Box::new(val)
        //                 };
        //             }
        //             break;
        //         }
        //     };

        //     expr = Expression::Conditional {
        //         base: Box::new(expr),
        //         condition,
        //         guarded_op: then_op,
        //         guarded_val: then_val.map(Box::new),
        //     };
        }
        Some(expr)
    }

    // additive: handles infix + and - with standard left-to-right chaining
    fn parse_additive(&mut self) -> Option<Expression> {
        let mut left = self.parse_multiplicative()?;

        loop { 
            match self.peek() {
                // Addition Operation: left + right
                Some(Token::Op(Operation::Add)) => {
                    self.consume();
                    self.skip_fillers();
                    let right = self.parse_rhs()?;
                    left = Expression::BinOp {
                        op: Operation::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }

                // Subtraction Operation: left - right
                Some(Token::Op(Operation::Subtract)) => {
                    self.consume();
                    self.skip_fillers();
                    
                    // peek ahead: if next is a number/expr followed by "from",
                    // then it's a natural-language "subtract X from Y" => use parse_subtract_args 
                    // Otherwise it's plain infix: left - right
                    let x = self.parse_rhs();
                    if matches!(self.peek(), Some(Token::From)) {
                        // "subtract X from Y" form — Y is the new left, X is right
                        self.consume(); // eat From
                        let y = self.parse_rhs()?; // parse Y after "from"
                        left = Expression::BinOp {
                            op: Operation::Subtract,
                            left: Box::new(y),
                            right: Box::new(x?),
                        };
                    } else {
                        // plain infix: accumulated left - right
                        left = Expression::BinOp {
                            op: Operation::Subtract,
                            left: Box::new(left),
                            right: Box::new(x?),
                        };
                    }
                }

                _ => break,
            }
        }
        Some(left)
    }

    // multiplicative: handles infix * and /
    fn parse_multiplicative(&mut self) -> Option<Expression> {
        let mut left = self.parse_power()?; // ← fix: was parse_primary(), now we allow powers to be nested inside mul/div

        loop {
            match self.peek() {
                Some(Token::Op(Operation::Multiply)) => {
                    self.consume();
                    self.skip_fillers();
                    let right = self.parse_power()?;
                    left = Expression::BinOp {
                        op: Operation::Multiply,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some(Token::Op(Operation::Divide)) => {
                    self.consume();
                    self.skip_fillers();
                    let right = self.parse_power()?;
                    left = Expression::BinOp {
                        op: Operation::Divide,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }

                Some(Token::Variable(_) | Token::Sin | Token::Cos | Token::Tan | Token::Cosec | Token::Sec | Token::Cot | Token::Arcsin | Token::Arccos | Token::Arctan | Token::LParenthesis) => {
                    // Handle implicit multiplication: e.g. "2x", "3sin(30)", "(x+1)(x-1)"
                    self.skip_fillers();
                    let right = self.parse_power()?;
                    left = Expression::BinOp {
                        op: Operation::Multiply,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }

                _ => break,
            }
        }

        Some(left)
    }

    // handles operators that come immediately after a value, like factorial
    fn parse_postfix(&mut self) -> Option<Expression> {
        let mut left = self.parse_primary()?;

        loop {
            match self.peek() {
                Some(Token::SingleDigitOp(UnaryOperation::Factorial)) => {
                    self.consume();
                    left = Expression::UnaryOp { 
                        op: UnaryOperation::Factorial, 
                        operand: Box::new(left)
                     };
                }
                _ => break,
            }
        }
        Some(left)
    }

    // handles ^ with right-associativity
    fn parse_power(&mut self) -> Option<Expression> {
        let mut left = self.parse_postfix()?;
        
        loop{
            // Since Exponentiation is right-associative, we don't want to loop here like the others.
            // Instead, we check for the power operator and if it's there, we consume it and parse the right
            if matches!(self.peek(), Some(Token::Op(Operation::Power))) {
                self.consume();
                self.skip_fillers();
                let right = self.parse_power()?; // recursive call to handle right-associativity
                left = Expression::BinOp {
                    op: Operation::Power,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else { break; }
        }
        Some(left)
    }

    // compare logic
    fn parse_comparison(&mut self) -> Option<Expression> {
        let left = self.parse_additive()?;
        self.skip_fillers();
        if let Some(Token::Cmp(cmp_op)) = self.peek().cloned() {
            self.consume();
            self.skip_fillers();
            let right = self.parse_additive()?;
            Some(Expression::Comparison {
                op: cmp_op,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Some(left) // no comparison, return the additive expression
        }
    }
    
    // rhs: right-hand side that may be introduced by "the result of <sub-expr>"
    fn parse_rhs(&mut self) -> Option<Expression> {
        self.skip_fillers();
        if matches!(self.peek(), Some(Token::The) | Some(Token::Result) | Some(Token::Of)) {
            while matches!(self.peek(), Some(Token::The) | Some(Token::Result) | Some(Token::Of)) {
                self.consume();
            }
            self.parse_additive()
        } else {
            self.parse_multiplicative()  // ← fix: was parse_primary()
        }
    }

    // "subtract X from Y" → (Y, X) → Y - X
    // "subtract X and Y"  → (X, Y) → X - Y
    fn parse_subtract_args(&mut self) -> Option<(Expression, Expression)> {
        let x = self.parse_rhs()?;
        if matches!(self.peek(), Some(Token::From)) {
            self.consume();
            let y = self.parse_rhs()?;
            Some((y, x)) // reversed
        } else {
            self.skip_fillers();
            let y = self.parse_rhs()?;
            Some((x, y))
        }
    }

    // primary: terminal values and prefix operators
    fn parse_primary(&mut self) -> Option<Expression> {
        self.skip_fillers();

        match self.peek()?.clone() {
            Token::Number(num) => {
                self.consume();
                Some(Expression::Number(num))
            }

            // Prefix operator — "Add 5 and 10", "Multiply 2 3 4"
            Token::Op(op) => {
                self.consume();
                self.skip_fillers();
                match op {
                    Operation::Subtract => {
                        let (l, r) = self.parse_subtract_args()?;
                        let expr = Expression::BinOp {
                            op: Operation::Subtract,
                            left: Box::new(l),
                            right: Box::new(r),
                        };
                        Some(self.fold_same_op(expr, Operation::Subtract))
                    }
                    _ => {
                        let first = self.parse_primary()?;
                        Some(self.fold_operands(op, first))
                    }
                }
            }

            // Safety net for stray The/Result/Of
            Token::The | Token::Result | Token::Of => {
                self.consume();
                self.skip_fillers();
                self.parse_primary()
            }

            // Parenthesised sub-expression
            Token::LParenthesis => {
                self.consume(); 
                let expression = self.parse_additive(); // allow full additive expressions inside parentheses
                if matches!(self.peek(), Some(Token::RParenthesis)){ // check for closing parenthesis
                    self.consume();
                    expression
                } else {
                    None // unbalanced parentheses
                }
            }

            Token::Sin | Token::Cos | Token::Tan | Token::Cosec | Token::Sec | Token::Cot | Token::Arcsin | Token::Arccos | Token::Arctan => {
                let func = match self.peek().clone().unwrap() {
                    Token::Sin => TrigonometricFunction::Sine,
                    Token::Cos => TrigonometricFunction::Cosine,
                    Token::Tan => TrigonometricFunction::Tangent,
                    Token::Cosec => TrigonometricFunction::Cosecant,
                    Token::Sec => TrigonometricFunction::Secant,
                    Token::Cot => TrigonometricFunction::Cotangent,
                    Token::Arcsin => TrigonometricFunction::InverseSine,
                    Token::Arccos => TrigonometricFunction::InverseCosine,
                    Token::Arctan => TrigonometricFunction::InverseTangent,
                    _ => unreachable!(), 
                };

                self.consume(); // consume the trigonometric function token
                self.skip_fillers(); // skips words like "of"

                // Backtracking Lookahead Strategy: 
                let saved_pos = self.pos;
                let mut explicit_block_success = false;
                let mut explicit_expr = None;
                let mut explicit_unit = AngleType::Degrees;

                // ATTEMPT 1: Try parsing a massive loose expression block.
                // ONLY accept it if the user explicitly closed it with 'degrees' or 'radians'.
                // This satisfies: "sine of add 5 ... then subtract ... degrees"
                if let Some(expr) = self.parse_expression() {
                    if matches!(self.peek(), Some(Token::Degrees) | Some(Token::Radians)) {
                        explicit_block_success = true;
                        explicit_expr = Some(expr);
                        explicit_unit = match self.peek() {
                            Some(Token::Degrees) => { self.consume(); AngleType::Degrees },
                            Some(Token::Radians) => { self.consume(); AngleType::Radians },
                            _ => unreachable!(),
                        };
                    }
                }

                if explicit_block_success {
                    return Some(Expression::TrigoOp { 
                        func, 
                        operand: Box::new(explicit_expr.unwrap()), 
                        unit: explicit_unit 
                    });
                }

                // ATTEMPT 2: Backtrack! The long expression didn't end in a unit.
                // Revert parser state and parse tightly to respect BODMAS.
                // This safely satisfies: "sin 30 ^ 2" and "sin 30 then add 5"
                self.pos = saved_pos;
                let operand = self.parse_primary()?; 

                let unit = match self.peek() {
                    Some(Token::Degrees) => { self.consume(); AngleType::Degrees },
                    Some(Token::Radians) => { self.consume(); AngleType::Radians },
                    _ => AngleType::Degrees, // default to degrees if no unit specified
                 };
                
                Some(Expression::TrigoOp { func, operand: Box::new(operand), unit })
            }

            Token::Convert => {
                self.consume();
                self.skip_fillers();
                let operand = self.parse_primary()?;
                self.skip_fillers();  // calling again to skip "degrees/radians to radians/degrees" or "in radians/degrees"
                
                let from_unit = match self.peek() {
                    Some(Token::Degrees) => {self.consume(); AngleType::Degrees},
                    Some(Token::Radians) => {self.consume(); AngleType::Radians},
                    _ => AngleType::Radians, // default to radians if no target unit specified
                };

                if let Some(Token::To) = self.peek() {
                    self.consume();
                } else if let Some(Token::In) = self.peek() {
                    self.consume();
                }

                self.skip_fillers(); // skip any fillers before the target unit

                let target_unit = match self.peek() {
                    Some(Token::Degrees) => { self.consume(); AngleType::Degrees },
                    Some(Token::Radians) => { self.consume(); AngleType::Radians },
                    _ => AngleType::Radians,    // default to radians if no target unit specified
                };

                Some(Expression::Convert { 
                    from: from_unit,
                    to: target_unit,
                    operand: Box::new(operand) 
                })
            }
            
            Token::Let => {
                self.consume();
                if let Some(Token::Variable(name)) = self.peek().cloned() {
                    self.consume(); // consume/process variable name
                    self.skip_fillers();

                    match self.peek() {
                        Some(Token::Be) => {self.consume();},
                        Some(Token::Cmp(CompareOp::EqualTo)) => { self.consume(); }
                        _ => {} // allow "Let x 5" as well as "Let x be 5" and "Let x = 5"
                    }

                    let value = self.parse_additive()?;
                    Some(Expression::Assign { name, value: Box::new(value) })
                } else {
                    None // invalid variable name
                }
            }

            Token::Solve => {
                self.consume();
                self.skip_fillers();
                if let Some(Token::Variable(name)) = self.peek().cloned() {
                    self.consume();
                    self.skip_fillers();
                    let equation = self.parse_expression()?;
                    Some(Expression::Solve{
                        var: name,
                        equation: Box::new(equation)})
                } else {
                    None
                }
            }

            Token::Variable(name) => {
                self.consume();
                Some(Expression::Variable(name))
            }
            
            _ => None,
        }
    }

    // Fold additional space-separated numbers with the same op
    // "Add 1 2 3 4 5" → Add(Add(Add(Add(1,2),3),4),5)
    fn fold_operands(&mut self, op: Operation, first: Expression) -> Expression {
        let mut acc = first;
        loop {
            self.skip_fillers();
            match self.peek() {
                Some(Token::Number(_)) => {
                    if let Some(next) = self.parse_additive() {
                        acc = Expression::BinOp {
                            op: op.clone(),
                            left: Box::new(acc),
                            right: Box::new(next),
                        };
                    } else { break; }
                    // break; 
                }
                // "Add 5 to the result of X" — right side is a sub-expression
                Some(Token::The) | Some(Token::Result) | Some(Token::Of) => {
                    if let Some(next) = self.parse_rhs() {
                        acc = Expression::BinOp {
                            op: op.clone(),
                            left: Box::new(acc),
                            right: Box::new(next),
                        };
                    }
                    break; // sub-expression always terminates the fold
                }
                _ => break,
            }
        }
        acc
    }

    // Same but for subtract chains: "Subtract 100 20 5 5"
    fn fold_same_op(&mut self, first: Expression, op: Operation) -> Expression {
        let mut acc = first;
        loop {
            self.skip_fillers();
            if matches!(self.peek(), Some(Token::Number(_))) {
                if let Some(next) = self.parse_primary() {
                    acc = Expression::BinOp {
                        op: op.clone(),
                        left: Box::new(acc),
                        right: Box::new(next),
                    };
                } else { break; }
            } else { break; }
        }
        acc
    }

    fn skip_fillers(&mut self) {
    loop {
        match self.peek() {
            Some(Token::And)
            | Some(Token::To)
            | Some(Token::By) 
            | Some(Token::Of)
            | Some(Token::Is)
            | Some(Token::In) => { self.consume(); }  // for "convert 30 degrees in radians"
            _ => break,
        }
    }
}
}

// ──────────────────────────────────────────────
// EVALUATOR
// ──────────────────────────────────────────────

pub fn evaluate_with_context(expr: &Expression, context: &mut VarContext) -> Result<f64, String> {
    match expr {
        Expression::Number(n) => Ok(*n),

        Expression::Variable(name) => {
            context.get(name).copied().ok_or_else(|| format!("{} Variable not found", name))
        },

        Expression::Assign { name, value } => {
            let value = evaluate_with_context(value, context)?;
            context.insert(name.clone(), value);
            Ok(value)
        },

        Expression::UnaryOp { op, operand } => {
            let value = evaluate_with_context(operand, context)?;
            match op {
                UnaryOperation::Factorial => Ok(factorial(value)),
            }
        }

        Expression::BinOp { op, left, right } => {
            let l = evaluate_with_context(left, context)?;
            let r = evaluate_with_context(right, context)?;
            match op {
                Operation::Add      => Ok(l + r),
                Operation::Subtract => Ok(l - r),
                Operation::Multiply => Ok(l * r),
                Operation::Divide   => {
                    if r != 0.0 { Ok(l / r) }
                    else { Err("Error: Division by zero".to_string()) }
                }
                Operation::Power    => Ok(l.powf(r)),
            }
        }

        Expression::Conditional { base, condition, guarded_op, guarded_val } => {
            let base_val = evaluate_with_context(base, context)?;

            let should_apply_guarded_op = match condition {
                Condition::IsNegative => base_val >= 0.0,  // apply when NOT negative
                Condition::IsPositive => base_val <= 0.0,  // apply when NOT positive
                Condition::IsZero    => base_val != 0.0,  // apply when NOT zero
                Condition::Comparison { op, threshold, polarity } => {
                    let raw = match op {
                        CompareOp::GreaterThan          => base_val > *threshold,
                        CompareOp::LessThan             => base_val < *threshold,
                        CompareOp::EqualTo              => (base_val - threshold).abs() < 1e-9,
                        CompareOp::NotEqualTo           => (base_val - threshold).abs() >= 1e-9,
                        CompareOp::GreaterThanOrEqualTo => base_val >= *threshold,
                        CompareOp::LessThanOrEqualTo    => base_val <= *threshold,
                    };
                    // "if"     (polarity=true)  → apply when raw is true
                    // "unless" (polarity=false) → apply when raw is false
                    if *polarity { raw } else { !raw }
                }
            };

            if should_apply_guarded_op {
                // condition says yes — apply the guarded operation
                match (guarded_op, guarded_val) {
                    (Some(op), Some(val)) => {
                        let v = evaluate_with_context(val, context)?;
                        match op {
                            Operation::Add      => Ok(base_val + v),
                            Operation::Subtract => Ok(base_val - v),
                            Operation::Multiply => Ok(base_val * v),
                            Operation::Divide   => {
                                if v != 0.0 { Ok(base_val / v) }
                                else { Err("Division by zero in conditional".to_string()) }
                            }
                            Operation::Power    => Ok(base_val.powf(v)),

                        }
                    }
                    _ => Ok(base_val),
                }
            } else {
                // condition says no — return base unchanged
                Ok(base_val)
            }
        }

        Expression::Comparison { op, left, right } => {
            let lv = evaluate_with_context(left, context)?;
            let rv = evaluate_with_context(right, context)?;
            let result = match op {
                CompareOp::GreaterThan => lv > rv,
                CompareOp::LessThan => lv < rv,
                CompareOp::EqualTo => (lv - rv).abs() < 1e-9, // handle floating-point equality with a tolerance we do this instead of == to avoid issues where two numbers are mathematically equal but differ in their last decimal places due to floating-point precision limitations
                CompareOp::NotEqualTo => (lv - rv).abs() >= 1e-9,
                CompareOp::GreaterThanOrEqualTo => lv >= rv,
                CompareOp::LessThanOrEqualTo => lv <= rv,
            };
            Ok(result as u8 as f64) // return 1.0 for true, 0.0 for false
        }
    
        // Trigonometric functions with unit handling
        Expression::TrigoOp { func, operand, unit } => {
            let angle = evaluate_with_context(operand, context)?;
            compute_trigo_func(func, angle, unit)
        }

        Expression::Convert { from, to, operand } => {
            let value = evaluate_with_context(operand, context)?;
            match (from, to) {
                (AngleType::Degrees, AngleType::Radians) => Ok(value * PI / 180.0),
                (AngleType::Radians, AngleType::Degrees) => Ok(value * 180.0 / PI),
                _ => Ok(value), // No conversion needed
            }
        }

        Expression::Solve { var, equation} => {
            match equation.as_ref() {
                Expression::Comparison { op: CompareOp::EqualTo, left, right } => {
                    let lhs_minus_rhs = Expression::BinOp{
                        op: Operation::Subtract,
                        left: left.clone(),
                        right: right.clone(),
                    };
                    solve_linear(lhs_minus_rhs.clone(), var, 0.0).or_else(|_| solve_numerically(&lhs_minus_rhs, var, 1.0) )
                }
                _ => Err("Equation is required for anything to be solved".to_string()),
            }
        }
    }
}

pub fn evaluate(expr: &Expression) -> Result<f64, String> {
    evaluate_with_context(expr, &mut HashMap::new())
}