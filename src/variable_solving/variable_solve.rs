// A seperate file for handling variable solving logic. 
use std::collections::HashMap;

use crate::{ast::{Expression, Operation, evaluate_with_context}, trignometry::trigo::{AngleType, TrigonometricFunction}};

type VariableContext = HashMap<String, f64>; // A context for variable values during evaluation and context lookups to avoid repeated allocations

fn contains_var(expr: &Expression, var: &str) -> bool {
    match expr {
        Expression::Variable(name) => name == var,
        Expression::BinOp { left, right, .. } => contains_var(left, var) || contains_var(right, var),
        Expression::TrigoOp { operand, .. } => contains_var(operand, var),
        _ => false,
    }
}

fn contains_trig(expr: &Expression) -> bool {
    match expr {
        Expression::TrigoOp { .. } => true,
        Expression::BinOp { left, right, .. } => {
            contains_trig(left) || contains_trig(right)
        }
        Expression::UnaryOp { operand, .. } => {
            contains_trig(operand)
        }
        Expression::Comparison { left, right, .. } => {
            contains_trig(left) || contains_trig(right)
        }
        Expression::Conditional { base, guarded_val, .. } => {
            contains_trig(base) || 
            guarded_val.as_ref().map(|v| contains_trig(v)).unwrap_or(false)
        }
        Expression::Convert { operand, .. } => {
            contains_trig(operand)
        }
        // These expression types can't contain trig functions
        Expression::Number(_) => false,
        Expression::Variable(_) => false,
        Expression::Assign { .. } => false,
        Expression::Solve { .. } => false,
        Expression::Multivariable_Solve {..} => false,
    }
}

pub fn solve_linear(expr: Expression, var: &str, target: f64, context: &VariableContext) -> Result<f64, String> {
    match expr {
        Expression::Variable(name) if name == var => Ok(target),

        Expression::BinOp { op: Operation::Add, left, right } => {
            let left_contains_var = contains_var(&left, var);
            let right_contains_var = contains_var(&right, var);
            if left_contains_var && !right_contains_var {
                let b = evaluate_with_context(&right, context)?;
                solve_linear(*left, var, target - b, context)
            } else if right_contains_var && !left_contains_var {
                let b = evaluate_with_context(&left, context)?;
                solve_linear(*right, var, target - b, context)
            } else {
                Err("Cannot solve algebraically: variable appears on both sides".to_string())
            }
        }

        Expression::BinOp { op: Operation::Subtract, left, right } => {
            let left_contains_var = contains_var(&left, var);
            let right_contains_var = contains_var(&right, var);
            if left_contains_var && !right_contains_var {
                let b = evaluate_with_context(&right, context)?;
                solve_linear(*left, var, target + b, context)
            } else if right_contains_var && !left_contains_var {
                let a = evaluate_with_context(&left, context)?;
                solve_linear(*right, var, a - target, context)
            } else {
                Err("Cannot solve algebraically: variable appears on both sides".to_string())
            }
        }

        Expression::BinOp { op: Operation::Multiply, left, right } => {
            let left_contains_var = contains_var(&left, var);
            let right_contains_var = contains_var(&right, var);
            if left_contains_var && !right_contains_var {
                let a = evaluate_with_context(&right, context)?;
                if a.abs() < 1e-10 { return Err("Cannot divide by zero coefficient".to_string()); }
                solve_linear(*left, var, target / a, context)
            } else if right_contains_var && !left_contains_var {
                let a = evaluate_with_context(&left, context)?;
                if a.abs() < 1e-10 { return Err("Cannot divide by zero coefficient".to_string()); }
                solve_linear(*right, var, target / a, context)
            } else {
                Err("Cannot solve algebraically: nonlinear multiplication".to_string())
            }
        }

        // Algebraic solving for Exponents (x^2 = 4 -> x = 2) is more complex than the other operations because it can be nonlinear and have multiple solutions. {currenly takin principle root, will be handled for multiple solutions in the future}
        Expression::BinOp { op: Operation::Power, left, right } => {
            let left_contains_var = contains_var(&left, var);
            let right_contains_var = contains_var(&right, var);
            
            if left_contains_var && !right_contains_var {
                // x ^ n = target -> x = target ^ (1/n)
                let exponent = evaluate_with_context(&right, context)?;
                if exponent == 0.0 { return Err("Math error: zero exponent".to_string()); }
                if target < 0.0 && exponent % 2.0 == 0.0 {
                    return Err("Math error: Even root of a negative number".to_string());
                }
                
                let new_target = target.powf(1.0 / exponent);
                solve_linear(*left, var, new_target, context)
            } else if right_contains_var && !left_contains_var {
                // n ^ x = target -> x = ln(target) / ln(n)
                let base = evaluate_with_context(&left, context)?;
                if base <= 0.0 || base == 1.0 { return Err("Math error: Invalid log base".to_string()); }
                if target <= 0.0 { return Err("Math error: Log of non-positive number".to_string()); }
                
                let new_target = target.ln() / base.ln();
                solve_linear(*right, var, new_target, context)
            } else {
                Err("Cannot solve algebraically: variable in both base and exponent".to_string())
            }
        }

        // Direct inverse trig isolation: Instead of relying on Newton method, we can directly apply inverse trig functions when the variable is inside a single trig function. This is much more efficient and accurate for these cases.
        Expression::TrigoOp { func, operand, unit } => {
            if !contains_var(&operand, var) {
                return Err("Variable not found in trigonometric function".to_string());
            }

            let pi = std::f64::consts::PI;
            let new_target = match func {
                // Standard Trig: Target is a ratio, output is an angle
                TrigonometricFunction::Sine => {
                    let rad = target.asin();
                    if unit == AngleType::Degrees { rad * 180.0 / pi } else { rad }
                }
                TrigonometricFunction::Cosine => {
                    let rad = target.acos();
                    if unit == AngleType::Degrees { rad * 180.0 / pi } else { rad }
                }
                TrigonometricFunction::Tangent => {
                    let rad = target.atan();
                    if unit == AngleType::Degrees { rad * 180.0 / pi } else { rad }
                }
                // Inverse Trig: Target is an angle, output is a ratio
                TrigonometricFunction::InverseSine => {
                    let rad = if unit == AngleType::Degrees { target * pi / 180.0 } else { target };
                    rad.sin()
                }
                TrigonometricFunction::InverseCosine => {
                    let rad = if unit == AngleType::Degrees { target * pi / 180.0 } else { target };
                    rad.cos()
                }
                TrigonometricFunction::InverseTangent => {
                    let rad = if unit == AngleType::Degrees { target * pi / 180.0 } else { target };
                    rad.tan()
                }
                _ => return Err(format!("Inverse for {:?} not yet supported algebraically", func)),
            };

            if new_target.is_nan() {
                return Err("Math error: No real solution for this domain".to_string());
            }

            // Snap to clean numbers to avoid floating point drift (e.g., 29.999999999999996 -> 30.0)
            let cleaned_target = (new_target * 10000.0).round() / 10000.0;

            solve_linear(*operand, var, cleaned_target, context)
        }

        _ => Err(format!("Cannot isolate variable algebraically in: {:#?}", expr))
    }
}


