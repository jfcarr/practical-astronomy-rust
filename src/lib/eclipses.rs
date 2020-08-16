use crate::lib::macros;
use crate::lib::util;

/// Determine if a lunar eclipse is likely to occur.
///
/// ## Arguments
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
///
/// ## Returns
/// * `status` -- One of "Lunar eclipse certain", "Lunar eclipse possible", or "No lunar eclipse".
/// * `event_date_day` -- Date of eclipse event (day).
/// * `event_date_month` -- Date of eclipse event (month).
/// * `event_date_year` -- Date of eclipse event (year).
pub fn lunar_eclipse_occurrence(
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
) -> (String, f64, u32, u32) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let julian_date_of_full_moon = macros::full_moon(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let g_date_of_full_moon_day = macros::jdc_day(julian_date_of_full_moon);
    let integer_day = (g_date_of_full_moon_day).floor();
    let g_date_of_full_moon_month = macros::jdc_month(julian_date_of_full_moon);
    let g_date_of_full_moon_year = macros::jdc_year(julian_date_of_full_moon);
    let ut_of_full_moon_hours = g_date_of_full_moon_day - integer_day;
    let _local_civil_time_hours = macros::ut_lct(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let local_civil_date_day = macros::ut_lc_day(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let local_civil_date_month = macros::ut_lc_month(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let local_civil_date_year = macros::ut_lc_year(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let eclipse_occurrence = macros::lunar_eclipse_occurrence(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let status = eclipse_occurrence;
    let event_date_day = local_civil_date_day;
    let event_date_month = local_civil_date_month;
    let event_date_year = local_civil_date_year;

    return (status, event_date_day, event_date_month, event_date_year);
}

/// Calculate the circumstances of a lunar eclipse.
///
/// ## Arguments
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
///
/// ## Returns
/// * `lunar_eclipse_certain_date_day` -- Lunar eclipse date (day)
/// * `lunar_eclipse_certain_date_month` -- Lunar eclipse date (month)
/// * `lunar_eclipse_certain_date_year` -- Lunar eclipse date (year)
/// * `ut_start_pen_phase_hour` -- Start of penumbral phase (hour)
/// * `ut_start_pen_phase_minutes` -- Start of penumbral phase (minutes)
/// * `ut_start_umbral_phase_hour` -- Start of umbral phase (hour)
/// * `ut_start_umbral_phase_minutes` -- Start of umbral phase (minutes)
/// * `ut_start_total_phase_hour` -- Start of total phase (hour)
/// * `ut_start_total_phase_minutes` -- Start of total phase (minutes)
/// * `ut_mid_eclipse_hour` -- Mid-eclipse (hour)
/// * `ut_mid_eclipse_minutes` -- Mid-eclipse (minutes)
/// * `ut_end_total_phase_hour` -- End of total phase (hour)
/// * `ut_end_total_phase_minutes` -- End of total phase (minutes)
/// * `ut_end_umbral_phase_hour` -- End of umbral phase (hour)
/// * `ut_end_umbral_phase_minutes` -- End of umbral phase (minutes)
/// * `ut_end_pen_phase_hour` -- End of penumbral phase (hour)
/// * `ut_end_pen_phase_minutes` -- End of penumbral phase (minutes)
/// * `eclipse_magnitude` -- Eclipse magnitude
pub fn lunar_eclipse_circumstances(
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
) -> (
    f64,
    u32,
    u32,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let julian_date_of_full_moon = macros::full_moon(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let g_date_of_full_moon_day = macros::jdc_day(julian_date_of_full_moon);
    let integer_day = g_date_of_full_moon_day.floor();
    let g_date_of_full_moon_month = macros::jdc_month(julian_date_of_full_moon);
    let g_date_of_full_moon_year = macros::jdc_year(julian_date_of_full_moon);
    let ut_of_full_moon_hours = g_date_of_full_moon_day - integer_day;
    let _local_civil_time_hours = macros::ut_lct(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let local_civil_date_day = macros::ut_lc_day(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let local_civil_date_month = macros::ut_lc_month(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let local_civil_date_year = macros::ut_lc_year(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let _eclipse_occurrence = macros::lunar_eclipse_occurrence(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let ut_max_eclipse = macros::ut_max_lunar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
    );
    let ut_first_contact = macros::ut_first_contact_lunar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
    );
    let ut_last_contact = macros::ut_last_contact_lunar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
    );
    let ut_start_umbral_phase = macros::ut_start_umbra_lunar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
    );
    let ut_end_umbral_phase = macros::ut_end_umbra_lunar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
    );
    let ut_start_total_phase = macros::ut_start_total_lunar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
    );
    let ut_end_total_phase = macros::ut_end_total_lunar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
    );
    let eclipse_magnitude1 = macros::mag_lunar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
    );

    let lunar_eclipse_certain_date_day = local_civil_date_day;
    let lunar_eclipse_certain_date_month = local_civil_date_month;
    let lunar_eclipse_certain_date_year = local_civil_date_year;
    let ut_start_pen_phase_hour = if ut_first_contact == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_first_contact + 0.008333) as f64
    };
    let ut_start_pen_phase_minutes = if ut_first_contact == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_first_contact + 0.008333) as f64
    };
    let ut_start_umbral_phase_hour = if ut_start_umbral_phase == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_start_umbral_phase + 0.008333) as f64
    };
    let ut_start_umbral_phase_minutes = if ut_start_umbral_phase == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_start_umbral_phase + 0.008333) as f64
    };
    let ut_start_total_phase_hour = if ut_start_total_phase == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_start_total_phase + 0.008333) as f64
    };
    let ut_start_total_phase_minutes = if ut_start_total_phase == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_start_total_phase + 0.008333) as f64
    };
    let ut_mid_eclipse_hour = if ut_max_eclipse == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_max_eclipse + 0.008333) as f64
    };
    let ut_mid_eclipse_minutes = if ut_max_eclipse == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_max_eclipse + 0.008333) as f64
    };
    let ut_end_total_phase_hour = if ut_end_total_phase == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_end_total_phase + 0.008333) as f64
    };
    let ut_end_total_phase_minutes = if ut_end_total_phase == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_end_total_phase + 0.008333) as f64
    };
    let ut_end_umbral_phase_hour = if ut_end_umbral_phase == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_end_umbral_phase + 0.008333) as f64
    };
    let ut_end_umbral_phase_minutes = if ut_end_umbral_phase == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_end_umbral_phase + 0.008333) as f64
    };
    let ut_end_pen_phase_hour = if ut_last_contact == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_last_contact + 0.008333) as f64
    };
    let ut_end_pen_phase_minutes = if ut_last_contact == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_last_contact + 0.008333) as f64
    };
    let eclipse_magnitude = if eclipse_magnitude1 == -99.0 {
        -99.0
    } else {
        util::round_f64(eclipse_magnitude1, 2)
    };

    return (
        lunar_eclipse_certain_date_day,
        lunar_eclipse_certain_date_month,
        lunar_eclipse_certain_date_year,
        ut_start_pen_phase_hour,
        ut_start_pen_phase_minutes,
        ut_start_umbral_phase_hour,
        ut_start_umbral_phase_minutes,
        ut_start_total_phase_hour,
        ut_start_total_phase_minutes,
        ut_mid_eclipse_hour,
        ut_mid_eclipse_minutes,
        ut_end_total_phase_hour,
        ut_end_total_phase_minutes,
        ut_end_umbral_phase_hour,
        ut_end_umbral_phase_minutes,
        ut_end_pen_phase_hour,
        ut_end_pen_phase_minutes,
        eclipse_magnitude,
    );
}

