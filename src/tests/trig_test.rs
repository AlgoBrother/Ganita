#[cfg(test)]
use crate::trignometry::trigo::PI;

// ═══════════════════════════════════════════════════════════════
// References from NCERT Chapter 3 — Trigonometric Functions (basic questions)
// Test cases for NLP_Math_Engine
// Organised by what the engine can handle NOW vs what needs work 
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
// Used whenever PI is involved in test queries to check for tolerance.
fn approx_eps(a: f64, b: f64, eps: f64) -> bool {
    // returns true if a and b are approximately equal within the given epsilon tolerance
    (a - b).abs() < eps
}

#[cfg(test)]
mod trig_tests {
    use super::*;
    use crate::math_engine::compute;

    const EPSILON: f64 = 1e-9;
    fn approx(a: f64, b: f64) -> bool { (a - b).abs() < EPSILON }
    fn approx_3dp(a: f64, b: f64) -> bool { (a - b).abs() < 1e-3 }

    // ─────────────────────────────────────────────────────────────
    // TIER 1: Direct evaluation — ENGINE CAN HANDLE NOW
    // Section 3.3 standard angle table (degrees)
    // ─────────────────────────────────────────────────────────────

    // sin values
    #[test] fn sin_0()  { assert!(approx(compute("sin 0 degrees").unwrap(),   0.0)); }
    #[test] fn sin_30() { assert!(approx(compute("sin 30 degrees").unwrap(),  0.5)); }
    #[test] fn sin_45() { assert!(approx(compute("sin 45 degrees").unwrap(),  2f64.sqrt() / 2.0)); }
    #[test] fn sin_60() { assert!(approx(compute("sin 60 degrees").unwrap(),  3f64.sqrt() / 2.0)); }
    #[test] fn sin_90() { assert!(approx(compute("sin 90 degrees").unwrap(),  1.0)); }
    #[test] fn sin_180(){ assert!(approx(compute("sin 180 degrees").unwrap(), 0.0)); }
    #[test] fn sin_270(){ assert!(approx(compute("sin 270 degrees").unwrap(),-1.0)); }
    #[test] fn sin_360(){ assert!(approx(compute("sin 360 degrees").unwrap(), 0.0)); }

    // cos values
    #[test] fn cos_0()  { assert!(approx(compute("cos 0 degrees").unwrap(),   1.0)); }
    #[test] fn cos_30() { assert!(approx(compute("cos 30 degrees").unwrap(),  3f64.sqrt() / 2.0)); }
    #[test] fn cos_45() { assert!(approx(compute("cos 45 degrees").unwrap(),  2f64.sqrt() / 2.0)); }
    #[test] fn cos_60() { assert!(approx(compute("cos 60 degrees").unwrap(),  0.5)); }
    #[test] fn cos_90() { assert!(approx(compute("cos 90 degrees").unwrap(),  0.0)); }
    #[test] fn cos_180(){ assert!(approx(compute("cos 180 degrees").unwrap(),-1.0)); }
    #[test] fn cos_270(){ assert!(approx(compute("cos 270 degrees").unwrap(), 0.0)); }
    #[test] fn cos_360(){ assert!(approx(compute("cos 360 degrees").unwrap(), 1.0)); }

    // tan values
    #[test] fn tan_0()  { assert!(approx(compute("tan 0 degrees").unwrap(),   0.0)); }
    #[test] fn tan_30() { assert!(approx(compute("tan 30 degrees").unwrap(),  1.0 / 3f64.sqrt())); }
    #[test] fn tan_45() { assert!(approx(compute("tan 45 degrees").unwrap(),  1.0)); }
    #[test] fn tan_60() { assert!(approx(compute("tan 60 degrees").unwrap(),  3f64.sqrt())); }
    #[test] fn tan_90_undefined() { assert!(compute("tan 90 degrees").is_err()); }
    #[test] fn tan_180(){ assert!(approx(compute("tan 180 degrees").unwrap(), 0.0)); }