// Newton's method for solving nonlinear equations. This is a fallback when linear solving fails. It requires an initial guess and iteratively refines it.
pub fn solve_numerically(
    expr: &Expression,
    var: &str,
    initial_guess: f64,
    context: &VariableContext
) -> Result<f64, String> {
    // Try the provided initial guess first (fast path)
    if let Ok(result) = newton_single(expr, var, initial_guess, context) {
        return Ok(result);
    }
    
    // Only try multiple guesses if the first one fails AND we have trig
    if contains_trig(expr) {
        let guesses = [0.0, 30.0, 45.0, 60.0, 90.0, 180.0, 270.0, -30.0, -45.0, -60.0, -90.0];
        
        for &guess in &guesses {
            if (guess - initial_guess).abs() < 0.1 {
                continue;
            }
            
            if let Ok(result) = newton_single(expr, var, guess, context) {
                return Ok(result);
            }
        }
    }
    
    Err("Newton's method did not converge".to_string())
}

fn normalize_to_principal(result: f64, expr: &Expression) -> f64 {
    // For inverse trig-like solutions, normalize to common ranges
    let mut normalized = result;
    
    // Normalize to [-360, 360] first
    while normalized > 360.0 { normalized -= 360.0; }
    while normalized < -360.0 { normalized += 360.0; }
    
    // For sine equations, prefer [-90, 90]
    // For cosine equations, prefer [0, 180]  
    // For tangent equations, prefer [-90, 90]
    
    normalized
}

fn snap_to_standard_angle(result: f64) -> f64 {
    // Round to 4 decimal places - catches floating point noise
    let rounded = (result * 10000.0).round() / 10000.0;
    
    // Check common fractions/decimals that might appear
    let nearest_int = rounded.round();
    if (rounded - nearest_int).abs() < 1e-6 {
        return nearest_int;
    }
    
    // Check half-integers: -30.5, 45.5, etc.
    let nearest_half = (rounded * 2.0).round() / 2.0;
    if (rounded - nearest_half).abs() < 1e-6 {
        return nearest_half;
    }
    
    rounded
}

fn newton_single(
    expr: &Expression,
    var: &str,
    initial_guess: f64,
    context: &VariableContext, 
) -> Result<f64, String> {
    let mut x = initial_guess;
    let h = 1e-7;
    let mut local_context = context.clone(); // Start with known variables
    
    for _ in 0..50 {
        local_context.insert(var.to_string(), x);
        let fx = evaluate_with_context(expr, &local_context)?;
        
        if fx.abs() < 1e-9 {
            return Ok(snap_to_standard_angle(normalize_to_principal(x, expr)));
        }
        
        local_context.insert(var.to_string(), x + h);
        let fxh = evaluate_with_context(expr, &local_context)?;
        
        let derivative = (fxh - fx) / h;
        
        if derivative.abs() < 1e-10 {
            x += 0.1;
            continue;
        }
        
        let step = fx / derivative;
        x = x - step.clamp(-50.0, 50.0);
        
        if x.abs() > 10000.0 {
            return Err("Solution diverged".to_string());
        }
    }
    
    Err("Newton's method did not converge".to_string())
}

