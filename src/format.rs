pub fn format_number(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }

    if value.is_infinite() {
        if value.is_sign_positive() {
            return "Infinity".to_string();
        }

        return "-Infinity".to_string();
    }

    let rounded = value.round();

    if (value - rounded).abs() < 1e-9 {
        return format!("{}", rounded as i64);
    }

    format!("{}", value)
}
