use crate::math_engine::{word_to_number, is_number_word};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    BinOp {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Conditional {
        base: Box<Expression>,
        condition: Condition,
        guarded_op: Option<Operation>,
        guarded_val: Option<Box<Expression>>, // For future extension: allow conditions like "unless the result is less than 10"
    },
}

#[derive(Debug, Clone)]
pub enum Condition {
    IsNegative,
    IsPositive,
    IsZero,
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
pub enum Token {
    Op(Operation),
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
    Negative,
    Positive,
    LParenthesis,
    RParenthesis,
}

// ──────────────────────────────────────────────
// TOKENIZER
// ──────────────────────────────────────────────
//
// Symbol handling strategy:
//   Trim punctuation EXCEPT when the whole word is a known symbol (+,-,*,/)
//   or when it's a negative number (-5).
//   We do NOT use trim_matches for operator detection — instead we check the
//   raw word first, then trim for number parsing.

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
            _ => {}
        }

        // ── Step 2: strip punctuation (commas, periods at end, etc.)
        //    but keep '-' for negative numbers and '.' for decimals
        let word: String = word_lower
            .trim_matches(|c: char| c == ',' || c == '!' || c == '?')
            .to_string();

        match word.as_str() {
            "add"  | "plus"  | "adding"              => tokens.push(Token::Op(Operation::Add)),
            "subtract" | "subtracting" | "minus"     => tokens.push(Token::Op(Operation::Subtract)),
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
            "("   | "[" | "{"     => tokens.push(Token::LParenthesis),
            ")"   | "]" | "}"     => tokens.push(Token::RParenthesis),
            _ => {
                // ── Step 3: try to parse as a number (handles -5, 3.14, etc.)
                if let Ok(num) = word.parse::<f64>() {
                    tokens.push(Token::Number(num));
                } else if is_number_word(&word) {
                    // ── Step 4: greedily consume multi-word number phrase
                    let mut phrase = word.clone();
                    loop {
                        if i + 1 >= raw.len() { break; }
                        let next = raw[i + 1]
                            .trim_matches(|c: char| c == ',' || c == '!' || c == '?')
                            .to_lowercase();
                        if !is_number_word(&next) { break; }
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
        let mut expr = self.parse_additive()?;

        // Collect the "then <op> <val>" if present — but DON'T apply it yet
        let mut then_op: Option<Operation> = None;
        let mut then_val: Option<Expression> = None;

        while matches!(self.peek(), Some(Token::Then)) {
            self.consume();
            self.skip_fillers();
            if let Some(Token::Op(op)) = self.peek().cloned() {
                self.consume();
                self.skip_fillers();
                let val = self.parse_multiplicative()?;
                then_op = Some(op);
                then_val = Some(val);
            }
        }

        // Now check for unless
        if matches!(self.peek(), Some(Token::Unless)) {
            self.consume();
            while matches!(self.peek(),
                Some(Token::The) | Some(Token::Result) |
                Some(Token::Is)  | Some(Token::Of)
            ) { self.consume(); }

            let condition = match self.peek() {
                Some(Token::Negative) => { self.consume(); Condition::IsNegative }
                Some(Token::Positive) => { self.consume(); Condition::IsPositive }
                _ => {
                    // No recognised condition — apply then_op normally and return
                    if let (Some(op), Some(val)) = (then_op, then_val) {
                        expr = Expression::BinOp {
                            op, left: Box::new(expr), right: Box::new(val)
                        };
                    }
                    return Some(expr);
                }
            };

            // Build Conditional with the guarded op stored separately
            return Some(Expression::Conditional {
                base: Box::new(expr),
                condition,
                guarded_op: then_op,
                guarded_val: then_val.map(Box::new),
            });
        }

        // No unless — apply then_op normally
        if let (Some(op), Some(val)) = (then_op, then_val) {
            expr = Expression::BinOp {
                op, left: Box::new(expr), right: Box::new(val)
            };
        }

        Some(expr)
    }

    // After "then", we expect an infix or prefix operation applied to the
    // accumulated expression so far as the implicit left operand.
    // "then multiply by 3"  → Mul(expr, 3)
    // "then add 5"          → Add(expr, 5)
    fn parse_then_continuation(&mut self, left: Expression) -> Option<Expression> {
        match self.peek()?.clone() {
            Token::Op(Operation::Multiply) => {
                self.consume();
                self.skip_fillers();
                let right = self.parse_primary()?;
                Some(Expression::BinOp {
                    op: Operation::Multiply,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Token::Op(Operation::Divide) => {
                self.consume();
                self.skip_fillers();
                let right = self.parse_primary()?;
                Some(Expression::BinOp {
                    op: Operation::Divide,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Token::Op(Operation::Add) => {
                self.consume();
                self.skip_fillers();
                let right = self.parse_rhs()?;
                Some(Expression::BinOp {
                    op: Operation::Add,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Token::Op(Operation::Subtract) => {
                self.consume();
                self.skip_fillers();
                let (l, r) = self.parse_subtract_args()?;
                // Here left was the accumulated expr; wrap it with the subtraction
                // For "then subtract X from Y", Y-X is new standalone — rare case
                Some(Expression::BinOp {
                    op: Operation::Subtract,
                    left: Box::new(l),
                    right: Box::new(r),
                })
            }
            _ => Some(left), // nothing recognised, return as-is
        }
    }

    // "unless the result is negative/positive/zero"
    fn parse_unless(&mut self, main: Expression) -> Option<Expression> {
        self.consume(); // eat Unless
        // skip "the result is" or any subset of those words
        while matches!(self.peek(),
            Some(Token::The) | Some(Token::Result) | Some(Token::Is) | Some(Token::Of)
        ) {
            self.consume();
        }
        let condition = match self.peek() {
            Some(Token::Negative) => { self.consume(); Condition::IsNegative }
            Some(Token::Positive) => { self.consume(); Condition::IsPositive }
            Some(Token::Number(n)) if *n == 0.0 => { self.consume(); Condition::IsZero }
            _ => return Some(main),
        };
        // Any "then <op> <val>" after the condition sets the alternative
        // Default: when condition met, result becomes 0
        Some(Expression::Conditional {
            base: Box::new(main),
            condition,
            guarded_op: Some(Operation::Multiply),
            guarded_val: Some(Box::new(Expression::Number(0.0))),
        })
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
                    if let Some(next) = self.parse_primary() {
                        acc = Expression::BinOp {
                            op: op.clone(),
                            left: Box::new(acc),
                            right: Box::new(next),
                        };
                    } else { break; }
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
            | Some(Token::By) => { self.consume(); }  // ← removed The/Result/Of
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
            let condition_met = match condition {
                Condition::IsNegative => base_val < 0.0,
                Condition::IsPositive => base_val > 0.0,
                Condition::IsZero     => base_val == 0.0,
            };
            if condition_met {
                Ok(base_val)  // condition fired — skip the guarded op, return base
            } else {
                // condition not fired — apply the guarded op
                match (guarded_op, guarded_val) {
                    (Some(op), Some(val)) => {
                        let v = evaluate(val)?;
                        match op {
                            Operation::Multiply => Ok(base_val * v),
                            Operation::Add      => Ok(base_val + v),
                            Operation::Subtract => Ok(base_val - v),
                            Operation::Divide   => {
                                if v != 0.0 { Ok(base_val / v) }
                                else { Err("Division by zero".to_string()) }
                            }
                            Operation::Power    => Ok(base_val.powf(v)),
                        }
                    }
                    _ => Ok(base_val),
                }
            }
        }
    }
}