// In variable_solve.rs
pub fn solve_system_numerically(
    equations: &[Expression],
    vars: &[String],
    context: &VariableContext,
) -> Result<HashMap<String, f64>, String> {
    
    // FAST PATH: 2D Newton-Raphson for 2 variables / 2 equations
    if vars.len() == 2 && equations.len() >= 2 {
        if let Ok(res) = solve_2d_newton_raphson(equations, vars, context) {
            return Ok(res);
        }
    }

    // FALLBACK: Gradient Descent for >2 variables
    let mut current_guesses: HashMap<String, f64> = vars.iter()
        .map(|v| (v.clone(), 1.0))
        .collect();

    let learning_rate = 0.01;
    let h = 1e-6;

    for _ in 0..5000 {
        let mut local_context = context.clone();
        local_context.extend(current_guesses.clone());

        let mut total_error = 0.0;
        for eq in equations {
            if let Expression::Comparison { left, right, .. } = eq {
                let l_val = evaluate_with_context(left, &local_context)?;
                let r_val = evaluate_with_context(right, &local_context)?;
                total_error += (l_val - r_val).powi(2);
            }
        }

        if total_error < 1e-9 {
            // Clean up floating point noise
            for val in current_guesses.values_mut() {
                *val = (*val * 10000.0).round() / 10000.0;
            }
            return Ok(current_guesses);
        }

        let mut gradients = HashMap::new();
        for var in vars {
            let mut stepped_context = local_context.clone();
            stepped_context.insert(var.clone(), current_guesses[var] + h);

            let mut stepped_error = 0.0;
            for eq in equations {
                if let Expression::Comparison { left, right, .. } = eq {
                    let l_val = evaluate_with_context(left, &stepped_context)?;
                    let r_val = evaluate_with_context(right, &stepped_context)?;
                    stepped_error += (l_val - r_val).powi(2);
                }
            }

            let grad = (stepped_error - total_error) / h;
            gradients.insert(var.clone(), grad);
        }

        for var in vars {
            let guess = current_guesses.get_mut(var).unwrap();
            let step = learning_rate * gradients[var];
            *guess -= step.clamp(-10.0, 10.0);
        }
    }

    Err("System did not converge to a solution".to_string())
}

// Helper: Evaluates an equation to find its error (LHS - RHS = 0)
fn get_eq_error(eq: &Expression, ctx: &VariableContext) -> Result<f64, String> {
    if let Expression::Comparison { left, right, .. } = eq {
        let l = evaluate_with_context(left, ctx)?;
        let r = evaluate_with_context(right, ctx)?;
        Ok(l - r)
    } else {
        Err("Not a comparison".to_string())
    }
}

// Jacobian Matrix solver for 2x2 systems
fn solve_2d_newton_raphson(
    equations: &[Expression],
    vars: &[String],
    context: &VariableContext,
) -> Result<HashMap<String, f64>, String> {
    let mut x = 1.0;
    let mut y = 1.0;
    let h = 1e-6;
    let var_x = &vars[0];
    let var_y = &vars[1];
    let eq1 = &equations[0];
    let eq2 = &equations[1];

    let mut local_context = context.clone();

    for _ in 0..100 {
        local_context.insert(var_x.clone(), x);
        local_context.insert(var_y.clone(), y);

        let f1 = get_eq_error(eq1, &local_context)?;
        let f2 = get_eq_error(eq2, &local_context)?;

        // Converged!
        if f1.abs() < 1e-9 && f2.abs() < 1e-9 {
            let mut res = HashMap::new();
            res.insert(var_x.clone(), (x * 10000.0).round() / 10000.0);
            res.insert(var_y.clone(), (y * 10000.0).round() / 10000.0);
            return Ok(res);
        }

        // Calculate Partial Derivatives for the Jacobian Matrix
        local_context.insert(var_x.clone(), x + h);
        let f1_dx = (get_eq_error(eq1, &local_context)? - f1) / h;
        let f2_dx = (get_eq_error(eq2, &local_context)? - f2) / h;
        local_context.insert(var_x.clone(), x); // Reset x

        local_context.insert(var_y.clone(), y + h);
        let f1_dy = (get_eq_error(eq1, &local_context)? - f1) / h;
        let f2_dy = (get_eq_error(eq2, &local_context)? - f2) / h;
        local_context.insert(var_y.clone(), y); // Reset y

        // Determinant of Jacobian
        let det = (f1_dx * f2_dy) - (f1_dy * f2_dx);
        if det.abs() < 1e-12 {
            return Err("Singular Jacobian (Parallel lines or local minima)".to_string());
        }

        // Inverse Jacobian multiplication to find step size
        let dx = (f1 * f2_dy - f2 * f1_dy) / det;
        let dy = (f1_dx * f2 - f2_dx * f1) / det;

        x -= dx;
        y -= dy;
    }
    
    Err("Newton 2D failed to converge".to_string())
}