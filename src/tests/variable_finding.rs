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