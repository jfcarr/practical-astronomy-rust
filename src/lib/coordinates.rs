use crate::lib::macros;
use crate::lib::util as utils;

/// Convert an Angle (degrees, minutes, and seconds) to Decimal Degrees
pub fn angle_to_decimal_degrees(degrees: f64, minutes: f64, seconds: f64) -> f64 {
    let a = seconds.abs() / 60.0;
    let b = (minutes.abs() + a) / 60.0;
    let c = degrees.abs() + b;
    let d = if degrees < 0.0 || minutes < 0.0 || seconds < 0.0 {
        -c
    } else {
        c
    };

    return d;
}

/// Convert Decimal Degrees to an Angle (degrees, minutes, and seconds)
///
/// ## Returns
/// degrees, minutes, seconds
pub fn decimal_degrees_to_angle(decimal_degrees: f64) -> (f64, f64, f64) {
    let unsigned_decimal = decimal_degrees.abs();
    let total_seconds = unsigned_decimal * 3600.0;
    let seconds_2_dp = utils::round_f64(total_seconds % 60.0, 2);
    let corrected_seconds = if seconds_2_dp == 60.0 {
        0.0
    } else {
        seconds_2_dp
    };
    let corrected_remainder = if seconds_2_dp == 60.0 {
        total_seconds + 60.0
    } else {
        total_seconds
    };
    let minutes = (corrected_remainder / 60.0).floor() % 60.0;
    let unsigned_degrees = (corrected_remainder / 3600.0).floor();
    let signed_degrees = if decimal_degrees < 0.0 {
        -1.0 * unsigned_degrees
    } else {
        unsigned_degrees
    };

    return (signed_degrees, minutes, corrected_seconds.floor());
}

/// Convert Right Ascension to Hour Angle
pub fn right_ascension_to_hour_angle(
    ra_hours: f64,
    ra_minutes: f64,
    ra_seconds: f64,
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    is_daylight_saving: bool,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    geographical_longitude: f64,
) -> (f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let hour_angle = macros::ra_ha(
        ra_hours,
        ra_minutes,
        ra_seconds,
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
        geographical_longitude,
    );

    let hour_angle_hours = macros::dh_hour(hour_angle);
    let hour_angle_minutes = macros::dh_min(hour_angle);
    let hour_angle_seconds = macros::dh_sec(hour_angle);

    return (
        hour_angle_hours as f64,
        hour_angle_minutes as f64,
        hour_angle_seconds,
    );
}

/// Convert Hour Angle to Right Ascension
pub fn hour_angle_to_right_ascension(
    hour_angle_hours: f64,
    hour_angle_minutes: f64,
    hour_angle_seconds: f64,
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    is_daylight_saving: bool,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    geographical_longitude: f64,
) -> (f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let right_ascension = macros::ha_ra(
        hour_angle_hours,
        hour_angle_minutes,
        hour_angle_seconds,
        lct_hours,
        lct_minutes,
        lct_seconds as f64,
        daylight_saving,
        zone_correction,
        local_day as f64,
        local_month,
        local_year,
        geographical_longitude,
    );

    let right_ascension_hours = macros::dh_hour(right_ascension);
    let right_ascension_minutes = macros::dh_min(right_ascension);
    let right_ascension_seconds = macros::dh_sec(right_ascension);

    return (
        right_ascension_hours as f64,
        right_ascension_minutes as f64,
        right_ascension_seconds,
    );
}

/// Convert Equatorial Coordinates to Horizon Coordinates
pub fn equatorial_coordinates_to_horizon_coordinates(
    hour_angle_hours: f64,
    hour_angle_minutes: f64,
    hour_angle_seconds: f64,
    declination_degrees: f64,
    declination_minutes: f64,
    declination_seconds: f64,
    geographical_latitude: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let azimuth_in_decimal_degrees = macros::eq_az(
        hour_angle_hours,
        hour_angle_minutes,
        hour_angle_seconds,
        declination_degrees,
        declination_minutes,
        declination_seconds,
        geographical_latitude,
    );

    let altitude_in_decimal_degrees = macros::eq_alt(
        hour_angle_hours,
        hour_angle_minutes,
        hour_angle_seconds,
        declination_degrees,
        declination_minutes,
        declination_seconds,
        geographical_latitude,
    );

    let azimuth_degrees = macros::dd_deg(azimuth_in_decimal_degrees);
    let azimuth_minutes = macros::dd_min(azimuth_in_decimal_degrees);
    let azimuth_seconds = macros::dd_sec(azimuth_in_decimal_degrees);

    let altitude_degrees = macros::dd_deg(altitude_in_decimal_degrees);
    let altitude_minutes = macros::dd_min(altitude_in_decimal_degrees);
    let altitude_seconds = macros::dd_sec(altitude_in_decimal_degrees);

    return (
        azimuth_degrees,
        azimuth_minutes,
        azimuth_seconds,
        altitude_degrees,
        altitude_minutes,
        altitude_seconds,
    );
}

/// Convert Horizon Coordinates to Equatorial Coordinates
pub fn horizon_coordinates_to_equatorial_coordinates(
    azimuth_degrees: f64,
    azimuth_minutes: f64,
    azimuth_seconds: f64,
    altitude_degrees: f64,
    altitude_minutes: f64,
    altitude_seconds: f64,
    geographical_latitude: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let hour_angle_in_decimal_degrees = macros::hor_ha(
        azimuth_degrees,
        azimuth_minutes,
        azimuth_seconds,
        altitude_degrees,
        altitude_minutes,
        altitude_seconds,
        geographical_latitude,
    );

    let declination_in_decimal_degrees = macros::hor_dec(
        azimuth_degrees,
        azimuth_minutes,
        azimuth_seconds,
        altitude_degrees,
        altitude_minutes,
        altitude_seconds,
        geographical_latitude,
    );

    let hour_angle_hours = macros::dh_hour(hour_angle_in_decimal_degrees);
    let hour_angle_minutes = macros::dh_min(hour_angle_in_decimal_degrees);
    let hour_angle_seconds = macros::dh_sec(hour_angle_in_decimal_degrees);

    let declination_degrees = macros::dd_deg(declination_in_decimal_degrees);
    let declination_minutes = macros::dd_min(declination_in_decimal_degrees);
    let declination_seconds = macros::dd_sec(declination_in_decimal_degrees);

    return (
        hour_angle_hours as f64,
        hour_angle_minutes as f64,
        hour_angle_seconds,
        declination_degrees,
        declination_minutes,
        declination_seconds,
    );
}

