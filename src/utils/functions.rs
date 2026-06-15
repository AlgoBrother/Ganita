pub fn factorial(x: f64) -> f64 {
    if x < 0.0 {
        panic!("Factorial is not defined for negative numbers");
    } else if x == 0.0 || x == 1.0 {
        1.0
    } else {
        return x * factorial(x - 1.0);
    }
}

pub fn quadratic_solver(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
    let discriminant = (b * b) - 4.0 * a * c;
    if discriminant < 0.0 {
        return Err("No real roots".to_string());
    }
    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
    Ok((root1, root2))
}