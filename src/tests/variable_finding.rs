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

mod variable_finding {
    use super::*;

    #[test]
    fn test_variable_finding() {
        assert_approx_eq(compute("solve x in 2 * x = 10"), 5.0);
        assert_approx_eq(compute("Solve 1 + 1"), 2.0);
        assert_approx_eq(compute("solve x in (((x + 5) * 2) - 10) = 20"), 10.0);
        assert_approx_eq(compute("solve x in (((((x + 1) + 2) + 3) + 4) + 5) = 20"), 5.0);
        assert_approx_eq(compute("solve x in (x + 5) * 2 = 20"), 5.0);
        // assert_approx_eq(compute("x  = 5 + 10, what is the value of x"), 15.0);
        // assert_approx_eq(compute("What is the value of x if x + 5 = 10"), 5.0);
    }
}