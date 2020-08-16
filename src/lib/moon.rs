use crate::lib::macros;
use crate::lib::util;

/// Calculate approximate position of the Moon.
///
/// ## Arguments
/// * `lct_hour` -- Local civil time, in hours.
/// * `lct_min` -- Local civil time, in minutes.
/// * `lct_sec` -- Local civil time, in seconds.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
///
/// ## Returns
/// * `moon_ra_hour` -- Right ascension of Moon (hour part)
/// * `moon_ra_min` -- Right ascension of Moon (minutes part)
/// * `moon_ra_sec` -- Right ascension of Moon (seconds part)
/// * `moon_dec_deg` -- Declination of Moon (degrees part)
/// * `moon_dec_min` -- Declination of Moon (minutes part)
/// * `moon_dec_sec` -- Declination of Moon (seconds part)
pub fn approximate_position_of_moon(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
) -> (f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let l0 = 91.9293359879052;
    let p0 = 130.143076320618;
    let n0 = 291.682546643194;
    let i: f64 = 5.145396;

    let gdate_day = macros::lct_gday(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let gdate_month = macros::lct_gmonth(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let gdate_year = macros::lct_gyear(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let ut_hours = macros::lct_ut(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let d_days = macros::cd_jd(gdate_day, gdate_month, gdate_year) - macros::cd_jd(0.0, 1, 2010)
        + ut_hours / 24.0;
    let sun_long_deg = macros::sun_long(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let sun_mean_anomaly_rad = macros::sun_mean_anomaly(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let lm_deg = macros::unwind_deg(13.1763966 * d_days + l0);
    let mm_deg = macros::unwind_deg(lm_deg - 0.1114041 * d_days - p0);
    let n_deg = macros::unwind_deg(n0 - (0.0529539 * d_days));
    let ev_deg = 1.2739 * ((2.0 * (lm_deg - sun_long_deg) - mm_deg).to_radians()).sin();
    let ae_deg = 0.1858 * (sun_mean_anomaly_rad).sin();
    let a3_deg = 0.37 * (sun_mean_anomaly_rad).sin();
    let mmd_deg = mm_deg + ev_deg - ae_deg - a3_deg;
    let ec_deg = 6.2886 * mmd_deg.to_radians().sin();
    let a4_deg = 0.214 * (2.0 * (mmd_deg).to_radians()).sin();
    let ld_deg = lm_deg + ev_deg + ec_deg - ae_deg + a4_deg;
    let v_deg = 0.6583 * (2.0 * (ld_deg - sun_long_deg).to_radians()).sin();
    let ldd_deg = ld_deg + v_deg;
    let nd_deg = n_deg - 0.16 * (sun_mean_anomaly_rad).sin();
    let y = ((ldd_deg - nd_deg).to_radians()).sin() * i.to_radians().cos();
    let x = (ldd_deg - nd_deg).to_radians().cos();

    let moon_long_deg = macros::unwind_deg(macros::degrees(y.atan2(x)) + nd_deg);
    let moon_lat_deg =
        macros::degrees(((ldd_deg - nd_deg).to_radians().sin() * i.to_radians().sin()).asin());
    let moon_ra_hours1 = macros::dd_dh(macros::ec_ra(
        moon_long_deg,
        0.0,
        0.0,
        moon_lat_deg,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    ));
    let moon_dec_deg1 = macros::ec_dec(
        moon_long_deg,
        0.0,
        0.0,
        moon_lat_deg,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    );

    let moon_ra_hour = macros::dh_hour(moon_ra_hours1);
    let moon_ra_min = macros::dh_min(moon_ra_hours1);
    let moon_ra_sec = macros::dh_sec(moon_ra_hours1);
    let moon_dec_deg = macros::dd_deg(moon_dec_deg1);
    let moon_dec_min = macros::dd_min(moon_dec_deg1);
    let moon_dec_sec = macros::dd_sec(moon_dec_deg1);

    return (
        moon_ra_hour as f64,
        moon_ra_min as f64,
        moon_ra_sec,
        moon_dec_deg,
        moon_dec_min,
        moon_dec_sec,
    );
}

/// Calculate approximate position of the Moon.
///
/// ## Arguments
/// * `lct_hour` -- Local civil time, in hours.
/// * `lct_min` -- Local civil time, in minutes.
/// * `lct_sec` -- Local civil time, in seconds.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
///
/// ## Returns
/// * `moon_ra_hour` -- Right ascension of Moon (hour part)
/// * `moon_ra_min` -- Right ascension of Moon (minutes part)
/// * `moon_ra_sec` -- Right ascension of Moon (seconds part)
/// * `moon_dec_deg` -- Declination of Moon (degrees part)
/// * `moon_dec_min` -- Declination of Moon (minutes part)
/// * `moon_dec_sec` -- Declination of Moon (seconds part)
/// * `earth_moon_dist_km` -- Distance from Earth to Moon (km)
/// * `moon_hor_parallax_deg` -- Horizontal parallax of Moon (degrees)
pub fn precise_position_of_moon(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let gdate_day = macros::lct_gday(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let gdate_month = macros::lct_gmonth(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let gdate_year = macros::lct_gyear(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let _ut_hours = macros::lct_ut(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let (moon_ecliptic_longitude_deg, moon_ecliptic_latitude_deg, moon_horizontal_parallax_deg) =
        macros::moon_long_lat_hp(
            lct_hour,
            lct_min,
            lct_sec,
            daylight_saving,
            zone_correction_hours,
            local_date_day,
            local_date_month,
            local_date_year,
        );

    let nutation_in_longitude_deg = macros::nutat_long(gdate_day, gdate_month, gdate_year);
    let corrected_long_deg = moon_ecliptic_longitude_deg + nutation_in_longitude_deg;
    let earth_moon_distance_km = 6378.14 / moon_horizontal_parallax_deg.to_radians().sin();
    let moon_ra_hours_1 = macros::dd_dh(macros::ec_ra(
        corrected_long_deg,
        0.0,
        0.0,
        moon_ecliptic_latitude_deg,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    ));
    let moon_dec_deg1 = macros::ec_dec(
        corrected_long_deg,
        0.0,
        0.0,
        moon_ecliptic_latitude_deg,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    );

    let moon_ra_hour = macros::dh_hour(moon_ra_hours_1);
    let moon_ra_min = macros::dh_min(moon_ra_hours_1);
    let moon_ra_sec = macros::dh_sec(moon_ra_hours_1);
    let moon_dec_deg = macros::dd_deg(moon_dec_deg1);
    let moon_dec_min = macros::dd_min(moon_dec_deg1);
    let moon_dec_sec = macros::dd_sec(moon_dec_deg1);
    let earth_moon_dist_km = util::round_f64(earth_moon_distance_km, 0);
    let moon_hor_parallax_deg = util::round_f64(moon_horizontal_parallax_deg, 6);

    return (
        moon_ra_hour as f64,
        moon_ra_min as f64,
        moon_ra_sec,
        moon_dec_deg,
        moon_dec_min,
        moon_dec_sec,
        earth_moon_dist_km,
        moon_hor_parallax_deg,
    );
}

/// Calculate Moon phase and position angle of bright limb.
///
/// ## Arguments
/// * `lct_hour` -- Local civil time, in hours.
/// * `lct_min` -- Local civil time, in minutes.
/// * `lct_sec` -- Local civil time, in seconds.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
/// * `accuracy_level` -- "A" (approximate) or "P" (precise)
///
/// ## Returns
/// * `moon_phase` -- Phase of Moon, between 0 and 1, where 0 is New and 1 is Full.
/// * `pa_bright_limb_deg` -- Position angle of the bright limb (degrees)
pub fn moon_phase(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    accuracy_level: String,
) -> (f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let gdate_day = macros::lct_gday(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let gdate_month = macros::lct_gmonth(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let gdate_year = macros::lct_gyear(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let sun_long_deg = macros::sun_long(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let (moon_ecliptic_longitude_deg, moon_ecliptic_latitude_deg, _moon_horizontal_parallax_deg) =
        macros::moon_long_lat_hp(
            lct_hour,
            lct_min,
            lct_sec,
            daylight_saving,
            zone_correction_hours,
            local_date_day,
            local_date_month,
            local_date_year,
        );
    let d_rad = (moon_ecliptic_longitude_deg - sun_long_deg).to_radians();

    let moon_phase1 = if accuracy_level.to_string() == "P" {
        macros::moon_phase(
            lct_hour,
            lct_min,
            lct_sec,
            daylight_saving,
            zone_correction_hours,
            local_date_day,
            local_date_month,
            local_date_year,
        )
    } else {
        (1.0 - (d_rad).cos()) / 2.0
    };

    let sun_ra_rad = (macros::ec_ra(
        sun_long_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    ))
    .to_radians();
    let moon_ra_rad = (macros::ec_ra(
        moon_ecliptic_longitude_deg,
        0.0,
        0.0,
        moon_ecliptic_latitude_deg,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    ))
    .to_radians();
    let sun_dec_rad = (macros::ec_dec(
        sun_long_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    ))
    .to_radians();
    let moon_dec_rad = (macros::ec_dec(
        moon_ecliptic_longitude_deg,
        0.0,
        0.0,
        moon_ecliptic_latitude_deg,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    ))
    .to_radians();

    let y = (sun_dec_rad).cos() * (sun_ra_rad - moon_ra_rad).sin();
    let x = (moon_dec_rad).cos() * (sun_dec_rad).sin()
        - (moon_dec_rad).sin() * (sun_dec_rad).cos() * (sun_ra_rad - moon_ra_rad).cos();

    let chi_deg = macros::degrees(y.atan2(x));

    let moon_phase = util::round_f64(moon_phase1, 2);
    let pa_bright_limb_deg = util::round_f64(chi_deg, 2);

    return (moon_phase, pa_bright_limb_deg);
}

/// Calculate new moon and full moon instances.
///
/// ## Arguments
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
///
/// ## Returns
/// * `nm_local_time_hour` -- new Moon instant - local time (hour)
/// * `nm_local_time_min` -- new Moon instant - local time (minutes)
/// * `nm_local_date_day` -- new Moon instance - local date (day)
/// * `nm_local_date_month` -- new Moon instance - local date (month)
/// * `nm_local_date_year` -- new Moon instance - local date (year)
/// * `fm_local_time_hour` -- full Moon instant - local time (hour)
/// * `fm_local_time_min` -- full Moon instant - local time (minutes)
/// * `fm_local_date_day` -- full Moon instance - local date (day)
/// * `fm_local_date_month` -- full Moon instance - local date (month)
/// * `fm_local_date_year` -- full Moon instance - local date (year)
pub fn times_of_new_moon_and_full_moon(
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
) -> (f64, f64, f64, u32, u32, f64, f64, f64, u32, u32) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let jd_of_new_moon_days = macros::new_moon(
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let jd_of_full_moon_days = macros::full_moon(
        3,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let g_date_of_new_moon_day = macros::jdc_day(jd_of_new_moon_days);
    let integer_day1 = g_date_of_new_moon_day.floor();
    let g_date_of_new_moon_month = macros::jdc_month(jd_of_new_moon_days);
    let g_date_of_new_moon_year = macros::jdc_year(jd_of_new_moon_days);

    let g_date_of_full_moon_day = macros::jdc_day(jd_of_full_moon_days);
    let integer_day2 = g_date_of_full_moon_day.floor();
    let g_date_of_full_moon_month = macros::jdc_month(jd_of_full_moon_days);
    let g_date_of_full_moon_year = macros::jdc_year(jd_of_full_moon_days);

    let ut_of_new_moon_hours = 24.0 * (g_date_of_new_moon_day - integer_day1);
    let ut_of_full_moon_hours = 24.0 * (g_date_of_full_moon_day - integer_day2);
    let lct_of_new_moon_hours = macros::ut_lct(
        ut_of_new_moon_hours + 0.008333,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day1,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let lct_of_full_moon_hours = macros::ut_lct(
        ut_of_full_moon_hours + 0.008333,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day2,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );

    let nm_local_time_hour = macros::dh_hour(lct_of_new_moon_hours);
    let nm_local_time_min = macros::dh_min(lct_of_new_moon_hours);
    let nm_local_date_day = macros::ut_lc_day(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day1,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let nm_local_date_month = macros::ut_lc_month(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day1,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let nm_local_date_year = macros::ut_lc_year(
        ut_of_new_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day1,
        g_date_of_new_moon_month,
        g_date_of_new_moon_year,
    );
    let fm_local_time_hour = macros::dh_hour(lct_of_full_moon_hours);
    let fm_local_time_min = macros::dh_min(lct_of_full_moon_hours);
    let fm_local_date_day = macros::ut_lc_day(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day2,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let fm_local_date_month = macros::ut_lc_month(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day2,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );
    let fm_local_date_year = macros::ut_lc_year(
        ut_of_full_moon_hours,
        0.0,
        0.0,
        daylight_saving,
        zone_correction_hours,
        integer_day2,
        g_date_of_full_moon_month,
        g_date_of_full_moon_year,
    );

    return (
        nm_local_time_hour as f64,
        nm_local_time_min as f64,
        nm_local_date_day,
        nm_local_date_month,
        nm_local_date_year,
        fm_local_time_hour as f64,
        fm_local_time_min as f64,
        fm_local_date_day,
        fm_local_date_month,
        fm_local_date_year,
    );
}

/// Calculate Moon's distance, angular diameter, and horizontal parallax.
///
/// ## Arguments
/// * `lct_hour` -- Local civil time, in hours.
/// * `lct_min` -- Local civil time, in minutes.
/// * `lct_sec` -- Local civil time, in seconds.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
///
/// ## Returns
/// * `earth_moon_dist` -- Earth-Moon distance (km)
/// * `ang_diameter_deg` -- Angular diameter (degrees part)
/// * `ang_diameter_min` -- Angular diameter (minutes part)
/// * `hor_parallax_deg` -- Horizontal parallax (degrees part)
/// * `hor_parallax_min` -- Horizontal parallax (minutes part)
/// * `hor_parallax_sec` -- Horizontal parallax (seconds part)
pub fn moon_dist_ang_diam_hor_parallax(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
) -> (f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let moon_distance = macros::moon_dist(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let moon_angular_diameter = macros::moon_size(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let moon_horizontal_parallax = macros::moon_hp(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    // earth_moon_dist = round(moon_distance,-1)
    let earth_moon_dist = util::round_f64(moon_distance, 0);
    let ang_diameter_deg = macros::dd_deg(moon_angular_diameter + 0.008333);
    let ang_diameter_min = macros::dd_min(moon_angular_diameter + 0.008333);
    let hor_parallax_deg = macros::dd_deg(moon_horizontal_parallax);
    let hor_parallax_min = macros::dd_min(moon_horizontal_parallax);
    let hor_parallax_sec = macros::dd_sec(moon_horizontal_parallax);

    return (
        earth_moon_dist,
        ang_diameter_deg,
        ang_diameter_min,
        hor_parallax_deg,
        hor_parallax_min,
        hor_parallax_sec,
    );
}

/// Calculate date/time of local moonrise and moonset.
///
/// ## Arguments
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `geog_long_deg` -- Geographical longitude, in degrees.
/// * `geog_lat_deg` -- Geographical latitude, in degrees.
///
/// ## Returns
/// * `mr_lt_hour` -- Moonrise, local time (hour part)
/// * `mr_lt_min` -- Moonrise, local time (minutes part)
/// * `mr_local_date_day` -- Moonrise, local date (day)
/// * `mr_local_date_month` -- Moonrise, local date (month)
/// * `mr_local_date_year` -- Moonrise, local date (year)
/// * `mr_azimuth_deg` -- Moonrise, azimuth (degrees)
/// * `ms_lt_hour` -- Moonset, local time (hour part)
/// * `ms_lt_min` -- Moonset, local time (minutes part)
/// * `ms_local_date_day` -- Moonset, local date (day)
/// * `ms_local_date_month` -- Moonset, local date (month)
/// * `ms_local_date_year` -- Moonset, local date (year)
/// * `ms_azimuth_deg` -- Moonset, azimuth (degrees)
pub fn moonrise_and_moonset(
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    geog_long_deg: f64,
    geog_lat_deg: f64,
) -> (f64, f64, f64, u32, u32, f64, f64, f64, f64, u32, u32, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let local_time_of_moonrise_hours = macros::moon_rise_lct(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_long_deg,
        geog_lat_deg,
    );
    let _local_moonrise_status1 = macros::e_moon_rise(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_long_deg,
        geog_lat_deg,
    );
    let (local_date_of_moonrise_day, local_date_of_moonrise_month, local_date_of_moonrise_year) =
        macros::moon_rise_lc_dmy(
            local_date_day,
            local_date_month,
            local_date_year,
            daylight_saving,
            zone_correction_hours,
            geog_long_deg,
            geog_lat_deg,
        );
    let local_azimuth_deg1 = macros::moon_rise_az(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_long_deg,
        geog_lat_deg,
    );

    let local_time_of_moonset_hours = macros::moon_set_lct(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_long_deg,
        geog_lat_deg,
    );
    let _local_moonset_status1 = macros::e_moon_set(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_long_deg,
        geog_lat_deg,
    );
    let (local_date_of_moonset_day, local_date_of_moonset_month, local_date_of_moonset_year) =
        macros::moon_set_lc_dmy(
            local_date_day,
            local_date_month,
            local_date_year,
            daylight_saving,
            zone_correction_hours,
            geog_long_deg,
            geog_lat_deg,
        );
    let local_azimuth_deg2 = macros::moon_set_az(
        local_date_day,
        local_date_month,
        local_date_year,
        daylight_saving,
        zone_correction_hours,
        geog_long_deg,
        geog_lat_deg,
    );

    let mr_lt_hour = macros::dh_hour(local_time_of_moonrise_hours + 0.008333);
    let mr_lt_min = macros::dh_min(local_time_of_moonrise_hours + 0.008333);
    let mr_local_date_day = local_date_of_moonrise_day;
    let mr_local_date_month = local_date_of_moonrise_month;
    let mr_local_date_year = local_date_of_moonrise_year;
    let mr_azimuth_deg = util::round_f64(local_azimuth_deg1, 2);
    let ms_lt_hour = macros::dh_hour(local_time_of_moonset_hours + 0.008333);
    let ms_lt_min = macros::dh_min(local_time_of_moonset_hours + 0.008333);
    let ms_local_date_day = local_date_of_moonset_day;
    let ms_local_date_month = local_date_of_moonset_month;
    let ms_local_date_year = local_date_of_moonset_year;
    let ms_azimuth_deg = util::round_f64(local_azimuth_deg2, 2);

    return (
        mr_lt_hour as f64,
        mr_lt_min as f64,
        mr_local_date_day,
        mr_local_date_month,
        mr_local_date_year,
        mr_azimuth_deg,
        ms_lt_hour as f64,
        ms_lt_min as f64,
        ms_local_date_day,
        ms_local_date_month,
        ms_local_date_year,
        ms_azimuth_deg,
    );
}
