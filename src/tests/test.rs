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

     // ── comparison ───────────────────────────────────────────────

    #[test]
    fn test_comparison_equal() {
        assert_eq!(compute("3 = 3"), Ok(1.0));
    }

    #[test]
    fn test_comparison_greater() {
        assert_eq!(compute("10 is greater than 4"), Ok(1.0));
    }

    #[test]
    fn test_comparison_lower() {
        assert_eq!(compute("10 is less than 4"), Ok(0.0));
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

    #[test]
    fn test_wtf_comparison() {
        // 67 + (1000-100) = 967, not >= 900 → 967 * 3 = 2901
        assert_eq!(compute("Add sixty seven to the result of subtracting hundred from thousand, then multiply it by three if the result is greater than  or equal to 900."), Ok(2901.0));
    }

    #[test]
fn test_the_zero_behavior() {
    // Ensures multiplying by text-based zero doesn't break evaluation
    assert_eq!(compute("Multiply zero by one hundred"), Ok(0.0));
}

#[test]
fn test_conditional_scoping() {
    // Does "unless" scope back to the very beginning, or just the immediate previous action?
    // (5 + 5) * 2 unless ... -> 20
    // If it fails: 5 + (5 * 2) -> 15
    assert_eq!(compute("Add five and five then multiply by two unless the result is less than zero"), Ok(20.0));
}

#[test]
fn test_nested_conditionals() {
    // Double condition processing!
    // 10 - 20 = -10 (is negative, so skip "multiply by three") -> -10
    // Then check next condition: is -10 less than zero? Yes! -> multiply by -1 -> 10.0
    assert_eq!(compute("Subtract twenty from ten then multiply by three unless the result is negative, then multiply by -1 if the result is less than zero"), Ok(10.0));
}

#[test]
fn test_false_conditionals() {
    // 50 - 20 = 30. Is 30 less than 10? No. 
    // The "then multiply by 2" should be entirely skipped. 
    assert_eq!(compute("Subtract twenty from fifty, then multiply by two if the result is less than ten"), Ok(30.0));
}

#[test]
fn test_multi_operand_precedence() {
    // Does it do (100 - 20 - 5) * 2 = 150? Or does * 2 only apply to the last element?
    assert_eq!(compute("Subtract 100 20 5 * 2"), Ok(150.0)); 
}

#[test]
fn test_mixed_word_and_symbol_precedence() {
    // Tests if "Multiply" captures everything, or respects standard BODMAS mid-sentence
    // 4 * (6 + 2) = 32 vs (4 * 6) + 2 = 26
    assert_eq!(compute("Multiply 4 by 6 + 2"), Ok(32.0)); 
}

#[test]
fn test_word_number_collisions() {
    // "one" is part of "one hundred", but "one" is also standalone. 
    // "then" contains "ten". Does your lexer separate "then" into "ten"?
    assert_eq!(compute("Add ten then subtract two"), Ok(8.0)); 
}

#[test]
fn test_hyphenated_and_unspaced_numbers() {
    // Checking if your word-to-number engine needs exact spaces or handles natural variations
    assert_eq!(compute("Add twenty-one and thirty-two"), Ok(53.0));
}

#[test]
fn test_double_negatives() {
    // "Subtract minus five" -> minus minus five -> + 5
    assert_eq!(compute("Subtract minus five from ten"), Ok(15.0));
}
}

// TEST RESULTS :  test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s