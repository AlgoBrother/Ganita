pub fn factorial(x: f64) -> f64 {
    if x < 0.0 {
        panic!("Factorial is not defined for negative numbers");
    } else if x == 0.0 || x == 1.0 {
        1.0
    } else {
        return x * factorial(x - 1.0);
    }
}