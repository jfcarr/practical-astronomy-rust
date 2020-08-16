use crate::lib::macros;
use crate::lib::types as pa_types;
use crate::lib::util as utils;

/// Calculate approximate position of the sun for a local date and time.
///
/// ## Arguments
/// * `lct_hours` -- Local civil time, in hours.
/// * `lct_minutes` -- Local civil time, in minutes.
/// * `lct_seconds` -- Local civil time, in seconds.
/// * `local_day` -- Local date, day part.
/// * `local_month` -- Local date, month part.
/// * `local_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction` -- Time zone correction, in hours.
///
/// ## Returns
/// * `sun_ra_hour` -- Right Ascension of Sun, hour part
/// * `sun_ra_min` -- Right Ascension of Sun, minutes part
/// * `sun_ra_sec` -- Right Ascension of Sun, seconds part
/// * `sun_dec_deg` -- Declination of Sun, degrees part
/// * `sun_dec_min` -- Declination of Sun, minutes part
/// * `sun_dec_sec` -- Declination of Sun, seconds part
pub fn approximate_position_of_sun(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
) -> (f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let greenwich_date_day = macros::lct_gday(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let greenwich_date_month = macros::lct_gmonth(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let greenwich_date_year = macros::lct_gyear(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let ut_hours = macros::lct_ut(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let ut_days = ut_hours / 24.0;
    let jd_days = macros::cd_jd(
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    ) + ut_days;
    let d_days = jd_days - macros::cd_jd(0 as f64, 1, 2010);
    let n_deg = 360.0 * d_days / 365.242191;
    let m_deg1 =
        n_deg + macros::sun_e_long(0 as f64, 1, 2010) - macros::sun_peri(0 as f64, 1, 2010);
    let m_deg2 = m_deg1 - 360.0 * (m_deg1 / 360.0).floor();
    let e_c_deg = 360.0 * macros::sun_ecc(0 as f64, 1, 2010) * m_deg2.to_radians().sin()
        / std::f64::consts::PI;
    let l_s_deg1 = n_deg + e_c_deg + macros::sun_e_long(0 as f64, 1, 2010);
    let l_s_deg2 = l_s_deg1 - 360.0 * (l_s_deg1 / 360.0).floor();
    let ra_deg = macros::ec_ra(
        l_s_deg2,
        0 as f64,
        0 as f64,
        0 as f64,
        0 as f64,
        0 as f64,
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    );
    let ra_hours = macros::dd_dh(ra_deg);
    let dec_deg = macros::ec_dec(
        l_s_deg2,
        0 as f64,
        0 as f64,
        0 as f64,
        0 as f64,
        0 as f64,
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    );

    let sun_ra_hour = macros::dh_hour(ra_hours);
    let sun_ra_min = macros::dh_min(ra_hours);
    let sun_ra_sec = macros::dh_sec(ra_hours);
    let sun_dec_deg = macros::dd_deg(dec_deg);
    let sun_dec_min = macros::dd_min(dec_deg);
    let sun_dec_sec = macros::dd_sec(dec_deg);

    return (
        sun_ra_hour as f64,
        sun_ra_min as f64,
        sun_ra_sec,
        sun_dec_deg,
        sun_dec_min,
        sun_dec_sec,
    );
}

/// Calculate precise position of the sun for a local date and time.
///
/// ## Arguments
/// * `lct_hours` -- Local civil time, in hours.
/// * `lct_minutes` -- Local civil time, in minutes.
/// * `lct_seconds` -- Local civil time, in seconds.
/// * `local_day` -- Local date, day part.
/// * `local_month` -- Local date, month part.
/// * `local_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction` -- Time zone correction, in hours.
///
/// ## Returns
/// * `sun_ra_hour` -- Right Ascension of Sun, hour part
/// * `sun_ra_min` -- Right Ascension of Sun, minutes part
/// * `sun_ra_sec` -- Right Ascension of Sun, seconds part
/// * `sun_dec_deg` -- Declination of Sun, degrees part
/// * `sun_dec_min` -- Declination of Sun, minutes part
/// * `sun_dec_sec` -- Declination of Sun, seconds part
pub fn precise_position_of_sun(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
) -> (f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let g_day = macros::lct_gday(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let g_month = macros::lct_gmonth(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let g_year = macros::lct_gyear(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let sun_ecliptic_longitude_deg = macros::sun_long(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let ra_deg = macros::ec_ra(
        sun_ecliptic_longitude_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        g_day,
        g_month,
        g_year,
    );
    let ra_hours = macros::dd_dh(ra_deg);
    let dec_deg = macros::ec_dec(
        sun_ecliptic_longitude_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        g_day,
        g_month,
        g_year,
    );

    let sun_ra_hour = macros::dh_hour(ra_hours);
    let sun_ra_min = macros::dh_min(ra_hours);
    let sun_ra_sec = macros::dh_sec(ra_hours);
    let sun_dec_deg = macros::dd_deg(dec_deg);
    let sun_dec_min = macros::dd_min(dec_deg);
    let sun_dec_sec = macros::dd_sec(dec_deg);

    return (
        sun_ra_hour as f64,
        sun_ra_min as f64,
        sun_ra_sec,
        sun_dec_deg,
        sun_dec_min,
        sun_dec_sec,
    );
}

/// Calculate distance to the Sun (in km), and angular size.
///
/// ## Arguments
/// * `lct_hours` -- Local civil time, in hours.
/// * `lct_minutes` -- Local civil time, in minutes.
/// * `lct_seconds` -- Local civil time, in seconds.
/// * `local_day` -- Local date, day part.
/// * `local_month` -- Local date, month part.
/// * `local_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction` -- Time zone correction, in hours.
///
/// ## Returns
/// * `sun_dist_km` -- Sun's distance, in kilometers
/// * `sun_ang_size_deg` -- Sun's angular size (degrees part)
/// * `sun_ang_size_min` -- Sun's angular size (minutes part)
/// * `sun_ang_size_sec` -- Sun's angular size (seconds part)
pub fn sun_distance_and_angular_size(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
) -> (f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let g_day = macros::lct_gday(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let g_month = macros::lct_gmonth(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let g_year = macros::lct_gyear(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let true_anomaly_deg = macros::sun_true_anomaly(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let true_anomaly_rad = true_anomaly_deg.to_radians();
    let eccentricity = macros::sun_ecc(g_day, g_month, g_year);
    let f = (1.0 + eccentricity * true_anomaly_rad.cos()) / (1.0 - eccentricity * eccentricity);
    let r_km = 149598500.0 / f;
    let theta_deg = f * 0.533128;

    let sun_dist_km = utils::round_f64(r_km, 0);
    let sun_ang_size_deg = macros::dd_deg(theta_deg);
    let sun_ang_size_min = macros::dd_min(theta_deg);
    let sun_ang_size_sec = macros::dd_sec(theta_deg);

    return (
        sun_dist_km,
        sun_ang_size_deg,
        sun_ang_size_min,
        sun_ang_size_sec,
    );
}

/// Calculate local sunrise and sunset.
///
/// ## Arguments
/// * local_day -- Local date, day part.
/// * local_month -- Local date, month part.
/// * local_year -- Local date, year part.
/// * is_daylight_saving -- Is daylight savings in effect?
/// * zone_correction -- Time zone correction, in hours.
/// * geographical_long_deg -- Geographical longitude, in degrees.
/// * geographical_lat_deg -- Geographical latitude, in degrees.
///
/// ## Returns
/// * local_sunrise_hour -- Local sunrise, hour part
/// * local_sunrise_minute -- Local sunrise, minutes part
/// * local_sunset_hour -- Local sunset, hour part
/// * local_sunset_minute -- Local sunset, minutes part
/// * azimuth_of_sunrise_deg -- Azimuth (horizon direction) of sunrise, in degrees
/// * azimuth_of_sunset_deg -- Azimuth (horizon direction) of sunset, in degrees
/// * status -- Calculation status
pub fn sunrise_and_sunset(
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
    geographical_long_deg: f64,
    geographical_lat_deg: f64,
) -> (f64, f64, f64, f64, f64, f64, String) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let local_sunrise_hours = macros::sunrise_lct(
        local_day,
        local_month,
        local_year,
        daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
    );

    let local_sunset_hours = macros::sunset_lct(
        local_day,
        local_month,
        local_year,
        daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
    );

    let sun_rise_set_status = macros::e_sun_rs(
        local_day,
        local_month,
        local_year,
        daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
    );

    let adjusted_sunrise_hours = local_sunrise_hours + 0.008333;
    let adjusted_sunset_hours = local_sunset_hours + 0.008333;
    let azimuth_of_sunrise_deg1 = macros::sunrise_az(
        local_day,
        local_month,
        local_year,
        daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
    );
    let azimuth_of_sunset_deg1 = macros::sunset_az(
        local_day,
        local_month,
        local_year,
        daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
    );

    let local_sunrise_hour = if sun_rise_set_status == "OK" {
        macros::dh_hour(adjusted_sunrise_hours) as f64
    } else {
        0.0
    };
    let local_sunrise_minute = if sun_rise_set_status == "OK" {
        macros::dh_min(adjusted_sunrise_hours) as f64
    } else {
        0.0
    };
    let local_sunset_hour = if sun_rise_set_status == "OK" {
        macros::dh_hour(adjusted_sunset_hours) as f64
    } else {
        0.0
    };
    let local_sunset_minute = if sun_rise_set_status == "OK" {
        macros::dh_min(adjusted_sunset_hours) as f64
    } else {
        0.0
    };
    let azimuth_of_sunrise_deg = if sun_rise_set_status == "OK" {
        utils::round_f64(azimuth_of_sunrise_deg1, 2)
    } else {
        0.0
    };
    let azimuth_of_sunset_deg = if sun_rise_set_status == "OK" {
        utils::round_f64(azimuth_of_sunset_deg1, 2)
    } else {
        0.0
    };
    let status = sun_rise_set_status.to_string();

    return (
        local_sunrise_hour,
        local_sunrise_minute,
        local_sunset_hour,
        local_sunset_minute,
        azimuth_of_sunrise_deg,
        azimuth_of_sunset_deg,
        status,
    );
}

/// Calculate times of morning and evening twilight.
///
/// ## Arguments
/// * `local_day` -- Local date, day part.
/// * `local_month` -- Local date, month part.
/// * `local_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction` -- Time zone correction, in hours.
/// * `geographical_long_deg` -- Geographical longitude, in degrees.
/// * `geographical_lat_deg` -- Geographical latitude, in degrees.
/// * `twilight_type` -- "C" (civil), "N" (nautical), or "A" (astronomical)
///
/// ## Returns
/// * `am_twilight_begins_hour` -- Beginning of AM twilight (hour part)
/// * `am_twilight_begins_min` -- Beginning of AM twilight (minutes part)
/// * `pm_twilight_ends_hour` -- Ending of PM twilight (hour part)
/// * `pm_twilight_ends_min` -- Ending of PM twilight (minutes part)
/// * `status` -- Calculation status
pub fn morning_and_evening_twilight(
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
    geographical_long_deg: f64,
    geographical_lat_deg: f64,
    twilight_type: pa_types::TwilightType,
) -> (f64, f64, f64, f64, String) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let start_of_am_twilight_hours = macros::twilight_am_lct(
        local_day,
        local_month,
        local_year,
        daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
        &twilight_type,
    );

    let end_of_pm_twilight_hours = macros::twilight_pm_lct(
        local_day,
        local_month,
        local_year,
        daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
        &twilight_type,
    );

    let twilight_status = macros::e_twilight(
        local_day,
        local_month,
        local_year,
        daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
        &twilight_type,
    );

    let adjusted_am_start_time = start_of_am_twilight_hours + 0.008333;
    let adjusted_pm_start_time = end_of_pm_twilight_hours + 0.008333;

    let am_twilight_begins_hour = if twilight_status == "OK" {
        macros::dh_hour(adjusted_am_start_time) as f64
    } else {
        -99.0
    };
    let am_twilight_begins_min = if twilight_status == "OK" {
        macros::dh_min(adjusted_am_start_time) as f64
    } else {
        -99.0
    };
    let pm_twilight_ends_hour = if twilight_status == "OK" {
        macros::dh_hour(adjusted_pm_start_time) as f64
    } else {
        -99.0
    };
    let pm_twilight_ends_min = if twilight_status == "OK" {
        macros::dh_min(adjusted_pm_start_time) as f64
    } else {
        -99.0
    };
    let status = twilight_status;

    return (
        am_twilight_begins_hour,
        am_twilight_begins_min,
        pm_twilight_ends_hour,
        pm_twilight_ends_min,
        status,
    );
}

/// Calculate the equation of time. (The difference between the real Sun time and the mean Sun time.)
///
/// ## Arguments
/// * `gwdate_day` -- Greenwich date (day part)
/// * `gwdate_month` -- Greenwich date (month part)
/// * `gwdate_year` -- Greenwich date (year part)
///
/// ## Returns
/// * `equation_of_time_min` -- equation of time (minute part)
/// * `equation_of_time_sec` -- equation of time (seconds part)
pub fn equation_of_time(gwdate_day: f64, gwdate_month: u32, gwdate_year: u32) -> (f64, f64) {
    let sun_longitude_deg =
        macros::sun_long(12.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year);
    let sun_ra_hours = macros::dd_dh(macros::ec_ra(
        sun_longitude_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        gwdate_day,
        gwdate_month,
        gwdate_year,
    ));
    let equivalent_ut_hours = macros::gst_ut(
        sun_ra_hours,
        0.0,
        0.0,
        gwdate_day,
        gwdate_month,
        gwdate_year,
    );
    let equation_of_time_hours = equivalent_ut_hours - 12.0;

    let equation_of_time_min = macros::dh_min(equation_of_time_hours) as f64;
    let equation_of_time_sec = macros::dh_sec(equation_of_time_hours);

    return (equation_of_time_min, equation_of_time_sec);
}

/// Calculate solar elongation for a celestial body.
///
/// Solar elongation is the angle between the lines of sight from the Earth to the Sun and from the Earth to the celestial body.
///
/// ## Arguments
/// * `ra_hour` -- Right Ascension, hour part
/// * `ra_min` -- Right Ascension, minutes part
/// * `ra_sec` -- Right Ascension, seconds part
/// * `dec_deg` -- Declination, degrees part
/// * `dec_min` -- Declination, minutes part
/// * `dec_sec` -- Declination, seconds part
/// * `gwdate_day` -- Greenwich Date, day part
/// * `gwdate_month` -- Greenwich Date, month part
/// * `gwdate_year` -- Greenwich Date, year part
///
/// ## Returns
/// * `solar_elongation_deg` -- Solar elongation, in degrees
pub fn solar_elongation(
    ra_hour: f64,
    ra_min: f64,
    ra_sec: f64,
    dec_deg: f64,
    dec_min: f64,
    dec_sec: f64,
    gwdate_day: f64,
    gwdate_month: u32,
    gwdate_year: u32,
) -> f64 {
    let sun_longitude_deg =
        macros::sun_long(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year);
    let sun_ra_hours = macros::dd_dh(macros::ec_ra(
        sun_longitude_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        gwdate_day,
        gwdate_month,
        gwdate_year,
    ));
    let sun_dec_deg = macros::ec_dec(
        sun_longitude_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        gwdate_day,
        gwdate_month,
        gwdate_year,
    );
    let solar_elongation_deg = macros::angle(
        sun_ra_hours,
        0.0,
        0.0,
        sun_dec_deg,
        0.0,
        0.0,
        ra_hour,
        ra_min,
        ra_sec,
        dec_deg,
        dec_min,
        dec_sec,
        pa_types::AngleMeasure::Hours,
    );

    return utils::round_f64(solar_elongation_deg, 2);
}