/// Determine if a solar eclipse is likely to occur.
///
/// ## Arguments
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
///
/// ## Returns
/// * `status` -- One of "Solar eclipse certain", "Solar eclipse possible", or "No solar eclipse".
/// * `event_date_day` -- Date of eclipse event (day).
/// * `event_date_month` -- Date of eclipse event (month).
/// * `event_date_year` -- Date of eclipse event (year).
pub fn solar_eclipse_occurrence(
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
) -> (String, f64, u32, u32) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let julian_date_of_new_moon = macros::new_moon(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let g_date_of_new_moon_day = macros::jdc_day(julian_date_of_new_moon);
    let integer_day = (g_date_of_new_moon_day).floor();
    let g_date_of_new_moon_month = macros::jdc_month(julian_date_of_new_moon);
    let g_date_of_new_moon_year = macros::jdc_year(julian_date_of_new_moon);
    let ut_of_new_moon_hours = g_date_of_new_moon_day - integer_day;
    let _local_civil_time_hours = macros::ut_lct(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let local_civil_date_day = macros::ut_lc_day(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let local_civil_date_month = macros::ut_lc_month(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let local_civil_date_year = macros::ut_lc_year(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let eclipse_occurrence = macros::solar_eclipse_occurrence(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let status = eclipse_occurrence;
    let event_date_day = local_civil_date_day;
    let event_date_month = local_civil_date_month;
    let event_date_year = local_civil_date_year;

    return (status, event_date_day, event_date_month, event_date_year);
}

/// Calculate the circumstances of a lunar eclipse.
///
/// ## Arguments
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `geog_longitude_deg` -- Geographical longitude of observer.
/// * `geog_latitude_deg` -- Geographical latitude of observer.
///
/// ## Returns
/// * `solar_eclipse_certain_date_day` -- Solar eclipse date (day)
/// * `solar_eclipse_certain_date_month` -- Solar eclipse date (month)
/// * `solar_eclipse_certain_date_year` -- Solar eclipse date (year)
/// * `ut_first_contact_hour` -- First contact of shadow (hour)
/// * `ut_first_contact_minutes` -- First contact of shadow (minutes)
/// * `ut_mid_eclipse_hour` -- Mid-eclipse (hour)
/// * `ut_mid_eclipse_minutes` -- Mid-eclipse (minutes)
/// * `ut_last_contact_hour` -- Last contact of shadow (hour)
/// * `ut_last_contact_minutes` -- Last contact of shadow (minutes)
/// * `eclipse_magnitude` -- Eclipse magnitude
pub fn solar_eclipse_circumstances(
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    geog_longitude_deg: f64,
    geog_latitude_deg: f64,
) -> (f64, u32, u32, f64, f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let julian_date_of_new_moon = macros::new_moon(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let g_date_of_new_moon_day = macros::jdc_day(julian_date_of_new_moon);
    let integer_day = (g_date_of_new_moon_day).floor();
    let g_date_of_new_moon_month = macros::jdc_month(julian_date_of_new_moon);
    let g_date_of_new_moon_year = macros::jdc_year(julian_date_of_new_moon);
    let ut_of_new_moon_hours = g_date_of_new_moon_day - integer_day;
    let _local_civil_time_hours = macros::ut_lct(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let local_civil_date_day = macros::ut_lc_day(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let local_civil_date_month = macros::ut_lc_month(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let local_civil_date_year = macros::ut_lc_year(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let _eclipse_occurrence = macros::solar_eclipse_occurrence(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let ut_max_eclipse = macros::ut_max_solar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_longitude_deg,
        geog_latitude_deg,
    );
    let ut_first_contact = macros::ut_first_contact_solar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_longitude_deg,
        geog_latitude_deg,
    );
    let ut_last_contact = macros::ut_last_contact_solar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_longitude_deg,
        geog_latitude_deg,
    );
    let magnitude = macros::mag_solar_eclipse(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_longitude_deg,
        geog_latitude_deg,
    );

    let solar_eclipse_certain_date_day = local_civil_date_day;
    let solar_eclipse_certain_date_month = local_civil_date_month;
    let solar_eclipse_certain_date_year = local_civil_date_year;
    let ut_first_contact_hour = if ut_first_contact == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_first_contact + 0.008333) as f64
    };
    let ut_first_contact_minutes = if ut_first_contact == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_first_contact + 0.008333) as f64
    };
    let ut_mid_eclipse_hour = if ut_max_eclipse == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_max_eclipse + 0.008333) as f64
    };
    let ut_mid_eclipse_minutes = if ut_max_eclipse == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_max_eclipse + 0.008333) as f64
    };
    let ut_last_contact_hour = if ut_last_contact == -99.0 {
        -99.0
    } else {
        macros::dh_hour(ut_last_contact + 0.008333) as f64
    };
    let ut_last_contact_minutes = if ut_last_contact == -99.0 {
        -99.0
    } else {
        macros::dh_min(ut_last_contact + 0.008333) as f64
    };
    let eclipse_magnitude = if magnitude == -99.0 {
        -99.0
    } else {
        util::round_f64(magnitude, 3)
    };

    return (
        solar_eclipse_certain_date_day,
        solar_eclipse_certain_date_month,
        solar_eclipse_certain_date_year,
        ut_first_contact_hour,
        ut_first_contact_minutes,
        ut_mid_eclipse_hour,
        ut_mid_eclipse_minutes,
        ut_last_contact_hour,
        ut_last_contact_minutes,
        eclipse_magnitude,
    );
}
