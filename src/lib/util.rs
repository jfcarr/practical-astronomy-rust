/// Determine if year is a leap year.
///
/// ## Arguments
///
/// year
///
/// ## Returns
///
/// true or false
pub fn is_leap_year(input_year: u32) -> bool {
    let year = input_year as f64;

    if year % 4.0 == 0.0 {
        if year % 100.0 == 0.0 {
            return if year % 400.0 == 0.0 { true } else { false };
        } else {
            return true;
        }
    } else {
        return false;
    }
}

/// Round an f64 primitive to the specified number of decimal places.
pub fn round_f64(input_value: f64, places: usize) -> f64 {
    return format!("{:.width$}", input_value, width = places)
        .parse::<f64>()
        .unwrap();
}