    // reciprocal functions — sec 3.3
    #[test] fn cosec_30(){ assert!(approx(compute("cosecant of 30 degrees").unwrap(), 2.0)); }
    #[test] fn cosec_90(){ assert!(approx(compute("cosec 90 degrees").unwrap(),       1.0)); }
    #[test] fn sec_0()   { assert!(approx(compute("secant of 0 degrees").unwrap(),    1.0)); }
    #[test] fn sec_60()  { assert!(approx(compute("sec 60 degrees").unwrap(),         2.0)); }
    #[test] fn cot_45()  { assert!(approx(compute("cotangent of 45 degrees").unwrap(),1.0)); }
    #[test] fn cot_60()  { assert!(approx(compute("cot 60 degrees").unwrap(),         1.0 / 3f64.sqrt())); }

    // undefined reciprocals
    #[test] fn cosec_0_undefined()  { assert!(compute("cosec 0 degrees").is_err()); }
    #[test] fn cosec_180_undefined(){ assert!(compute("cosecant of 180 degrees").is_err()); }
    #[test] fn sec_90_undefined()   { assert!(compute("sec 90 degrees").is_err()); }
    #[test] fn cot_0_undefined()    { assert!(compute("cot 0 degrees").is_err()); }

    // natural language forms
    #[test] fn sine_of_30()    { assert!(approx(compute("sine of 30 degrees").unwrap(), 0.5)); }
    #[test] fn cosine_of_60()  { assert!(approx(compute("cosine of 60 degrees").unwrap(), 0.5)); }
    #[test] fn tangent_of_45() { assert!(approx(compute("tangent of 45 degrees").unwrap(), 1.0)); }

    // negative angles — sec 3.3.1
    // sin(-x) = -sin(x)
    #[test] fn sin_neg_30()  { assert!(approx(compute("sin -30 degrees").unwrap(), -0.5)); }
    #[test] fn sin_neg_90()  { assert!(approx(compute("sin -90 degrees").unwrap(), -1.0)); }
    // cos(-x) = cos(x)
    #[test] fn cos_neg_30()  { assert!(approx(compute("cos -30 degrees").unwrap(), 3f64.sqrt() / 2.0)); }
    #[test] fn cos_neg_90()  { assert!(approx(compute("cos -90 degrees").unwrap(), 0.0)); }

    // ─────────────────────────────────────────────────────────────
    // TIER 2: Arithmetic ON trig results — ENGINE CAN HANDLE NOW
    // These test that trig values feed into arithmetic correctly
    // ─────────────────────────────────────────────────────────────

    // Pythagorean identity numerically: sin²x + cos²x = 1
    // sec 3.3: cos²x + sin²x = 1
    #[test]
    fn pythagorean_identity_30() {
        // sin(30)^2 + cos(30)^2 = 1
        let result = compute("sin 30 degrees ^ 2 + cos 30 degrees ^ 2").unwrap();
        assert!(approx(result, 1.0),
            "sin²30 + cos²30 = {} (expected 1.0)", result);
    }

    #[test]
    fn pythagorean_identity_45() {
        let result = compute("sin 45 degrees ^ 2 + cos 45 degrees ^ 2").unwrap();
        assert!(approx(result, 1.0));
    }

    #[test]
    fn pythagorean_identity_60() {
        let result = compute("sin 60 degrees ^ 2 + cos 60 degrees ^ 2").unwrap();
        assert!(approx(result, 1.0));
    }

    // Exercise 3.3 Q1: sin²(π/6) + cos²(π/3) - tan²(π/4) = -1/2
    // = sin²30 + cos²60 - tan²45 = 0.25 + 0.25 - 1 = -0.5
    #[test]
    fn ex33_q1() {
        // raw: "sin 30 degrees ^ 2 + cos 60 degrees ^ 2 - tan 45 degrees ^ 2"
        let result = compute("sin 30 degrees ^ 2 + cos 60 degrees ^ 2 - tan 45 degrees ^ 2").unwrap();
        assert!(approx(result, -0.5),
            "sin²30 + cos²60 - tan²45 = {} (expected -0.5)", result);
    }

    // Example 11: sin 15° = sin(45° - 30°) = (√6 - √2) / 4 ≈ 0.2588
    #[test]
    fn sin_15_degrees() {
        let result = compute("sin 15 degrees").unwrap();
        assert!(approx_3dp(result, 0.2588),
            "sin(15°) = {} (expected ≈ 0.2588)", result);
    }