/// Calculate Mean Obliquity of the Ecliptic for a Greenwich Date
pub fn mean_obliquity_of_the_ecliptic(
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> f64 {
    let jd = macros::cd_jd(greenwich_day, greenwich_month, greenwich_year);
    let mjd = jd - 2451545.0;
    let t = mjd / 36525.0;
    let de1 = t * (46.815 + t * (0.0006 - (t * 0.00181)));
    let de2 = de1 / 3600.0;

    return 23.439292 - de2;
}

/// Convert Ecliptic Coordinates to Equatorial Coordinates
pub fn ecliptic_coordinate_to_equatorial_coordinate(
    ecliptic_longitude_degrees: f64,
    ecliptic_longitude_minutes: f64,
    ecliptic_longitude_seconds: f64,
    ecliptic_latitude_degrees: f64,
    ecliptic_latitude_minutes: f64,
    ecliptic_latitude_seconds: f64,
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> (f64, f64, f64, f64, f64, f64) {
    let eclon_deg = macros::dms_dd(
        ecliptic_longitude_degrees,
        ecliptic_longitude_minutes,
        ecliptic_longitude_seconds,
    );
    let eclat_deg = macros::dms_dd(
        ecliptic_latitude_degrees,
        ecliptic_latitude_minutes,
        ecliptic_latitude_seconds,
    );
    let eclon_rad = eclon_deg.to_radians();
    let eclat_rad = eclat_deg.to_radians();
    let obliq_deg = macros::obliq(greenwich_day, greenwich_month, greenwich_year);
    let obliq_rad = obliq_deg.to_radians();
    let sin_dec =
        eclat_rad.sin() * obliq_rad.cos() + eclat_rad.cos() * obliq_rad.sin() * eclon_rad.sin();
    let dec_rad = sin_dec.asin();
    let dec_deg = macros::degrees(dec_rad);
    let y = eclon_rad.sin() * obliq_rad.cos() - eclat_rad.tan() * obliq_rad.sin();
    let x = eclon_rad.cos();
    let ra_rad = y.atan2(x);
    let ra_deg1 = macros::degrees(ra_rad);
    let ra_deg2 = ra_deg1 - 360.0 * (ra_deg1 / 360.0).floor();
    let ra_hours = macros::dd_dh(ra_deg2);

    let out_ra_hours = macros::dh_hour(ra_hours);
    let out_ra_minutes = macros::dh_min(ra_hours);
    let out_ra_seconds = macros::dh_sec(ra_hours);
    let out_dec_degrees = macros::dd_deg(dec_deg);
    let out_dec_minutes = macros::dd_min(dec_deg);
    let out_dec_seconds = macros::dd_sec(dec_deg);

    return (
        out_ra_hours as f64,
        out_ra_minutes as f64,
        out_ra_seconds,
        out_dec_degrees,
        out_dec_minutes,
        out_dec_seconds,
    );
}

/// Convert Equatorial Coordinates to Ecliptic Coordinates
pub fn equatorial_coordinate_to_ecliptic_coordinate(
    ra_hours: f64,
    ra_minutes: f64,
    ra_seconds: f64,
    dec_degrees: f64,
    dec_minutes: f64,
    dec_seconds: f64,
    gw_day: f64,
    gw_month: u32,
    gw_year: u32,
) -> (f64, f64, f64, f64, f64, f64) {
    let ra_deg = macros::dh_dd(macros::hms_dh(ra_hours, ra_minutes, ra_seconds));
    let dec_deg = macros::dms_dd(dec_degrees, dec_minutes, dec_seconds);
    let ra_rad = ra_deg.to_radians();
    let dec_rad = dec_deg.to_radians();
    let obliq_deg = macros::obliq(gw_day, gw_month, gw_year);
    let obliq_rad = obliq_deg.to_radians();
    let sin_ecl_lat =
        dec_rad.sin() * obliq_rad.cos() - dec_rad.cos() * obliq_rad.sin() * ra_rad.sin();
    let ecl_lat_rad = sin_ecl_lat.asin();
    let ecl_lat_deg = macros::degrees(ecl_lat_rad);
    let y = ra_rad.sin() * obliq_rad.cos() + dec_rad.tan() * obliq_rad.sin();
    let x = ra_rad.cos();
    let ecl_long_rad = y.atan2(x);
    let ecl_long_deg1 = macros::degrees(ecl_long_rad);
    let ecl_long_deg2 = ecl_long_deg1 - 360.0 * (ecl_long_deg1 / 360.0).floor();

    let out_ecl_long_deg = macros::dd_deg(ecl_long_deg2);
    let out_ecl_long_min = macros::dd_min(ecl_long_deg2);
    let out_ecl_long_sec = macros::dd_sec(ecl_long_deg2);
    let out_ecl_lat_deg = macros::dd_deg(ecl_lat_deg);
    let out_ecl_lat_min = macros::dd_min(ecl_lat_deg);
    let out_ecl_lat_sec = macros::dd_sec(ecl_lat_deg);

    return (
        out_ecl_long_deg,
        out_ecl_long_min,
        out_ecl_long_sec,
        out_ecl_lat_deg,
        out_ecl_lat_min,
        out_ecl_lat_sec,
    );
}

/// Convert Equatorial Coordinates to Galactic Coordinates
pub fn equatorial_coordinate_to_galactic_coordinate(
    ra_hours: f64,
    ra_minutes: f64,
    ra_seconds: f64,
    dec_degrees: f64,
    dec_minutes: f64,
    dec_seconds: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let ra_deg = macros::dh_dd(macros::hms_dh(ra_hours, ra_minutes, ra_seconds));
    let dec_deg = macros::dms_dd(dec_degrees, dec_minutes, dec_seconds);
    let ra_rad = ra_deg.to_radians();
    let dec_rad = dec_deg.to_radians();
    let sin_b = dec_rad.cos()
        * (27.4 as f64).to_radians().cos()
        * (ra_rad - (192.25 as f64).to_radians()).cos()
        + dec_rad.sin() * (27.4 as f64).to_radians().sin();
    let b_radians = sin_b.asin();
    let b_deg = macros::degrees(b_radians);
    let y = dec_rad.sin() - sin_b * (27.4 as f64).to_radians().sin();
    let x = dec_rad.cos()
        * (ra_rad - (192.25 as f64).to_radians()).sin()
        * (27.4 as f64).to_radians().cos();
    let long_deg1 = macros::degrees(y.atan2(x)) + 33.0;
    let long_deg2 = long_deg1 - 360.0 * (long_deg1 / 360.0).floor();

    let gal_long_deg = macros::dd_deg(long_deg2);
    let gal_long_min = macros::dd_min(long_deg2);
    let gal_long_sec = macros::dd_sec(long_deg2);
    let gal_lat_deg = macros::dd_deg(b_deg);
    let gal_lat_min = macros::dd_min(b_deg);
    let gal_lat_sec = macros::dd_sec(b_deg);

    return (
        gal_long_deg,
        gal_long_min,
        gal_long_sec,
        gal_lat_deg,
        gal_lat_min,
        gal_lat_sec,
    );
}

/// Convert Galactic Coordinates to Equatorial Coordinates
pub fn galactic_coordinate_to_equatorial_coordinate(
    gal_long_deg: f64,
    gal_long_min: f64,
    gal_long_sec: f64,
    gal_lat_deg: f64,
    gal_lat_min: f64,
    gal_lat_sec: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let glong_deg = macros::dms_dd(gal_long_deg, gal_long_min, gal_long_sec);
    let glat_deg = macros::dms_dd(gal_lat_deg, gal_lat_min, gal_lat_sec);
    let glong_rad = glong_deg.to_radians();
    let glat_rad = glat_deg.to_radians();
    let sin_dec = glat_rad.cos()
        * (27.4 as f64).to_radians().cos()
        * (glong_rad - (33 as f64).to_radians()).sin()
        + glat_rad.sin() * (27.4 as f64).to_radians().sin();
    let dec_radians = sin_dec.asin();
    let dec_deg = macros::degrees(dec_radians);
    let y = glat_rad.cos() * (glong_rad - (33 as f64).to_radians()).cos();
    let x = glat_rad.sin() * ((27.4 as f64).to_radians()).cos()
        - (glat_rad).cos()
            * ((27.4 as f64).to_radians()).sin()
            * (glong_rad - (33 as f64).to_radians()).sin();

    let ra_deg1 = macros::degrees(y.atan2(x)) + 192.25;
    let ra_deg2 = ra_deg1 - 360.0 * (ra_deg1 / 360.0).floor();
    let ra_hours1 = macros::dd_dh(ra_deg2);

    let ra_hours = macros::dh_hour(ra_hours1);
    let ra_minutes = macros::dh_min(ra_hours1);
    let ra_seconds = macros::dh_sec(ra_hours1);
    let dec_degrees = macros::dd_deg(dec_deg);
    let dec_minutes = macros::dd_min(dec_deg);
    let dec_seconds = macros::dd_sec(dec_deg);

    return (
        ra_hours as f64,
        ra_minutes as f64,
        ra_seconds,
        dec_degrees,
        dec_minutes,
        dec_seconds,
    );
}

/// Calculate the angle between two celestial objects
pub fn angle_between_two_objects(
    ra_long_1_hour_deg: f64,
    ra_long_1_min: f64,
    ra_long_1_sec: f64,
    dec_lat_1_deg: f64,
    dec_lat_1_min: f64,
    dec_lat_1_sec: f64,
    ra_long_2_hour_deg: f64,
    ra_long_2_min: f64,
    ra_long_2_sec: f64,
    dec_lat_2_deg: f64,
    dec_lat_2_min: f64,
    dec_lat_2_sec: f64,
    hour_or_degree: String,
) -> (f64, f64, f64) {
    let ra_long_1_decimal = if hour_or_degree == "H" {
        macros::hms_dh(ra_long_1_hour_deg, ra_long_1_min, ra_long_1_sec)
    } else {
        macros::dms_dd(ra_long_1_hour_deg, ra_long_1_min, ra_long_1_sec)
    };
    let ra_long_1_deg = if hour_or_degree == "H" {
        macros::dh_dd(ra_long_1_decimal)
    } else {
        ra_long_1_decimal
    };
    let ra_long_1_rad = ra_long_1_deg.to_radians();
    let dec_lat_1_deg1 = macros::dms_dd(dec_lat_1_deg, dec_lat_1_min, dec_lat_1_sec);
    let dec_lat_1_rad = dec_lat_1_deg1.to_radians();
    let ra_long_2_decimal = if hour_or_degree == "H" {
        macros::hms_dh(ra_long_2_hour_deg, ra_long_2_min, ra_long_2_sec)
    } else {
        macros::dms_dd(ra_long_2_hour_deg, ra_long_2_min, ra_long_2_sec)
    };
    let ra_long_2_deg = if hour_or_degree == "H" {
        macros::dh_dd(ra_long_2_decimal)
    } else {
        ra_long_2_decimal
    };
    let ra_long_2_rad = ra_long_2_deg.to_radians();
    let dec_lat_2_deg1 = macros::dms_dd(dec_lat_2_deg, dec_lat_2_min, dec_lat_2_sec);
    let dec_lat_2_rad = dec_lat_2_deg1.to_radians();

    let cos_d = dec_lat_1_rad.sin() * dec_lat_2_rad.sin()
        + dec_lat_1_rad.cos() * dec_lat_2_rad.cos() * (ra_long_1_rad - ra_long_2_rad).cos();
    let d_rad = cos_d.acos();
    let d_deg = macros::degrees(d_rad);

    let angle_deg = macros::dd_deg(d_deg);
    let angle_min = macros::dd_min(d_deg);
    let angle_sec = macros::dd_sec(d_deg);

    return (angle_deg, angle_min, angle_sec);
}

/// Rising and setting times
///
/// ## Arguments
/// * `ra_hours` -- Right Ascension, in hours.
/// * `ra_minutes` -- Right Ascension, in minutes.
/// * `ra_seconds` -- Right Ascension, in seconds.
/// * `dec_deg` -- Declination, in degrees.
/// * `dec_min` -- Declination, in minutes.
/// * `dec_sec` -- Declination, in seconds.
/// * `gw_date_day` -- Greenwich Date, day part.
/// * `gw_date_month` -- Greenwich Date, month part.
/// * `gw_date_year` -- Greenwich Date, year part.
/// * `geog_long_deg` -- Geographical Longitude, in degrees.
/// * `geog_lat_deg` -- Geographical Latitude, in degrees.
/// * `vert_shift_deg` -- Vertical Shift, in degrees.
///
/// ## Returns
/// * `rise_set_status` -- "Never Rises", "Circumpolar", or "OK".
/// * `ut_rise_hour` -- Rise time, UT, hour part.
/// * `ut_rise_min` -- Rise time, UT, minute part.
/// * `ut_set_hour` -- Set time, UT, hour part.
/// * `ut_set_min` -- Set time, UT, minute part.
/// * `az_rise` -- Azimuth angle, at rise.
/// * `az_set` -- Azimuth angle, at set.
pub fn rising_and_setting(
    ra_hours: f64,
    ra_minutes: f64,
    ra_seconds: f64,
    dec_deg: f64,
    dec_min: f64,
    dec_sec: f64,
    gw_date_day: f64,
    gw_date_month: u32,
    gw_date_year: u32,
    geog_long_deg: f64,
    geog_lat_deg: f64,
    vert_shift_deg: f64,
) -> (String, f64, f64, f64, f64, f64, f64) {
    let ra_hours1 = macros::hms_dh(ra_hours, ra_minutes, ra_seconds);
    let dec_rad = (macros::dms_dd(dec_deg, dec_min, dec_sec)).to_radians();
    let vertical_displ_radians = (vert_shift_deg).to_radians();
    let geo_lat_radians = (geog_lat_deg).to_radians();
    let cos_h = -((vertical_displ_radians).sin() + (geo_lat_radians).sin() * (dec_rad).sin())
        / ((geo_lat_radians).cos() * (dec_rad).cos());
    let h_hours = macros::dd_dh(macros::degrees((cos_h).acos()));
    let lst_rise_hours = (ra_hours1 - h_hours) - 24.0 * ((ra_hours1 - h_hours) / 24.0).floor();
    let lst_set_hours = (ra_hours1 + h_hours) - 24.0 * ((ra_hours1 + h_hours) / 24.0).floor();
    let a_deg = macros::degrees(
        (((dec_rad).sin() + (vertical_displ_radians).sin() * (geo_lat_radians).sin())
            / ((vertical_displ_radians).cos() * (geo_lat_radians).cos()))
        .acos(),
    );
    let az_rise_deg = a_deg - 360.0 * (a_deg / 360.0).floor();
    let az_set_deg = (360.0 - a_deg) - 360.0 * ((360.0 - a_deg) / 360.0).floor();
    let ut_rise_hours1 = macros::gst_ut(
        macros::lst_gst(lst_rise_hours, 0.0, 0.0, geog_long_deg),
        0.0,
        0.0,
        gw_date_day,
        gw_date_month,
        gw_date_year,
    );
    let ut_set_hours1 = macros::gst_ut(
        macros::lst_gst(lst_set_hours, 0.0, 0.0, geog_long_deg),
        0.0,
        0.0,
        gw_date_day,
        gw_date_month,
        gw_date_year,
    );
    let ut_rise_adjusted_hours = ut_rise_hours1 + 0.008333;
    let ut_set_adjusted_hours = ut_set_hours1 + 0.008333;

    let mut rise_set_status = "OK";
    if cos_h > 1.0 {
        rise_set_status = "never rises";
    }
    if cos_h < -1.0 {
        rise_set_status = "circumpolar";
    }

    let ut_rise_hour = if rise_set_status == "OK" {
        macros::dh_hour(ut_rise_adjusted_hours) as f64
    } else {
        0.0
    };
    let ut_rise_min = if rise_set_status == "OK" {
        macros::dh_min(ut_rise_adjusted_hours) as f64
    } else {
        0.0
    };
    let ut_set_hour = if rise_set_status == "OK" {
        macros::dh_hour(ut_set_adjusted_hours) as f64
    } else {
        0.0
    };
    let ut_set_min = if rise_set_status == "OK" {
        macros::dh_min(ut_set_adjusted_hours) as f64
    } else {
        0.0
    };
    let az_rise = if rise_set_status == "OK" {
        utils::round_f64(az_rise_deg, 2)
    } else {
        0.0
    };
    let az_set = if rise_set_status == "OK" {
        utils::round_f64(az_set_deg, 2)
    } else {
        0.0
    };

    return (
        rise_set_status.to_string(),
        ut_rise_hour,
        ut_rise_min,
        ut_set_hour,
        ut_set_min,
        az_rise,
        az_set,
    );
}

/// Calculate precession (corrected coordinates between two epochs)
///
/// ## Returns
/// * corrected RA hour
/// * corrected RA minutes
/// * corrected RA seconds
/// * corrected Declination degrees
/// * corrected Declination minutes
/// * corrected Declination seconds
pub fn correct_for_precession(
    ra_hour: f64,
    ra_minutes: f64,
    ra_seconds: f64,
    dec_deg: f64,
    dec_minutes: f64,
    dec_seconds: f64,
    epoch1_day: f64,
    epoch1_month: u32,
    epoch1_year: u32,
    epoch2_day: f64,
    epoch2_month: u32,
    epoch2_year: u32,
) -> (f64, f64, f64, f64, f64, f64) {
    let ra_1_rad = (macros::dh_dd(macros::hms_dh(ra_hour, ra_minutes, ra_seconds))).to_radians();
    let dec_1_rad = (macros::dms_dd(dec_deg, dec_minutes, dec_seconds)).to_radians();
    let t_centuries = (macros::cd_jd(epoch1_day, epoch1_month, epoch1_year) - 2415020.0) / 36525.0;
    let m_sec = 3.07234 + (0.00186 * t_centuries);
    let n_arcsec = 20.0468 - (0.0085 * t_centuries);
    let n_years = (macros::cd_jd(epoch2_day, epoch2_month, epoch2_year)
        - macros::cd_jd(epoch1_day, epoch1_month, epoch1_year))
        / 365.25;
    let s1_hours =
        ((m_sec + (n_arcsec * (ra_1_rad).sin() * (dec_1_rad).tan() / 15.0)) * n_years) / 3600.0;
    let ra_2_hours = macros::hms_dh(ra_hour, ra_minutes, ra_seconds) + s1_hours;
    let s2_deg = (n_arcsec * (ra_1_rad).cos() * n_years) / 3600.0;
    let dec_2_deg = macros::dms_dd(dec_deg, dec_minutes, dec_seconds) + s2_deg;

    let corrected_ra_hour = macros::dh_hour(ra_2_hours);
    let corrected_ra_minutes = macros::dh_min(ra_2_hours);
    let corrected_ra_seconds = macros::dh_sec(ra_2_hours);
    let corrected_dec_deg = macros::dd_deg(dec_2_deg);
    let corrected_dec_minutes = macros::dd_min(dec_2_deg);
    let corrected_dec_seconds = macros::dd_sec(dec_2_deg);

    return (
        corrected_ra_hour as f64,
        corrected_ra_minutes as f64,
        corrected_ra_seconds,
        corrected_dec_deg,
        corrected_dec_minutes,
        corrected_dec_seconds,
    );
}

/// Calculate nutation for two values: ecliptic longitude and obliquity, for a Greenwich date.
///
/// ## Returns
/// * nutation in ecliptic longitude (degrees)
/// * nutation in obliquity (degrees)
pub fn nutation_in_ecliptic_longitude_and_obliquity(
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> (f64, f64) {
    let jd_days = macros::cd_jd(greenwich_day, greenwich_month, greenwich_year);
    let t_centuries = (jd_days - 2415020.0) / 36525.0;
    let a_deg = 100.0021358 * t_centuries;
    let l_1_deg = 279.6967 + (0.000303 * t_centuries * t_centuries);
    let l_deg1 = l_1_deg + 360.0 * (a_deg - (a_deg).floor());
    let l_deg2 = l_deg1 - 360.0 * (l_deg1 / 360.0).floor();
    let l_rad = (l_deg2).to_radians();
    let b_deg = 5.372617 * t_centuries;
    let n_deg1 = 259.1833 - 360.0 * (b_deg - (b_deg).floor());
    let n_deg2 = n_deg1 - 360.0 * ((n_deg1 / 360.0).floor());
    let n_rad = (n_deg2).to_radians();
    let nut_in_long_arcsec = -17.2 * (n_rad).sin() - 1.3 * (2.0 * l_rad).sin();
    let nut_in_obl_arcsec = 9.2 * (n_rad).cos() + 0.5 * (2.0 * l_rad).cos();

    let nut_in_long_deg = nut_in_long_arcsec / 3600.0;
    let nut_in_obl_deg = nut_in_obl_arcsec / 3600.0;

    return (nut_in_long_deg, nut_in_obl_deg);
}

/// Correct ecliptic coordinates for the effects of aberration.
///
/// ## Returns
/// * apparent ecliptic longitude (degrees, minutes, seconds)
/// * apparent ecliptic latitude (degrees, minutes, seconds)
pub fn correct_for_aberration(
    ut_hour: f64,
    ut_minutes: f64,
    ut_seconds: f64,
    gw_day: f64,
    gw_month: u32,
    gw_year: u32,
    true_ecl_long_deg: f64,
    true_ecl_long_min: f64,
    true_ecl_long_sec: f64,
    true_ecl_lat_deg: f64,
    true_ecl_lat_min: f64,
    true_ecl_lat_sec: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let true_long_deg = macros::dms_dd(true_ecl_long_deg, true_ecl_long_min, true_ecl_long_sec);
    let true_lat_deg = macros::dms_dd(true_ecl_lat_deg, true_ecl_lat_min, true_ecl_lat_sec);
    let sun_true_long_deg = macros::sun_long(
        ut_hour, ut_minutes, ut_seconds, 0, 0, gw_day, gw_month, gw_year,
    );
    let dlong_arcsec = -20.5 * ((sun_true_long_deg - true_long_deg).to_radians()).cos()
        / ((true_lat_deg).to_radians()).cos();
    let dlat_arcsec = -20.5
        * ((sun_true_long_deg - true_long_deg).to_radians()).sin()
        * ((true_lat_deg).to_radians()).sin();
    let apparent_long_deg = true_long_deg + (dlong_arcsec / 3600.0);
    let apparent_lat_deg = true_lat_deg + (dlat_arcsec / 3600.0);

    let apparent_ecl_long_deg = macros::dd_deg(apparent_long_deg);
    let apparent_ecl_long_min = macros::dd_min(apparent_long_deg);
    let apparent_ecl_long_sec = macros::dd_sec(apparent_long_deg);
    let apparent_ecl_lat_deg = macros::dd_deg(apparent_lat_deg);
    let apparent_ecl_lat_min = macros::dd_min(apparent_lat_deg);
    let apparent_ecl_lat_sec = macros::dd_sec(apparent_lat_deg);

    return (
        apparent_ecl_long_deg,
        apparent_ecl_long_min,
        apparent_ecl_long_sec,
        apparent_ecl_lat_deg,
        apparent_ecl_lat_min,
        apparent_ecl_lat_sec,
    );
}

/// Calculate corrected RA/Dec, accounting for atmospheric refraction.
///
/// NOTE: Valid values for coordinate_type are "TRUE" and "APPARENT".
///
/// ## Returns
/// * corrected RA hours,minutes,seconds
/// * corrected Declination degrees,minutes,seconds
pub fn atmospheric_refraction(
    true_ra_hour: f64,
    true_ra_min: f64,
    true_ra_sec: f64,
    true_dec_deg: f64,
    true_dec_min: f64,
    true_dec_sec: f64,
    coordinate_type: String,
    geog_long_deg: f64,
    geog_lat_deg: f64,
    daylight_saving_hours: i32,
    timezone_hours: i32,
    lcd_day: f64,
    lcd_month: u32,
    lcd_year: u32,
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    atmospheric_pressure_mbar: f64,
    atmospheric_temperature_celsius: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let ha_hour = macros::ra_ha(
        true_ra_hour,
        true_ra_min,
        true_ra_sec,
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving_hours,
        timezone_hours,
        lcd_day,
        lcd_month,
        lcd_year,
        geog_long_deg,
    );
    let azimuth_deg = macros::eq_az(
        ha_hour,
        0.0,
        0.0,
        true_dec_deg,
        true_dec_min,
        true_dec_sec,
        geog_lat_deg,
    );
    let altitude_deg = macros::eq_alt(
        ha_hour,
        0.0,
        0.0,
        true_dec_deg,
        true_dec_min,
        true_dec_sec,
        geog_lat_deg,
    );
    let corrected_altitude_deg = macros::refract(
        altitude_deg,
        coordinate_type,
        atmospheric_pressure_mbar,
        atmospheric_temperature_celsius,
    );

    let corrected_ha_hour = macros::hor_ha(
        azimuth_deg,
        0.0,
        0.0,
        corrected_altitude_deg,
        0.0,
        0.0,
        geog_lat_deg,
    );
    let corrected_ra_hour1 = macros::ha_ra(
        corrected_ha_hour,
        0.0,
        0.0,
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving_hours,
        timezone_hours,
        lcd_day,
        lcd_month,
        lcd_year,
        geog_long_deg,
    );
    let corrected_dec_deg1 = macros::hor_dec(
        azimuth_deg,
        0.0,
        0.0,
        corrected_altitude_deg,
        0.0,
        0.0,
        geog_lat_deg,
    );

    let corrected_ra_hour = macros::dh_hour(corrected_ra_hour1);
    let corrected_ra_min = macros::dh_min(corrected_ra_hour1);
    let corrected_ra_sec = macros::dh_sec(corrected_ra_hour1);
    let corrected_dec_deg = macros::dd_deg(corrected_dec_deg1);
    let corrected_dec_min = macros::dd_min(corrected_dec_deg1);
    let corrected_dec_sec = macros::dd_sec(corrected_dec_deg1);

    return (
        corrected_ra_hour as f64,
        corrected_ra_min as f64,
        corrected_ra_sec,
        corrected_dec_deg,
        corrected_dec_min,
        corrected_dec_sec,
    );
}

/// Calculate corrected RA/Dec, accounting for geocentric parallax.
///
/// NOTE: Valid values for coordinate_type are "TRUE" and "APPARENT".
///
/// ## Returns
/// * corrected RA hours,minutes,seconds
/// * corrected Declination degrees,minutes,seconds
pub fn corrections_for_geocentric_parallax(
    ra_hour: f64,
    ra_min: f64,
    ra_sec: f64,
    dec_deg: f64,
    dec_min: f64,
    dec_sec: f64,
    coordinate_type: String,
    equatorial_hor_parallax_deg: f64,
    geog_long_deg: f64,
    geog_lat_deg: f64,
    height_m: f64,
    daylight_saving: i32,
    timezone_hours: i32,
    lcd_day: f64,
    lcd_month: u32,
    lcd_year: u32,
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let ha_hours = macros::ra_ha(
        ra_hour,
        ra_min,
        ra_sec,
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        timezone_hours,
        lcd_day,
        lcd_month,
        lcd_year,
        geog_long_deg,
    );

    let corrected_ha_hours = macros::parallax_ha(
        ha_hours,
        0.0,
        0.0,
        dec_deg,
        dec_min,
        dec_sec,
        coordinate_type.to_string(),
        geog_lat_deg,
        height_m,
        equatorial_hor_parallax_deg,
    );

    let corrected_ra_hours = macros::ha_ra(
        corrected_ha_hours,
        0.0,
        0.0,
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        timezone_hours,
        lcd_day,
        lcd_month,
        lcd_year,
        geog_long_deg,
    );

    let corrected_dec_deg1 = macros::parallax_dec(
        ha_hours,
        0.0,
        0.0,
        dec_deg,
        dec_min,
        dec_sec,
        coordinate_type.to_string(),
        geog_lat_deg,
        height_m,
        equatorial_hor_parallax_deg,
    );

    let corrected_ra_hour = macros::dh_hour(corrected_ra_hours);
    let corrected_ra_min = macros::dh_min(corrected_ra_hours);
    let corrected_ra_sec = macros::dh_sec(corrected_ra_hours);
    let corrected_dec_deg = macros::dd_deg(corrected_dec_deg1);
    let corrected_dec_min = macros::dd_min(corrected_dec_deg1);
    let corrected_dec_sec = macros::dd_sec(corrected_dec_deg1);

    return (
        corrected_ra_hour as f64,
        corrected_ra_min as f64,
        corrected_ra_sec,
        corrected_dec_deg,
        corrected_dec_min,
        corrected_dec_sec,
    );
}

/// Calculate heliographic coordinates for a given Greenwich date, with a given heliographic position angle and heliographic displacement in arc minutes.
///
/// ## Returns
/// heliographic longitude and heliographic latitude, in degrees
pub fn heliographic_coordinates(
    helio_position_angle_deg: f64,
    helio_displacement_arcmin: f64,
    gwdate_day: f64,
    gwdate_month: u32,
    gwdate_year: u32,
) -> (f64, f64) {
    let julian_date_days = macros::cd_jd(gwdate_day, gwdate_month, gwdate_year);
    let t_centuries = (julian_date_days - 2415020.0) / 36525.0;
    let long_asc_node_deg = macros::dms_dd(74.0, 22.0, 0.0) + (84.0 * t_centuries / 60.0);
    let sun_long_deg = macros::sun_long(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year);
    let y = ((long_asc_node_deg - sun_long_deg).to_radians()).sin()
        * ((macros::dms_dd(7.0, 15.0, 0.0)).to_radians()).cos();
    let x = -((long_asc_node_deg - sun_long_deg).to_radians()).cos();
    let a_deg = macros::degrees(y.atan2(x));
    let m_deg1 = 360.0 - (360.0 * (julian_date_days - 2398220.0) / 25.38);
    let m_deg2 = m_deg1 - 360.0 * (m_deg1 / 360.0).floor();
    let l0_deg1 = m_deg2 + a_deg;
    let _l0_deg2 = l0_deg1 - 360.0 * (l0_deg1 / 360.0).floor();
    let b0_rad = (((sun_long_deg - long_asc_node_deg).to_radians()).sin()
        * ((macros::dms_dd(7.0, 15.0, 0.0)).to_radians()).sin())
    .asin();
    let theta1_rad = (-((sun_long_deg).to_radians()).cos()
        * ((macros::obliq(gwdate_day, gwdate_month, gwdate_year)).to_radians()).tan())
    .atan();
    let theta2_rad = (-((long_asc_node_deg - sun_long_deg).to_radians()).cos()
        * ((macros::dms_dd(7.0, 15.0, 0.0)).to_radians()).tan())
    .atan();
    let p_deg = macros::degrees(theta1_rad + theta2_rad);
    let rho1_deg = helio_displacement_arcmin / 60.0;
    let rho_rad = (2.0 * rho1_deg
        / macros::sun_dia(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year))
    .asin()
        - (rho1_deg).to_radians();
    let b_rad = ((b0_rad).sin() * (rho_rad).cos()
        + (b0_rad).cos()
            * (rho_rad).sin()
            * ((p_deg - helio_position_angle_deg).to_radians()).cos())
    .asin();
    let b_deg = macros::degrees(b_rad);
    let l_deg1 = macros::degrees(
        ((rho_rad).sin() * ((p_deg - helio_position_angle_deg).to_radians()).sin() / (b_rad).cos())
            .asin(),
    ) + l0_deg1;
    let l_deg2 = l_deg1 - 360.0 * (l_deg1 / 360.0).floor();

    let helio_long_deg = utils::round_f64(l_deg2, 2);
    let helio_lat_deg = utils::round_f64(b_deg, 2);

    return (helio_long_deg, helio_lat_deg);
}

/// Calculate carrington rotation number for a Greenwich date
///
/// ## Returns
/// carrington rotation number
pub fn carrington_rotation_number(gwdate_day: f64, gwdate_month: u32, gwdate_year: u32) -> (i32) {
    let julian_date_days = macros::cd_jd(gwdate_day, gwdate_month, gwdate_year);

    let crn = 1690 + utils::round_f64((julian_date_days - 2444235.34) / 27.2753, 0) as i32;

    return crn;
}

/// Calculate selenographic (lunar) coordinates (sub-Earth)
///
/// ## Returns
/// * sub-earth longitude
/// * sub-earth latitude
/// * position angle of pole
pub fn selenographic_coordinates_1(
    gwdate_day: f64,
    gwdate_month: u32,
    gwdate_year: u32,
) -> (f64, f64, f64) {
    let julian_date_days = macros::cd_jd(gwdate_day, gwdate_month, gwdate_year);
    let t_centuries = (julian_date_days - 2451545.0) / 36525.0;
    let long_asc_node_deg = 125.044522 - 1934.136261 * t_centuries;
    let f1 = 93.27191 + 483202.0175 * t_centuries;
    let f2 = f1 - 360.0 * (f1 / 360.0).floor();
    let geocentric_moon_long_deg =
        macros::moon_long(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year);
    let geocentric_moon_lat_rad =
        (macros::moon_lat(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year)).to_radians();
    let inclination_rad = (macros::dms_dd(1.0, 32.0, 32.7)).to_radians();
    let node_long_rad = (long_asc_node_deg - geocentric_moon_long_deg).to_radians();
    let sin_be = -(inclination_rad).cos() * (geocentric_moon_lat_rad).sin()
        + (inclination_rad).sin() * (geocentric_moon_lat_rad).cos() * (node_long_rad).sin();
    let sub_earth_lat_deg = macros::degrees((sin_be).asin());
    let a_rad = (-(geocentric_moon_lat_rad).sin() * (inclination_rad).sin()
        - (geocentric_moon_lat_rad).cos() * (inclination_rad).cos() * (node_long_rad).sin())
    .atan2((geocentric_moon_lat_rad).cos() * (node_long_rad).cos());
    let a_deg = macros::degrees(a_rad);
    let sub_earth_long_deg1 = a_deg - f2;
    let sub_earth_long_deg2 = sub_earth_long_deg1 - 360.0 * (sub_earth_long_deg1 / 360.0).floor();
    let sub_earth_long_deg3 = if sub_earth_long_deg2 > 180.0 {
        sub_earth_long_deg2 - 360.0
    } else {
        sub_earth_long_deg2
    };
    let c1_rad = ((node_long_rad).cos() * (inclination_rad).sin()
        / ((geocentric_moon_lat_rad).cos() * (inclination_rad).cos()
            + (geocentric_moon_lat_rad).sin() * (inclination_rad).sin() * (node_long_rad).sin()))
    .atan();
    let obliquity_rad = (macros::obliq(gwdate_day, gwdate_month, gwdate_year)).to_radians();
    let c2_rad = ((obliquity_rad).sin() * ((geocentric_moon_long_deg).to_radians()).cos()
        / ((obliquity_rad).sin()
            * (geocentric_moon_lat_rad).sin()
            * ((geocentric_moon_long_deg).to_radians()).sin()
            - (obliquity_rad).cos() * (geocentric_moon_lat_rad).cos()))
    .atan();
    let c_deg = macros::degrees(c1_rad + c2_rad);

    let sub_earth_longitude = utils::round_f64(sub_earth_long_deg3, 2);
    let sub_earth_latitude = utils::round_f64(sub_earth_lat_deg, 2);
    let position_angle_of_pole = utils::round_f64(c_deg, 2);

    return (
        sub_earth_longitude,
        sub_earth_latitude,
        position_angle_of_pole,
    );
}

/// Calculate selenographic (lunar) coordinates (sub-Solar)
///
/// ## Returns
/// * sub-solar longitude
/// * sub-solar colongitude
/// * sub-solar latitude
pub fn selenographic_coordinates_2(
    gwdate_day: f64,
    gwdate_month: u32,
    gwdate_year: u32,
) -> (f64, f64, f64) {
    let julian_date_days = macros::cd_jd(gwdate_day, gwdate_month, gwdate_year);
    let t_centuries = (julian_date_days - 2451545.0) / 36525.0;
    let long_asc_node_deg = 125.044522 - 1934.136261 * t_centuries;
    let f1 = 93.27191 + 483202.0175 * t_centuries;
    let f2 = f1 - 360.0 * (f1 / 360.0).floor();
    let sun_geocentric_long_deg =
        macros::sun_long(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year);
    let moon_equ_hor_parallax_arc_min =
        macros::moon_hp(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year) * 60.0;
    let sun_earth_dist_au =
        macros::sun_dist(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year);
    let geocentric_moon_lat_rad =
        (macros::moon_lat(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year)).to_radians();
    let geocentric_moon_long_deg =
        macros::moon_long(0.0, 0.0, 0.0, 0, 0, gwdate_day, gwdate_month, gwdate_year);
    let adjusted_moon_long_deg = sun_geocentric_long_deg
        + 180.0
        + (26.4
            * (geocentric_moon_lat_rad).cos()
            * ((sun_geocentric_long_deg - geocentric_moon_long_deg).to_radians()).sin()
            / (moon_equ_hor_parallax_arc_min * sun_earth_dist_au));
    let adjusted_moon_lat_rad =
        0.14666 * geocentric_moon_lat_rad / (moon_equ_hor_parallax_arc_min * sun_earth_dist_au);
    let inclination_rad = (macros::dms_dd(1.0, 32.0, 32.7)).to_radians();
    let node_long_rad = (long_asc_node_deg - adjusted_moon_long_deg).to_radians();
    let sin_bs = -(inclination_rad).cos() * (adjusted_moon_lat_rad).sin()
        + (inclination_rad).sin() * (adjusted_moon_lat_rad).cos() * (node_long_rad).sin();
    let sub_solar_lat_deg = macros::degrees((sin_bs).asin());
    let a_rad = (-(adjusted_moon_lat_rad).sin() * (inclination_rad).sin()
        - (adjusted_moon_lat_rad).cos() * (inclination_rad).cos() * (node_long_rad).sin())
    .atan2((adjusted_moon_lat_rad).cos() * (node_long_rad).cos());
    let a_deg = macros::degrees(a_rad);
    let sub_solar_long_deg1 = a_deg - f2;
    let sub_solar_long_deg2 = sub_solar_long_deg1 - 360.0 * (sub_solar_long_deg1 / 360.0).floor();
    let sub_solar_long_deg3 = if sub_solar_long_deg2 > 180.0 {
        sub_solar_long_deg2 - 360.0
    } else {
        sub_solar_long_deg2
    };
    let sub_solar_colong_deg = 90.0 - sub_solar_long_deg3;

    let sub_solar_longitude = utils::round_f64(sub_solar_long_deg3, 2);
    let sub_solar_colongitude = utils::round_f64(sub_solar_colong_deg, 2);
    let sub_solar_latitude = utils::round_f64(sub_solar_lat_deg, 2);

    return (
        sub_solar_longitude,
        sub_solar_colongitude,
        sub_solar_latitude,
    );
}
