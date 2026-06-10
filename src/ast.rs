use crate::{math_engine::{is_number_word, word_to_number}, trignometry::trigo::{TrigonometricFunction, compute_trigo_func}};
use crate::trignometry::trigo::{AngleType, PI};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),

    BinOp {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    UnaryOp{ // for trigno
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
            "negative" => tokens.push(Token::Negative),
            "positive" => tokens.push(Token::Positive),
            "if"  | "when"    => tokens.push(Token::If),
            "in"      => tokens.push(Token::In),
            "convert" => tokens.push(Token::Convert),
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

            "pi" | "π" => tokens.push(Token::Number(std::f64::consts::PI)),
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
                        }
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
        let mut expr = self.parse_comparison()?;

        loop {  // ← loop so nested conditionals chain
            let mut then_op: Option<Operation> = None;
            let mut then_val: Option<Expression> = None;

            while matches!(self.peek(), Some(Token::Then)) {
                self.consume();
                self.skip_fillers();
                if let Some(Token::Op(op)) = self.peek().cloned() {
                    self.consume();
                    self.skip_fillers();
                    let val = self.parse_multiplicative()?;
                    
                    // If we already had a "then", fold it into the base expression 
                    // before saving the new one! This prevents chaining overwrites.
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

            let polarity = match self.peek() {
                Some(Token::If)     => { self.consume(); true }
                Some(Token::Unless) => { self.consume(); false }
                _ => {
                    if let (Some(op), Some(val)) = (then_op, then_val) {
                        expr = Expression::BinOp {
                            op, left: Box::new(expr), right: Box::new(val)
                        };
                    }
                    break;  // no more conditionals, exit loop
                }
            };

            // ... (The rest of parse_expression remains exactly the same starting from `while matches!... The | Result | Is | Of`)
            while matches!(self.peek(),
                Some(Token::The) | Some(Token::Result) |
                Some(Token::Is)  | Some(Token::Of)
            ) { self.consume(); }

            let condition = match self.peek() {
                Some(Token::Negative) => { self.consume(); Condition::IsNegative }
                Some(Token::Positive) => { self.consume(); Condition::IsPositive }
                Some(Token::Cmp(_)) => {
                    if let Some(Token::Cmp(op)) = self.peek().cloned() {
                        self.consume();
                        self.skip_fillers();
                        if let Some(Expression::Number(threshold)) = self.parse_primary() {
                            Condition::Comparison { op, threshold, polarity }
                        } else {
                            if let (Some(op), Some(val)) = (then_op, then_val) {
                                expr = Expression::BinOp {
                                    op, left: Box::new(expr), right: Box::new(val)
                                };
                            }
                            break;
                        }
                    } else { break; }
                }
                _ => {
                    if let (Some(op), Some(val)) = (then_op, then_val) {
                        expr = Expression::BinOp {
                            op, left: Box::new(expr), right: Box::new(val)
                        };
                    }
                    break;
                }
            };

            expr = Expression::Conditional {
                base: Box::new(expr),
                condition,
                guarded_op: then_op,
                guarded_val: then_val.map(Box::new),
            };
        }

        Some(expr)
    }

    // additive: handles infix + and - with standard left-to-right chaining
    fn parse_additive(&mut self) -> Option<Expression> {
        let mut left = self.parse_multiplicative()?;

        loop {
            match self.peek() {
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
                Some(Token::Op(Operation::Subtract)) => {
                    self.consume();
                    self.skip_fillers();
                    
                    // peek ahead: if next is a number/expr followed by "from",
                    // it's a natural-language "subtract X from Y" — use parse_subtract_args
                    // Otherwise it's plain infix: left - right
                    let x = self.parse_rhs();
                    if matches!(self.peek(), Some(Token::From)) {
                        // "subtract X from Y" form — Y is the new left, X is right
                        self.consume(); // eat From
                        let y = self.parse_rhs()?;
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
                _ => break,
            }
        }
        Some(left)
    }

    // handles ^ with right-associativity
    fn parse_power(&mut self) -> Option<Expression> {
        let mut left = self.parse_primary()?;

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
                    return Some(Expression::UnaryOp { 
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
                
                Some(Expression::UnaryOp { func, operand: Box::new(operand), unit })
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
            | Some(Token::Is) => { self.consume(); }  // ← removed The/Result/Of
            _ => break,
        }
    }
}
}

// ──────────────────────────────────────────────
// EVALUATOR
// ──────────────────────────────────────────────

pub fn evaluate(expr: &Expression) -> Result<f64, String> {
    match expr {
        Expression::Number(n) => Ok(*n),

        Expression::BinOp { op, left, right } => {
            let l = evaluate(left)?;
            let r = evaluate(right)?;
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
            let base_val = evaluate(base)?;

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
                        let v = evaluate(val)?;
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
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Ok(lv), Ok(rv)) => {
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
                
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        }
    
        Expression::UnaryOp { func, operand, unit } => {
            let angle = evaluate(operand)?;
            compute_trigo_func(func, angle, unit)
        }

        Expression::Convert { from, to, operand } => {
            let value = evaluate(operand)?;
            match (from, to) {
                (AngleType::Degrees, AngleType::Radians) => Ok(value * PI / 180.0),
                (AngleType::Radians, AngleType::Degrees) => Ok(value * 180.0 / PI),
                _ => Ok(value), // No conversion needed
            }
        }

    }
}