    // Example 9: cos(-1710°) = cos(90°) = 0
    // -1710 + 5*360 = -1710 + 1800 = 90
    #[test]
    fn cos_neg_1710() {
        let result = compute("cos -1710 degrees").unwrap();
        assert!(approx(result, 0.0),
            "cos(-1710°) = {} (expected 0)", result);
    }

    // Example 8: sin(31π/3) = sin(π/3) = √3/2
    // 31π/3 in radians
    #[test]
    fn sin_31pi_over_3_radians() {
        use std::f64::consts::PI;
        let angle = 31.0 * PI / 3.0;
        // feed the raw radian value
        let result = compute(&format!("sin {} radians", angle)).unwrap();
        assert!(approx_3dp(result, 3f64.sqrt() / 2.0),
            "sin(31π/3) = {} (expected √3/2 ≈ 0.866)", result);
    }

    // Exercise 3.2 Q6: sin(765°) = sin(45°) = √2/2
    // 765 = 2*360 + 45
    #[test]
    fn sin_765() {
        let result = compute("sin 765 degrees").unwrap();
        assert!(approx_3dp(result, 2f64.sqrt() / 2.0),
            "sin(765°) = {} (expected √2/2)", result);
    }

    // Exercise 3.2 Q9: sin(-11π/3) = sin(π/3) = √3/2
    #[test]
    fn sin_neg_11pi_over_3() {
        use std::f64::consts::PI;
        let angle = -11.0 * PI / 3.0;
        let result = compute(&format!("sin {} radians", angle)).unwrap();
        assert!(approx_3dp(result, 3f64.sqrt() / 2.0));
    }

    // Compound angle arithmetic: sin(A+B) verified numerically
    // sin(75°) = sin(45° + 30°) = sin45*cos30 + cos45*sin30
    // = (√2/2)(√3/2) + (√2/2)(1/2) = (√6+√2)/4 ≈ 0.9659
    #[test]
    fn sin_75_compound_angle() {
        let result = compute("sin 75 degrees").unwrap();
        assert!(approx_3dp(result, 0.9659),
            "sin(75°) = {} (expected ≈ 0.9659)", result);
    }

    // tan(15°) = tan(45° - 30°) = (1 - 1/√3)/(1 + 1/√3) = 2 - √3 ≈ 0.2679
    #[test]
    fn tan_15_degrees() {
        let result = compute("tan 15 degrees").unwrap();
        assert!(approx_3dp(result, 0.2679),
            "tan(15°) = {} (expected ≈ 0.2679)", result);
    }

    // double angle: sin(2*30°) = sin(60°) — verify 2*sin30*cos30 = sin60
    #[test]
    fn double_angle_sin_30_numeric() {
        // 2 * sin(30°) * cos(30°) should equal sin(60°)
        let lhs = compute("2 * sin 30 degrees * cos 30 degrees").unwrap();
        let rhs = compute("sin 60 degrees").unwrap();
        assert!(approx(lhs, rhs),
            "2*sin30*cos30 = {} but sin60 = {}", lhs, rhs);
    }

    // inverse trig
    #[test] fn arcsin_half()     { assert!(approx(compute("inverse sine of 0.5").unwrap(), 30.0)); }
    #[test] fn arccos_half()     { assert!(approx(compute("inverse cosine of 0.5").unwrap(), 60.0)); }
    #[test] fn arctan_one()      { assert!(approx(compute("inverse tangent of 1").unwrap(), 45.0)); }
    #[test] fn arcsin_neg_half() { assert!(approx(compute("arcsin -0.5").unwrap(), -30.0)); }
    #[test] fn arcsin_out_range(){ assert!(compute("arcsin 2").is_err()); }
    #[test] fn arccos_out_range(){ assert!(compute("arccos -2").is_err()); }


    // ─────────────────────────────────────────────────────────────
    // EXTRA: Quadrantal angle boundary tests — sec 3.3
    // These verify the engine handles multiples of 90° correctly
    // ─────────────────────────────────────────────────────────────
    #[test] fn sin_720()  { assert!(approx(compute("sin 720 degrees").unwrap(), 0.0)); }
    #[test] fn cos_720()  { assert!(approx(compute("cos 720 degrees").unwrap(), 1.0)); }
    #[test] fn sin_450()  { assert!(approx(compute("sin 450 degrees").unwrap(), 1.0)); }  // 450 = 360+90
    #[test] fn cos_540()  { assert!(approx(compute("cos 540 degrees").unwrap(),-1.0)); }  // 540 = 360+180
    #[test] fn tan_135()  { assert!(approx(compute("tan 135 degrees").unwrap(),-1.0)); }  // 2nd quadrant
    #[test] fn sin_neg_270(){ assert!(approx(compute("sin -270 degrees").unwrap(), 1.0)); }


