use crate::lib::macros;
use crate::lib::util as utils;

/// Gets the date of Easter for the year specified.
///
/// ## Arguments
///
/// input_year -- Year for which you'd like the date of Easter.
///
/// ## Returns
///
/// month, day, year
pub fn get_date_of_easter(input_year: u32) -> (u32, u32, u32) {
    let year = input_year as f64;

    let a = year % 19.0;
    let b = (year / 100.0).floor();
    let c = year % 100.0;
    let d = (b / 4.0).floor();
    let e = b % 4.0;
    let f = ((b + 8.0) / 25.0).floor();
    let g = ((b - f + 1.0) / 3.0).floor();
    let h = ((19.0 * a) + b - d - g + 15.0) % 30.0;
    let i = (c / 4.0).floor();
    let k = c % 4.0;
    let l = (32.0 + 2.0 * (e + i) - h - k) % 7.0;
    let m = ((a + (11.0 * h) + (22.0 * l)) / 451.0).floor();
    let n = ((h + l - (7.0 * m) + 114.0) / 31.0).floor();
    let p = (h + l - (7.0 * m) + 114.0) % 31.0;

    let day = p + 1.0;
    let month = n;

    return (month as u32, day as u32, year as u32);
}

/// Calculate day number for a date.
///
/// ## Arguments
///
/// month, day, year
///
/// ## Returns
///
/// day_number
pub fn civil_date_to_day_number(mut month: u32, day: u32, year: u32) -> u32 {
    if month <= 2 {
        month = month - 1;
        month = if utils::is_leap_year(year) {
            month * 62
        } else {
            month * 63
        };
        month = (month as f64 / 2.0).floor() as u32;
    } else {
        month = ((month as f64 + 1.0) * 30.6).floor() as u32;
        month = if utils::is_leap_year(year) {
            month - 62
        } else {
            month - 63
        };
    }

    return month + day;
}

/// Convert a Civil Time (hours,minutes,seconds) to Decimal Hours
pub fn civil_time_to_decimal_hours(hours: f64, minutes: f64, seconds: f64) -> f64 {
    return macros::hms_dh(hours, minutes, seconds as f64);
}

/// Convert Decimal Hours to Civil Time
///
/// ## Returns
///
/// hours (u32), minutes (u32), seconds (u32)
pub fn decimal_hours_to_civil_time(decimal_hours: f64) -> (f64, f64, f64) {
    let hours = macros::dh_hour(decimal_hours);
    let minutes = macros::dh_min(decimal_hours);
    let seconds = macros::dh_sec(decimal_hours);

    return (hours as f64, minutes as f64, seconds as f64);
}

/// Convert local Civil Time to Universal Time
///
/// ## Returns
///
/// UT hours, UT mins, UT secs, GW day, GW month, GW year
pub fn local_civil_time_to_universal_time(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    is_daylight_savings: bool,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
) -> (u32, u32, u32, u32, u32, u32) {
    let lct = civil_time_to_decimal_hours(lct_hours, lct_minutes, lct_seconds);

    let daylight_savings_offset = if is_daylight_savings == true { 1 } else { 0 };

    let ut_interim = lct - daylight_savings_offset as f64 - zone_correction as f64;
    let gday_interim = local_day as f64 + (ut_interim / 24.0);

    let jd = macros::cd_jd(gday_interim, local_month, local_year);

    let g_day = macros::jdc_day(jd) as f64;
    let g_month = macros::jdc_month(jd);
    let g_year = macros::jdc_year(jd);

    let ut = 24.0 * (g_day - g_day.floor());

    return (
        macros::dh_hour(ut),
        macros::dh_min(ut),
        macros::dh_sec(ut) as u32,
        g_day.floor() as u32,
        g_month,
        g_year,
    );
}

/// Convert Universal Time to local Civil Time
///
/// ## Returns
///
/// LCT hours, LCT minutes, LCT seconds, day, month, year
pub fn universal_time_to_local_civil_time(
    ut_hours: f64,
    ut_minutes: f64,
    ut_seconds: f64,
    is_daylight_savings: bool,
    zone_correction: i32,
    gw_day: u32,
    gw_month: u32,
    gw_year: u32,
) -> (u32, u32, u32, u32, u32, u32) {
    let dst_value = if is_daylight_savings == true { 1 } else { 0 };
    let ut = macros::hms_dh(ut_hours, ut_minutes, ut_seconds);
    let zone_time = ut + zone_correction as f64;
    let local_time = zone_time + dst_value as f64;
    let local_jd_plus_local_time =
        macros::cd_jd(gw_day as f64, gw_month, gw_year) + (local_time / 24.0);
    let local_day = macros::jdc_day(local_jd_plus_local_time) as f64;
    let integer_day = local_day.floor();
    let local_month = macros::jdc_month(local_jd_plus_local_time);
    let local_year = macros::jdc_year(local_jd_plus_local_time);

    let lct = 24.0 * (local_day - integer_day as f64);

    return (
        macros::dh_hour(lct),
        macros::dh_min(lct),
        macros::dh_sec(lct) as u32,
        integer_day as u32,
        local_month,
        local_year,
    );
}

