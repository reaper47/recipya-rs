pub fn float_to_i16_safe<F: Into<f64>>(n: F) -> i16 {
    let clamped = n
        .into()
        .round()
        .clamp(f64::from(i16::MIN), f64::from(i16::MAX));
    #[allow(clippy::cast_possible_truncation)]
    {
        clamped as i16
    }
}