    // Ex 3.1 Q1(i): 25° → 5π/36 radians ≈ 0.4363
    #[test]
    fn convert_25_deg_to_rad() {
        let result = compute("convert 25 degrees to radians").unwrap();
        assert!(approx_eps(result, 25.0 * PI / 180.0, 1e-9));
    }

    // Ex 3.1 Q1(ii): -47°30' = -47.5° → -19π/72 radians
    #[test]
    fn convert_neg_47_5_deg_to_rad() {
        let result = compute("convert -47.5 degrees to radians").unwrap();
        assert!(approx_eps(result, -47.5 * PI / 180.0, 1e-9));
    }

    // Ex 3.1 Q1(iii): 240° → 4π/3 radians
    #[test]
    fn convert_240_deg_to_rad() {
        let result = compute("convert 240 degrees to radians").unwrap();
        assert!(approx_eps(result, 4.0 * PI / 3.0, 1e-9));
    }

    // Ex 3.1 Q2(i): 11/16 radians → degrees
    #[test]
    fn convert_11_over_16_rad_to_deg() {
        let result = compute("convert 0.6875 radians to degrees").unwrap();
        assert!(approx_eps(result, 0.6875 * 180.0 / PI, 1e-9));
    }

    // Variable trignometric solving (not used refrence of NCERT, just added for coverage of variable solving + trigonometry)
    #[test]
    fn test_solve_sine_basic() {
        assert!(
            approx_eps(compute("solve x in sin(x) = 0.5").unwrap(), 30.0, 1e-9)    );
    }
            

    #[test]
    fn test_solve_sine_negative() {
        assert!(
            approx_eps(compute("solve x in sin(x) = -0.5").unwrap(), -30.0, 1e-9)
        );
    }

    #[test]
    fn test_solve_sine_one() {
        assert!(
            approx_eps(compute("solve x in sin(x) = 1").unwrap(), 90.0, 1e-9)
        );
    }

    #[test]
    fn test_solve_cosine_basic() {
        assert!(
            approx_eps(compute("solve x in cos(x) = 0.5").unwrap(), 60.0, 1e-9)
        );
    }

    #[test]
    fn test_solve_tangent_basic() {
        assert!(
            approx_eps(compute("solve x in tan(x) = 1").unwrap(), 45.0, 1e-9)
        );
    }

    #[test]
    fn test_solve_shifted_sine() {
        assert!(
            approx_eps(compute("solve x in sin(x + 30) = 0.5").unwrap(), 0.0, 1e-9)
        );
    }

    #[test]
    fn test_solve_shifted_cosine() {
        assert!(
            approx_eps(compute("solve x in cos(x + 60) = 0.5").unwrap(), 0.0, 1e-9)
        );
    }

    #[test]
    fn test_solve_scaled_sine() {
        assert!(
            approx_eps(compute("solve x in 2 * sin(x) = 1").unwrap(), 30.0, 1e-9)
        );
    }

    #[test]
    fn test_solve_sine_plus_constant() {
        assert!(
            approx_eps(compute("solve x in sin(x) + 1 = 1.5").unwrap(), 30.0, 1e-9)
        );

    }

    #[test]
    fn test_solve_arcsin() {
        assert!(
            approx_eps(compute("solve x in arcsin(x) = 30").unwrap(), 0.5, 1e-9)
        );
    }

    #[test]
    fn test_solve_arccos() {
        assert!(
            approx_eps(compute("solve x in arccos(x) = 60").unwrap(), 0.5, 1e-9)
        );
    }

    #[test]
    fn test_solve_arctan() {
        assert!(
            approx_eps(compute("solve x in arctan(x) = 45").unwrap(), 1.0, 1e-9)
        );
    }

    #[test]
    fn test_solve_multiple_solutions_sine() {
        assert!(
            approx_eps(compute("solve x in sin(x) = 0.5").unwrap(), 30.0, 1e-9)
        );
        // Principal solution only
    }




    }