/// Convert Universal Time to Greenwich Sidereal Time
///
/// ## Returns
/// GST hours, GST minutes, GST seconds
pub fn universal_time_to_greenwich_sidereal_time(
    ut_hours: f64,
    ut_minutes: f64,
    ut_seconds: f64,
    gw_day: f64,
    gw_month: u32,
    gw_year: u32,
) -> (u32, u32, f64) {
    let jd = macros::cd_jd(gw_day as f64, gw_month, gw_year);
    let s = jd - 2451545.0;
    let t = s / 36525.0;
    let t01 = 6.697374558 + (2400.051336 * t) + (0.000025862 * t * t);
    let t02 = t01 - (24.0 * (t01 / 24.0).floor());
    let ut = macros::hms_dh(ut_hours, ut_minutes, ut_seconds);
    let a = ut * 1.002737909;
    let gst1 = t02 + a;
    let gst2 = gst1 - (24.0 * (gst1 / 24.0).floor());

    let gst_hours = macros::dh_hour(gst2);
    let gst_minutes = macros::dh_min(gst2);
    let gst_seconds = macros::dh_sec(gst2);

    return (gst_hours, gst_minutes, gst_seconds);
}

/// Convert Greenwich Sidereal Time to Universal Time
///
/// ## Returns
/// UT hours, UT minutes, UT seconds, Warning Flag
pub fn greenwich_sidereal_time_to_universal_time(
    gst_hours: f64,
    gst_minutes: f64,
    gst_seconds: f64,
    gw_day: f64,
    gw_month: u32,
    gw_year: u32,
) -> (u32, u32, f64, String) {
    let jd = macros::cd_jd(gw_day, gw_month, gw_year);
    let s = jd - 2451545.0;
    let t = s / 36525.0;
    let t01 = 6.697374558 + (2400.051336 * t) + (0.000025862 * t * t);
    let t02 = t01 - (24.0 * (t01 / 24.0).floor());
    let gst_hours1 = macros::hms_dh(gst_hours, gst_minutes, gst_seconds);

    let a = gst_hours1 - t02;
    let b = a - (24.0 * (a / 24.0).floor());
    let ut = b * 0.9972695663;
    let ut_hours = macros::dh_hour(ut);
    let ut_minutes = macros::dh_min(ut);
    let ut_seconds = macros::dh_sec(ut);

    let warning_flag = if ut < 0.065574 { "Warning" } else { "OK" };

    return (ut_hours, ut_minutes, ut_seconds, warning_flag.to_string());
}

/// Convert Greenwich Sidereal Time to Local Sidereal Time
///
/// ## Returns
/// LST hours, LST minutes, LST seconds
pub fn greenwich_sidereal_time_to_local_sidereal_time(
    gst_hour: f64,
    gst_minutes: f64,
    gst_seconds: f64,
    geographical_longitude: f64,
) -> (u32, u32, f64) {
    let gst = macros::hms_dh(gst_hour, gst_minutes, gst_seconds);
    let offset = geographical_longitude / 15.0;
    let lst_hours1 = gst + offset;
    let lst_hours2 = lst_hours1 - (24.0 * (lst_hours1 / 24.0).floor());

    let lst_hours = macros::dh_hour(lst_hours2);
    let lst_minutes = macros::dh_min(lst_hours2);
    let lst_seconds = macros::dh_sec(lst_hours2);

    return (lst_hours, lst_minutes, lst_seconds);
}

/// Convert Local Sidereal Time to Greenwich Sidereal Time
///
/// ## Returns
/// GST hours, GST minutes, GST seconds
pub fn local_sidereal_time_to_greenwich_sidereal_time(
    lst_hours: f64,
    lst_minutes: f64,
    lst_seconds: f64,
    geographical_longitude: f64,
) -> (u32, u32, f64) {
    let gst = macros::hms_dh(lst_hours, lst_minutes, lst_seconds);
    let long_hours = geographical_longitude / 15.0;
    let gst1 = gst - long_hours;
    let gst2 = gst1 - (24.0 * (gst1 / 24.0).floor());

    let gst_hours = macros::dh_hour(gst2);
    let gst_minutes = macros::dh_min(gst2);
    let gst_seconds = macros::dh_sec(gst2);

    return (gst_hours, gst_minutes, gst_seconds);
}
