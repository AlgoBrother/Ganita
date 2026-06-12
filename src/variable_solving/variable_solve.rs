// A seperate file for handling variable solving logic. 

use std::collections::HashMap;

use crate::ast::{Expression, Operation, evaluate_with_context};

fn contains_var(expr: &Expression, var: &str) -> bool {
    match expr {
        Expression::Variable(name) => name == var,
        Expression::BinOp { left, right, .. } => contains_var(left, var) || contains_var(right, var),
        Expression::TrigoOp { operand, .. } => contains_var(operand, var),
        _ => false,
    }
}

pub fn solve_linear(expr: Expression, var: &str, target: f64) -> Result<f64, String> {
    match expr {
        Expression::Variable(name) if name == var => Ok(target), // If the expression is just the variable we want to solve for, then we can directly return the target value.

        Expression::BinOp { op: Operation::Add, left, right } =>{
            let left_contains_var = contains_var(&left, var);
            let right_contains_var = contains_var(&right, var);
             // x + b = target → x = target - b
            if left_contains_var && !right_contains_var {
                let b = evaluate_with_context(&right, &mut HashMap::new())?; 
                solve_linear(*left, var, target - b)
            }else if right_contains_var && !left_contains_var {
                let b = evaluate_with_context(&left, &mut HashMap::new())?;
                solve_linear(*right, var, target - b)
            } else {
                Err("Cannot solve: variable appears on both sides".to_string())
            }
        }

        Expression::BinOp { op: Operation::Multiply, left, right } => {
            let left_contains_var = contains_var(&left, var);
            let right_contains_var = contains_var(&right, var);
            // a * x = target → x = target / a
            if left_contains_var && !right_contains_var {
                let a = evaluate_with_context(&right, &mut HashMap::new())?;
                if a.abs() < 1e-10 { return Err("Cannot divide by zero coefficient".to_string()); }
                solve_linear(*left, var, target / a)
            } else if right_contains_var && !left_contains_var {
                let a = evaluate_with_context(&left, &mut HashMap::new())?;
                if a.abs() < 1e-10 { return Err("Cannot divide by zero coefficient".to_string()); }
                solve_linear(*right, var, target / a)
            } else {
                Err("Cannot solve: nonlinear".to_string())
            }
        }
        
        Expression::BinOp { op: Operation::Subtract, left, right } => {
            let left_contains_var = contains_var(&left, var);
            let right_contains_var = contains_var(&right, var);
            if left_contains_var && !right_contains_var {
                // x - b = target → x = target + b
                let b = evaluate_with_context(&right, &mut HashMap::new())?;
                solve_linear(*left, var, target + b)
            } else if right_contains_var && !left_contains_var {
                // a - x = target → x = a - target
                let a = evaluate_with_context(&left, &mut HashMap::new())?;
                solve_linear(*right, var, a - target)
            } else {
                Err("Cannot solve: variable appears on both sides".to_string())
            }
        }

        _ => Err(format!("Variable {} cannot be isolated in the provided expression: {:#?}", var, expr))
    }
}



// Newton's method for solving nonlinear equations. This is a fallback when linear solving fails. It requires an initial guess and iteratively refines it.
pub fn solve_numerically(
    expr: &Expression,  // LHS - RHS as a single expression
    var: &str,
    initial_guess: f64,
) -> Result<f64, String> {
    let mut x = initial_guess;
    let h = 1e-7; // step for numerical derivative
    
    for _ in 0..1000 {
        let mut context_x = HashMap::new();
        context_x.insert(var.to_string(), x);
        
        let fx = evaluate_with_context(expr, &mut context_x)?;
        if fx.abs() < 1e-9 { return Ok(x); }
        
        let mut context_xh = HashMap::new();
        context_xh.insert(var.to_string(), x + h);
        let fxh = evaluate_with_context(expr, &mut context_xh)?;
        
        let derivative = (fxh - fx) / h;
        if derivative.abs() < 1e-10 {
            return Err("Newton's method failed: derivative too small".to_string());
        }
        
        x = x - fx / derivative;
    }
    
    Err("Newton's method did not converge".to_string())
}