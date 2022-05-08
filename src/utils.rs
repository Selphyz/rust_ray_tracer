pub fn float_fuzzy_eq(left: f64, right: f64) -> bool {
    let eps = 0.00001;
    (left - right).abs() < eps
}
