use crate::math_engine::compute;

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_compute() {
        assert_eq!(compute("Add two and 3"), Ok(5.0));
    }

    #[test]
    fn test_basic_subtract() {
        assert_eq!(compute("Subtract 20 from 50"), Ok(30.0));
    }

    #[test]
    fn test_basic_multiply() { assert_eq!(compute("Multiply 4 by 6"), Ok(24.0)); }

    #[test]
    fn test_basic_divide()   { assert_eq!(compute("Divide 100 by 5"), Ok(20.0)); }

    // ── negatives ────────────────────────────────────────────
    #[test]
    fn test_negative_add()      { assert_eq!(compute("Add -15 and 10"), Ok(-5.0)); }

    #[test]
    fn test_negative_subtract() { assert_eq!(compute("Subtract -10 from -50"), Ok(-40.0)); }

    #[test]
    fn test_negative_multiply() { assert_eq!(compute("Multiply -5 by -10"), Ok(50.0)); }

    // ── word numbers ─────────────────────────────────────────
    #[test]
    fn test_word_numbers_simple()  { assert_eq!(compute("Add twenty one and thirty two"), Ok(53.0)); }

    #[test]
    fn test_word_numbers_large()   { assert_eq!(compute("Subtract one million from five million"), Ok(4_000_000.0)); }

    #[test]
    fn test_word_numbers_complex() { assert_eq!(compute("Add one hundred twenty three and four hundred fifty six"), Ok(579.0)); }

    #[test]
    fn test_word_numbers_billion() { assert_eq!(compute("Subtract one billion from two billion"), Ok(1_000_000_000.0)); }

    // ── multi-operand ─────────────────────────────────────────
    #[test]
    fn test_multi_add()      { assert_eq!(compute("Add 1 2 3 4 5"), Ok(15.0)); }

    #[test]
    fn test_multi_subtract() { assert_eq!(compute("Subtract 100 20 5 5"), Ok(70.0)); }

    #[test]
    fn test_multi_multiply() { assert_eq!(compute("Multiply 2 3 4 5"), Ok(120.0)); }

    // ── BODMAS ───────────────────────────────────────────────
    #[test]
    fn test_bodmas_basic()   { assert_eq!(compute("2 + 5 * 3"), Ok(17.0)); }

    #[test]
    fn test_bodmas_complex() { assert_eq!(compute("10 + 20 - 5 * 3 / 2"), Ok(22.5)); }

    #[test]
    fn test_division_chain() { assert_eq!(compute("10 / 2 / 5"), Ok(1.0)); }

    // ── floats ───────────────────────────────────────────────
    #[test]
    fn test_float_add()    { assert_eq!(compute("Add 5.5 and 2.5"), Ok(8.0)); }

    #[test]
    fn test_float_divide() { assert_eq!(compute("Divide 5 by 2"), Ok(2.5)); }

    #[test]
    fn test_float_divisor(){ assert_eq!(compute("Divide 1000 by 0.5"), Ok(2000.0)); }

    // ── exponents ───────────────────────────────────────────────

    #[test]
    fn test_expon1(){
        assert_eq!(compute("2 ^ 3 ^ 4"), Ok(2.0_f64.powf((3.0_f64).powf(4.0))));
    }

       #[test]
    fn test_expon2(){
        assert_eq!(compute("10 to the power of 5"), Ok(100000.0));
    }



    // ── errors ───────────────────────────────────────────────

    #[test]
    fn test_divide_by_zero() {
        assert!(compute("Divide 10 by 0").is_err());
    }

    #[test]
    fn test_invalid_input() {
        assert!(compute("Multiply hello by world").is_err());
    }

    // ── WTF conditionals ─────────────────────────────────────
    #[test]
    fn test_wtf_positive() {
        // 6 + (20-10) = 16, not negative → 16 * 3 = 48
        assert_eq!(compute("Add six to the result of subtracting ten from twenty, then multiply by three unless the result is negative"), Ok(48.0));
    }

    #[test]
    fn test_wtf_negative() {
        // 5 + (20-30) = -5, is negative → return -5
        assert_eq!(compute("Add five to the result of subtracting thirty from twenty, then multiply by three unless the result is negative"), Ok(-5.0));
    }
}

// TEST RESULTS :  test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s