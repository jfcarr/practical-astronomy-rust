/// Determine if year is a leap year.
///
/// ## Arguments
/// year
///
/// ## Returns
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

/// Convert a Universal Time hour to Local Time
pub fn get_local_hour_from_ut(
    input_hour: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
) -> f64 {
    let adjustment_value: f64 = if is_daylight_saving {
        (zone_correction_hours as f64) - 1.0
    } else {
        zone_correction_hours as f64
    };

    let local_hour: f64 = input_hour - adjustment_value;

    return local_hour;
}
