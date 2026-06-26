// Seperate code for testing variable finding logic 
#[cfg(test)]
use crate::math_engine::compute;

#[cfg(test)]
fn assert_approx_eq(left: Result<f64, String>, right: f64) {
    match left {
        Ok(val) => {
            // Rounds to 4 decimal places: e.g., 0.5773502 -> 0.5774
            let rounded_left = (val * 10000.0).round() / 10000.0;
            let rounded_right = (right * 10000.0).round() / 10000.0;
            assert_eq!(rounded_left, rounded_right);
        }
        Err(e) => panic!("Expected Ok, got Err: {:?}", e),
    }
}

#[cfg(test)]
mod variable_finding {
    use super::*;

    #[test]
    fn test_variable_finding_easy() {
        // Direct assignment / basic equality
        assert_approx_eq(compute("solve x in x = 42"), 42.0);
        
        // Single-step basic arithmetic
        assert_approx_eq(compute("solve x in x + 5 = 15"), 10.0);
        assert_approx_eq(compute("find x where x - 8 = 2"), 10.0);
        assert_approx_eq(compute("solve x in 4 * x = 24"), 6.0);
        assert_approx_eq(compute("find x in x / 3 = 7"), 21.0); 
    }

    #[test]
    fn test_variable_finding_medium() {
        // Order of operations and parentheses
        assert_approx_eq(compute("solve x in 3 * x + 4 = 19"), 5.0);
        assert_approx_eq(compute("solve x in (x + 5) * 2 = 20"), 5.0);
        
        // Deeply nested parentheses
        assert_approx_eq(compute("solve x in (((x + 2) * 3) - 4) = 14"), 4.0);
        assert_approx_eq(compute("solve x in (((((x + 1) + 2) + 3) + 4) + 5) = 20"), 5.0);
        
        // Handling negative numbers and decimal results
        assert_approx_eq(compute("solve x in 10 - x = 25"), -15.0);
        assert_approx_eq(compute("solve x in x * 0.5 = 2.5"), 5.0);
    }

    #[test]
    fn test_solve_factorial_addition() {
        assert_approx_eq(compute("solve x in x + 3! = 10"), 4.0);
    }

    #[test]
    fn test_solve_factorial_division() {
        assert_approx_eq(compute("solve x in x = 5! / 2"), 60.0);
    }

    #[test]
    fn test_solve_with_exponent() {
        assert_approx_eq(compute("solve x in x * 2 ^ 3 = 32"), 4.0);
    }

    #[test]
    fn test_solve_with_trigonometry() {
        assert_approx_eq(compute("solve x in x + sin 30 degrees = 5.5"), 5.0);
    }

    #[test]
    fn test_solve_complex_rhs_expression() {
        assert_approx_eq(compute("solve x in 2 * x = 10 + 5 * 2 - 4"), 8.0);
    }

    #[test]
    fn test_suffix_find_x() {
        assert_approx_eq(compute("x = 5 + 10, find x"), 15.0);
    }

    #[test]
    fn test_suffix_solve_x() {
        assert_approx_eq(compute("x * 3 = 27, solve x"), 9.0);
    }

    #[test]
    fn test_prefix_what_is_value_of_x() {
        assert_approx_eq(compute("What is the value of x if x + 5 = 10"), 5.0);
    }

    #[test]
    fn test_natural_language_subtract_from_x() {
        assert_approx_eq(compute("find x where subtract 3 from x equals 7"), 10.0);
    }

    #[test]
    fn test_solve_shifted_sine() {
        assert_approx_eq(
            compute("solve x in sin(x + 30) = 0.5"),
            0.0
        );
    }
}

#[cfg(test)]
mod multivariable_solving_tests {
    use crate::math_engine::compute;

    // Helper function for 4 decimal place precision matching
    fn assert_approx_eq(left: Result<f64, String>, right: f64) {
        match left {
            Ok(val) => {
                let rounded_left = (val * 10000.0).round() / 10000.0;
                let rounded_right = (right * 10000.0).round() / 10000.0;
                assert_eq!(rounded_left, rounded_right);
            }
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }

    // ── Tier 1: Sequential Variable Chaining ───────────────────
    
    #[test]
    fn test_sequential_dependency() {
        // Core context lookup: y depends on x, z depends on both
        assert_approx_eq(compute("Let x = 5, Let y = x + 10, solve z in z = x + y"), 20.0);
    }

    #[test]
    fn test_variable_redefinition_scoping() {
        // Validates that variables can safely mutate or overwrite state down the sequence chain
        assert_approx_eq(compute("Let a = 10, Let b = 20, solve x in x + a = b, Let a = 5, solve x in x + a = b"), 15.0);
    }

    #[test]
    fn test_recursive_context_assignment() {
        // Self-referential variable updates
        assert_approx_eq(compute("Let x = 10, Let x = x + 5, solve y in y = x * 2"), 30.0);
    }

    // ── Tier 2: Linear Simultaneous Systems ────────────────────
    
    #[test]
    fn test_simultaneous_linear_substitution() {
        // Classic system: y = 2x and x + y = 9 -> 3x = 9 -> x = 3
        // This validates if your pending equation accumulator functions properly
        assert_approx_eq(compute("y = 2 * x, x + y = 9, solve for x"), 3.0);
    }

    #[test]
    fn test_simultaneous_linear_two_variables() {
        // x + y = 10, x - y = 2 -> 2x = 12 -> x = 6
        assert_approx_eq(compute("x + y = 10, x - y = 2, solve for x"), 6.0);
    }

    // ── Tier 3: Non-Linear Multivariable Systems ───────────────
    
    #[test]
    fn test_mixed_multivariable_trig() {
        // Combines structural context mapping with your recursive trigonometric tracking
        assert_approx_eq(compute("Let angle = 30, solve x in sin(x + angle) = 0.5"), 0.0);
    }

    #[test]
    fn test_nonlinear_simultaneous_system() {
        // Optimization fallback test: x^2 + y = 7 and y - x = 1
        // Substituting y = x + 1 -> x^2 + x - 6 = 0 -> positive real root x = 2
        assert_approx_eq(compute("x ^ 2 + y = 7, y - x = 1, solve for x"), 2.0);
    }

    // ── Tier 4: Error Scenarios ────────────────────────────────
    
    #[test]
    fn test_underdetermined_system_error() {
        // Infinite solutions: engine should gracefully error or fail to converge
        assert!(compute("x + y = 10, solve x").is_err());
    }

    #[test]
    fn test_unsolvable_contradictory_system() {
        // Parallel lines: no real solution possible
        assert!(compute("x + y = 5, x + y = 10, solve x").is_err());
    }
}