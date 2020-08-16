use crate::lib::types as pa_types;
use crate::lib::util as utils;

/// Convert a Civil Time (hours,minutes,seconds) to Decimal Hours
///
/// Original macro name: HMSDH
pub fn hms_dh(hours: f64, minutes: f64, seconds: f64) -> f64 {
    let f_hours = hours as f64;
    let f_minutes = minutes as f64;
    let f_seconds = seconds as f64;

    let a = f_seconds.abs() / 60.0;
    let b = (f_minutes.abs() + a) / 60.0;
    let c = f_hours.abs() + b;

    return if f_hours < 0.0 || f_minutes < 0.0 || f_seconds < 0.0 {
        -c
    } else {
        c
    };
}

/// Return the hour part of a Decimal Hours
///
/// Original macro name: DHHour
pub fn dh_hour(decimal_hours: f64) -> u32 {
    let a = decimal_hours.abs();
    let b = a * 3600.0;
    let c = utils::round_f64(b - 60.0 * (b / 60.0).floor(), 2);
    // let d = if c == 60.0 { 0.0 } else { c };
    let e = if c == 60.0 { b + 60.0 } else { b };

    return if decimal_hours < 0.0 {
        -(e / 3600.0).floor() as u32
    } else {
        (e / 3600.0).floor() as u32
    };
}

/// Return the minutes part of a Decimal Hours
///
/// Original macro name: DHMin
pub fn dh_min(decimal_hours: f64) -> u32 {
    let a = decimal_hours.abs();
    let b = a * 3600.0;
    let c = utils::round_f64(b - 60.0 * (b / 60.0).floor(), 2);
    let e = if c == 60.0 { b + 60.0 } else { b };

    return ((e / 60.0).floor() % 60.0) as u32;
}

/// Return the seconds part of a Decimal Hours
///
/// Original macro name: DHSec
pub fn dh_sec(decimal_hours: f64) -> f64 {
    let a = decimal_hours.abs();
    let b = a * 3600.0;
    let c = utils::round_f64(b - 60.0 * (b / 60.0).floor(), 2);
    let d = if c == 60.0 { 0.0 } else { c };

    return d;
}

/// Convert a Greenwich Date/Civil Date (day,month,year) to Julian Date
///
/// Original macro name: CDJD
pub fn cd_jd(day: f64, month: u32, year: u32) -> f64 {
    let f_day = day as f64;
    let f_month = month as f64;
    let f_year = year as f64;

    let y = if f_month < 3.0 { f_year - 1.0 } else { f_year };
    let m = if f_month < 3.0 {
        f_month + 12.0
    } else {
        f_month
    };

    let b: f64;

    if f_year > 1582.0 {
        let a = (y / 100.0).floor();
        b = 2.0 - a + (a / 4.0).floor();
    } else {
        if f_year == 1582.0 && f_month > 10.0 {
            let a = (y / 100.0).floor();
            b = 2.0 - a + (a / 4.0).floor();
        } else {
            if f_year == 1582.0 && f_month == 10.0 && f_day >= 15.0 {
                let a = (y / 100.0).floor();
                b = 2.0 - a + (a / 4.0).floor();
            } else {
                b = 0.0;
            }
        }
    }

    let c = if y < 0.0 {
        ((365.25 * y) - 0.75).floor()
    } else {
        (365.25 * y).floor()
    };

    let d = (30.6001 * (m + 1.0)).floor();

    return b + c + d + f_day + 1720994.5;
}

/// Returns the day part of a Julian Date
///
/// Original macro name: JDCDay
pub fn jdc_day(julian_date: f64) -> f64 {
    let i = (julian_date + 0.5).floor();
    let f = julian_date + 0.5 - i;
    let a = ((i - 1867216.25) / 36524.25).floor();
    let b = if i > 2299160.0 {
        i + 1.0 + a - (a / 4.0).floor()
    } else {
        i
    };
    let c = b + 1524.0;
    let d = ((c - 122.1) / 365.25).floor();
    let e = (365.25 * d).floor();
    let g = ((c - e) / 30.6001).floor();

    return c - e + f - (30.6001 * g).floor();
}

/// Returns the month part of a Julian Date
///
/// Original macro name: JDCMonth
pub fn jdc_month(julian_date: f64) -> u32 {
    let i = (julian_date + 0.5).floor();
    let _f = julian_date + 0.5 - i;
    let a = ((i - 1867216.25) / 36524.25).floor();
    let b = if i > 2299160.0 {
        i + 1.0 + a - (a / 4.0).floor()
    } else {
        i
    };
    let c = b + 1524.0;
    let d = ((c - 122.1) / 365.25).floor();
    let e = (365.25 * d).floor();
    let g = ((c - e) / 30.6001).floor();

    let return_value = if g < 13.5 { g - 1.0 } else { g - 13.0 };

    return return_value as u32;
}

/// Returns the year part of a Julian Date
///
/// Original macro name: JDCYear
pub fn jdc_year(julian_date: f64) -> u32 {
    let i = (julian_date + 0.5).floor();
    let _f = julian_date + 0.5 - i;
    let a = ((i - 1867216.25) / 36524.25).floor();
    let b = if i > 2299160.0 {
        i + 1.0 + a - (a / 4.0).floor()
    } else {
        i
    };
    let c = b + 1524.0;
    let d = ((c - 122.1) / 365.25).floor();
    let e = (365.25 * d).floor();
    let g = ((c - e) / 30.6001).floor();
    let h = if g < 13.5 { g - 1.0 } else { g - 13.0 };

    let return_value = if h > 2.5 { d - 4716.0 } else { d - 4715.0 };

    return return_value as u32;
}

/// Convert a Julian Date to Day-of-Week (e.g., Sunday)
///
/// Original macro name: FDOW
pub fn f_dow(julian_date: f64) -> String {
    let j = (julian_date - 0.5).floor() + 0.5;
    let n = (j + 1.5) % 7.0;

    let return_value: &str;
    match n as u32 {
        0 => return_value = "Sunday",
        1 => return_value = "Monday",
        2 => return_value = "Tuesday",
        3 => return_value = "Wednesday",
        4 => return_value = "Thursday",
        5 => return_value = "Friday",
        6 => return_value = "Saturday",
        _ => return_value = "Unknown",
    }

    return return_value.to_string();
}

/// Convert Right Ascension to Hour Angle
///
/// Original macro name: RAHA
pub fn ra_ha(
    ra_hours: f64,
    ra_minutes: f64,
    ra_seconds: f64,
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    geographical_longitude: f64,
) -> f64 {
    let a = lct_ut(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let b = lct_gday(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let c = lct_gmonth(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let d = lct_gyear(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let e = ut_gst(a, 0.0, 0.0, b, c, d);
    let f = gst_lst(e, 0.0, 0.0, geographical_longitude);
    let g = hms_dh(ra_hours, ra_minutes, ra_seconds);
    let h = f - g;

    return if h < 0.0 { 24.0 + h } else { h };
}

/// Convert Hour Angle to Right Ascension
///
/// Original macro name: HARA
pub fn ha_ra(
    hour_angle_hours: f64,
    hour_angle_minutes: f64,
    hour_angle_seconds: f64,
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    geographical_longitude: f64,
) -> f64 {
    let a = lct_ut(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let b = lct_gday(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let c = lct_gmonth(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let d = lct_gyear(
        lct_hours,
        lct_minutes,
        lct_seconds,
        daylight_saving,
        zone_correction,
        local_day,
        local_month,
        local_year,
    );
    let e = ut_gst(a, 0.0, 0.0, b, c, d);
    let f = gst_lst(e, 0.0, 0.0, geographical_longitude);
    let g = hms_dh(hour_angle_hours, hour_angle_minutes, hour_angle_seconds);
    let h = f - g;

    return if h < 0.0 { 24.0 + h } else { h };
}

/// Convert Local Civil Time to Universal Time
///
/// Original macro name: LctUT
pub fn lct_ut(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
) -> f64 {
    let a = hms_dh(lct_hours, lct_minutes, lct_seconds as f64);
    let b = a - daylight_saving as f64 - zone_correction as f64;
    let c = local_day as f64 + (b / 24.0);
    let d = cd_jd(c, local_month, local_year);
    let e = jdc_day(d);
    let e1 = e.floor();

    return 24.0 * (e - e1);
}

/// Convert Universal Time to Local Civil Time
///
/// Original macro name: UTLct
pub fn ut_lct(
    u_hours: f64,
    u_minutes: f64,
    u_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> f64 {
    let a = hms_dh(u_hours, u_minutes, u_seconds);
    let b = a + zone_correction as f64;
    let c = b + daylight_saving as f64;
    let d = cd_jd(greenwich_day, greenwich_month, greenwich_year) + (c / 24.0);
    let e = jdc_day(d);
    let e1 = e.floor();

    return 24.0 * (e - e1);
}

/// Get Local Civil Day for Universal Time
///
/// Original macro name: UTLcDay
pub fn ut_lc_day(
    u_hours: f64,
    u_minutes: f64,
    u_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> f64 {
    let a = hms_dh(u_hours, u_minutes, u_seconds);
    let b = a + zone_correction as f64;
    let c = b + daylight_saving as f64;
    let d = cd_jd(greenwich_day, greenwich_month, greenwich_year) + (c / 24.0);
    let e = jdc_day(d);
    let e1 = e.floor();

    return e1;
}

/// Get Local Civil Month for Universal Time
///
/// Original macro name: UTLcMonth
pub fn ut_lc_month(
    u_hours: f64,
    u_minutes: f64,
    u_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> u32 {
    let a = hms_dh(u_hours, u_minutes, u_seconds);
    let b = a + zone_correction as f64;
    let c = b + daylight_saving as f64;
    let d = cd_jd(greenwich_day, greenwich_month, greenwich_year) + (c / 24.0);

    return jdc_month(d);
}

/// Get Local Civil Year for Universal Time
///
/// Original macro name: UTLcYear
pub fn ut_lc_year(
    u_hours: f64,
    u_minutes: f64,
    u_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> u32 {
    let a = hms_dh(u_hours, u_minutes, u_seconds);
    let b = a + zone_correction as f64;
    let c = b + daylight_saving as f64;
    let d = cd_jd(greenwich_day, greenwich_month, greenwich_year) + (c / 24.0);

    return jdc_year(d);
}

/// Determine Greenwich Day for Local Time
///
/// Original macro name: LctGDay
pub fn lct_gday(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
) -> f64 {
    let a = hms_dh(lct_hours, lct_minutes, lct_seconds as f64);
    let b = a - daylight_saving as f64 - zone_correction as f64;
    let c = local_day as f64 + (b / 24.0);
    let d = cd_jd(c, local_month, local_year);
    let e = jdc_day(d);

    return e.floor();
}

/// Determine Greenwich Month for Local Time
///
/// Original macro name: LctGMonth
pub fn lct_gmonth(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
) -> u32 {
    let a = hms_dh(lct_hours, lct_minutes, lct_seconds as f64);
    let b = a - daylight_saving as f64 - zone_correction as f64;
    let c = local_day as f64 + (b / 24.0);
    let d = cd_jd(c, local_month, local_year);

    return jdc_month(d);
}

/// Determine Greenwich Year for Local Time
///
/// Original macro name: LctGYear
pub fn lct_gyear(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    daylight_saving: i32,
    zone_correction: i32,
    local_day: f64,
    local_month: u32,
    local_year: u32,
) -> u32 {
    let a = hms_dh(lct_hours, lct_minutes, lct_seconds as f64);
    let b = a - daylight_saving as f64 - zone_correction as f64;
    let c = local_day as f64 + (b / 24.0);
    let d = cd_jd(c, local_month, local_year);

    return jdc_year(d);
}

/// Convert Universal Time to Greenwich Sidereal Time
///
/// Original macro name: UTGST
pub fn ut_gst(
    u_hours: f64,
    u_minutes: f64,
    u_seconds: f64,
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> f64 {
    let a = cd_jd(greenwich_day as f64, greenwich_month, greenwich_year);
    let b = a - 2451545.0;
    let c = b / 36525.0;
    let d = 6.697374558 + (2400.051336 * c) + (0.000025862 * c * c);
    let e = d - (24.0 * (d / 24.0).floor());
    let f = hms_dh(u_hours, u_minutes, u_seconds);
    let g = f * 1.002737909;
    let h = e + g;
    return h - (24.0 * (h / 24.0).floor());
}

/// Convert Greenwich Sidereal Time to Local Sidereal Time
///
/// Original macro name: GSTLST
pub fn gst_lst(
    greenwich_hours: f64,
    greenwich_minutes: f64,
    greenwich_seconds: f64,
    geographical_longitude: f64,
) -> f64 {
    let a = hms_dh(greenwich_hours, greenwich_minutes, greenwich_seconds);
    let b = geographical_longitude / 15.0;
    let c = a + b;

    return c - (24.0 * (c / 24.0).floor());
}

/// Convert Equatorial Coordinates to Azimuth (in decimal degrees)
///
/// Original macro name: EQAz
pub fn eq_az(
    hour_angle_hours: f64,
    hour_angle_minutes: f64,
    hour_angle_seconds: f64,
    declination_degrees: f64,
    declination_minutes: f64,
    declination_seconds: f64,
    geographical_latitude: f64,
) -> f64 {
    let a = hms_dh(hour_angle_hours, hour_angle_minutes, hour_angle_seconds);
    let b = a * 15.0;
    let c = b.to_radians();
    let d = dms_dd(
        declination_degrees,
        declination_minutes,
        declination_seconds,
    );
    let e = d.to_radians();
    let f = geographical_latitude.to_radians();
    let g = e.sin() * f.sin() + e.cos() * f.cos() * c.cos();
    let h = -e.cos() * f.cos() * c.sin();
    let i = e.sin() - (f.sin() * g);
    let j = degrees(h.atan2(i));

    return j - 360.0 * (j / 360.0).floor();
}

/// Convert Equatorial Coordinates to Altitude (in decimal degrees)
///
/// Original macro name: EQAlt
pub fn eq_alt(
    hour_angle_hours: f64,
    hour_angle_minutes: f64,
    hour_angle_seconds: f64,
    declination_degrees: f64,
    declination_minutes: f64,
    declination_seconds: f64,
    geographical_latitude: f64,
) -> f64 {
    let a = hms_dh(hour_angle_hours, hour_angle_minutes, hour_angle_seconds);
    let b = a * 15.0;
    let c = b.to_radians();
    let d = dms_dd(
        declination_degrees,
        declination_minutes,
        declination_seconds,
    );
    let e = d.to_radians();
    let f = geographical_latitude.to_radians();
    let g = e.sin() * f.sin() + e.cos() * f.cos() * c.cos();

    return degrees(g.asin());
}

/// Convert Degrees Minutes Seconds to Decimal Degrees
///
/// Original macro name: DMSDD
pub fn dms_dd(degrees: f64, minutes: f64, seconds: f64) -> f64 {
    let a = seconds.abs() / 60.0;
    let b = (minutes.abs() + a) / 60.0;
    let c = degrees.abs() + b;

    return if degrees < 0.0 || minutes < 0.0 || seconds < 0.0 {
        -c
    } else {
        c
    };
}

/// Convert W to Degrees
///
/// Original macro name: Degrees
pub fn degrees(w: f64) -> f64 {
    return w * 57.29577951;
}

/// Return Degrees part of Decimal Degrees
///
/// Original macro name: DDDeg
pub fn dd_deg(decimal_degrees: f64) -> f64 {
    let a = decimal_degrees.abs();
    let b = a * 3600.0;
    let c = utils::round_f64(b - 60.0 * (b / 60.0).floor(), 2);
    let _d = if c == 60.0 { 0.0 } else { c };
    let e = if c == 60.0 { 60.0 } else { b };

    return if decimal_degrees < 0.0 {
        -(e / 3600.0).floor()
    } else {
        (e / 3600.0).floor()
    };
}

/// Return Minutes part of Decimal Degrees
///
/// Original macro name: DDMin
pub fn dd_min(decimal_degrees: f64) -> f64 {
    let a = decimal_degrees.abs();
    let b = a * 3600.0;
    let c = utils::round_f64(b - 60.0 * (b / 60.0).floor(), 2);
    let _d = if c == 60.0 { 0.0 } else { c };
    let e = if c == 60.0 { b + 60.0 } else { b };

    return (e / 60.0).floor() % 60.0;
}

/// Return Seconds part of Decimal Degrees
///
/// Original macro name: DDSec
pub fn dd_sec(decimal_degrees: f64) -> f64 {
    let a = decimal_degrees.abs();
    let b = a * 3600.0;
    let c = utils::round_f64(b - 60.0 * (b / 60.0).floor(), 2);
    let d = if c == 60.0 { 0.0 } else { c };

    return d;
}

/// Convert Decimal Degrees to Degree-Hours
///
/// Original macro name: DDDH
pub fn dd_dh(decimal_degrees: f64) -> f64 {
    return decimal_degrees / 15.0;
}

/// Convert Degree-Hours to Decimal Degrees
///
/// Original macro name: DHDD
pub fn dh_dd(degree_hours: f64) -> f64 {
    return degree_hours * 15.0;
}

/// Convert Horizon Coordinates to Declination (in decimal degrees)
///
/// Original macro name: HORDec
pub fn hor_dec(
    azimuth_degrees: f64,
    azimuth_minutes: f64,
    azimuth_seconds: f64,
    altitude_degrees: f64,
    altitude_minutes: f64,
    altitude_seconds: f64,
    geographical_latitude: f64,
) -> f64 {
    let a = dms_dd(azimuth_degrees, azimuth_minutes, azimuth_seconds);
    let b = dms_dd(altitude_degrees, altitude_minutes, altitude_seconds);
    let c = a.to_radians();
    let d = b.to_radians();
    let e = geographical_latitude.to_radians();
    let f = d.sin() * e.sin() + d.cos() * e.cos() * c.cos();

    return degrees(f.asin());
}

/// Convert Horizon Coordinates to Hour Angle (in decimal degrees)
///
/// Original macro name: HORHa
pub fn hor_ha(
    azimuth_degrees: f64,
    azimuth_minutes: f64,
    azimuth_seconds: f64,
    altitude_degrees: f64,
    altitude_minutes: f64,
    altitude_seconds: f64,
    geographical_latitude: f64,
) -> f64 {
    let a = dms_dd(azimuth_degrees, azimuth_minutes, azimuth_seconds);
    let b = dms_dd(altitude_degrees, altitude_minutes, altitude_seconds);
    let c = a.to_radians();
    let d = b.to_radians();
    let e = geographical_latitude.to_radians();
    let f = d.sin() * e.sin() + d.cos() * e.cos() * c.cos();
    let g = -d.cos() * e.cos() * c.sin();
    let h = d.sin() - e.sin() * f;
    let i = dd_dh(degrees(g.atan2(h)));

    return i - 24.0 * (i / 24.0).floor();
}

/// Nutation amount to be added in ecliptic longitude, in degrees.
///
/// Original macro name: NutatLong
pub fn nutat_long(gd: f64, gm: u32, gy: u32) -> f64 {
    let dj = cd_jd(gd, gm, gy) - 2415020.0;
    let t = dj / 36525.0;
    let t2 = t * t;

    let a = 100.0021358 * t;
    let b = 360.0 * (a - a.floor());

    let l1 = 279.6967 + 0.000303 * t2 + b;
    let l2 = 2.0 * l1.to_radians();

    let a = 1336.855231 * t;
    let b = 360.0 * (a - a.floor());

    let d1 = 270.4342 - 0.001133 * t2 + b;
    let d2 = 2.0 * d1.to_radians();

    let a = 99.99736056 * t;
    let b = 360.0 * (a - a.floor());

    let m1 = 358.4758 - 0.00015 * t2 + b;
    let m1 = m1.to_radians();

    let a = 1325.552359 * t;
    let b = 360.0 * (a - a.floor());

    let m2 = 296.1046 + 0.009192 * t2 + b;
    let m2 = m2.to_radians();

    let a = 5.372616667 * t;
    let b = 360.0 * (a - a.floor());

    let n1 = 259.1833 + 0.002078 * t2 - b;
    let n1 = n1.to_radians();

    let n2 = 2.0 * n1;

    let dp = (-17.2327 - 0.01737 * t) * n1.sin();
    let dp = dp + (-1.2729 - 0.00013 * t) * (l2).sin() + 0.2088 * (n2).sin();
    let dp = dp - 0.2037 * (d2).sin() + (0.1261 - 0.00031 * t) * (m1).sin();
    let dp = dp + 0.0675 * (m2).sin() - (0.0497 - 0.00012 * t) * (l2 + m1).sin();
    let dp = dp - 0.0342 * (d2 - n1).sin() - 0.0261 * (d2 + m2).sin();
    let dp = dp + 0.0214 * (l2 - m1).sin() - 0.0149 * (l2 - d2 + m2).sin();
    let dp = dp + 0.0124 * (l2 - n1).sin() + 0.0114 * (d2 - m2).sin();

    return dp / 3600.0;
}

/// Nutation of Obliquity
///
/// Original macro name: NutatObl
pub fn nutat_obl(greenwich_day: f64, greenwich_month: u32, greenwich_year: u32) -> f64 {
    let dj = cd_jd(greenwich_day, greenwich_month, greenwich_year) - 2415020.0;
    let t = dj / 36525.0;
    let t2 = t * t;

    let a = 100.0021358 * t;
    let b = 360.0 * (a - a.floor());

    let l1 = 279.6967 + 0.000303 * t2 + b;
    let l2 = 2.0 * l1.to_radians();

    let a = 1336.855231 * t;
    let b = 360.0 * (a - a.floor());

    let d1 = 270.4342 - 0.001133 * t2 + b;
    let d2 = 2.0 * d1.to_radians();

    let a = 99.99736056 * t;
    let b = 360.0 * (a - a.floor());

    let m1 = (358.4758 - 0.00015 * t2 + b).to_radians();
    //M1 = math.radians(M1)

    let a = 1325.552359 * t;
    let b = 360.0 * (a - a.floor());

    let m2 = (296.1046 + 0.009192 * t2 + b).to_radians();
    // M2 = math.radians(M2)

    let a = 5.372616667 * t;
    let b = 360.0 * (a - a.floor());

    let n1 = (259.1833 + 0.002078 * t2 - b).to_radians();
    //	N1 = math.radians(N1)

    let n2 = 2.0 * n1;

    let ddo = (9.21 + 0.00091 * t) * n1.cos();
    let ddo = ddo + (0.5522 - 0.00029 * t) * l2.cos() - 0.0904 * n2.cos();
    let ddo = ddo + 0.0884 * d2.cos() + 0.0216 * (l2 + m1).cos();
    let ddo = ddo + 0.0183 * (d2 - n1).cos() + 0.0113 * (d2 + m2).cos();
    let ddo = ddo - 0.0093 * (l2 - m1).cos() - 0.0066 * (l2 - n1).cos();

    return ddo / 3600.0;
}

/// Obliquity of the Ecliptic for a Greenwich Date
///
/// Original macro name: Obliq
pub fn obliq(greenwich_day: f64, greenwich_month: u32, greenwich_year: u32) -> f64 {
    let a = cd_jd(greenwich_day, greenwich_month, greenwich_year);
    let b = a - 2415020.0;
    let c = (b / 36525.0) - 1.0;
    let d = c * (46.815 + c * (0.0006 - (c * 0.00181)));
    let e = d / 3600.0;

    return 23.43929167 - e + nutat_obl(greenwich_day, greenwich_month, greenwich_year);
}

/// Convert Local Sidereal Time to Greenwich Sidereal Time
///
/// Original macro name: LSTGST
pub fn lst_gst(local_hours: f64, local_minutes: f64, local_seconds: f64, longitude: f64) -> f64 {
    let a = hms_dh(local_hours, local_minutes, local_seconds);
    let b = longitude / 15.0;
    let c = a - b;
    return c - (24.0 * (c / 24.0).floor());
}

/// Convert Greenwich Sidereal Time to Universal Time
///
/// Original macro name: GSTUT
pub fn gst_ut(
    greenwich_sidereal_hours: f64,
    greenwich_sidereal_minutes: f64,
    greenwich_sidereal_seconds: f64,
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) -> f64 {
    let a = cd_jd(greenwich_day, greenwich_month, greenwich_year);
    let b = a - 2451545.0;
    let c = b / 36525.0;
    let d = 6.697374558 + (2400.051336 * c) + (0.000025862 * c * c);
    let e = d - (24.0 * (d / 24.0).floor());
    let f = hms_dh(
        greenwich_sidereal_hours,
        greenwich_sidereal_minutes,
        greenwich_sidereal_seconds,
    );
    let g = f - e;
    let h = g - (24.0 * (g / 24.0).floor());
    return h * 0.9972695663;
}

/// Status of conversion of Greenwich Sidereal Time to Universal Time.
///
/// Original macro name: eGSTUT
pub fn e_gst_ut(gsh: f64, gsm: f64, gss: f64, gd: f64, gm: u32, gy: u32) -> String {
    let a = cd_jd(gd, gm, gy);
    let b = a - 2451545.0;
    let c = b / 36525.0;
    let d = 6.697374558 + (2400.051336 * c) + (0.000025862 * c * c);
    let e = d - (24.0 * (d / 24.0).floor());
    let f = hms_dh(gsh, gsm, gss);
    let g = f - e;
    let h = g - (24.0 * (g / 24.0).floor());

    if (h * 0.9972695663) < (4.0 / 60.0) {
        return "Warning".to_string();
    } else {
        return "OK".to_string();
    };
}

/// Calculate Sun's ecliptic longitude
///
/// Original macro name: SunLong
pub fn sun_long(lch: f64, lcm: f64, lcs: f64, ds: i32, zc: i32, ld: f64, lm: u32, ly: u32) -> f64 {
    let aa = lct_gday(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let bb = lct_gmonth(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let cc = lct_gyear(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let ut = lct_ut(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let dj = cd_jd(aa, bb, cc) - 2415020.0;
    let t = (dj / 36525.0) + (ut / 876600.0);
    let t2 = t * t;
    let a = 100.0021359 * t;
    let b = 360.0 * (a - (a).floor());

    let l = 279.69668 + 0.0003025 * t2 + b;
    let a = 99.99736042 * t;
    let b = 360.0 * (a - a.floor());

    let m1 = 358.47583 - (0.00015 + 0.0000033 * t) * t2 + b;
    let ec = 0.01675104 - 0.0000418 * t - 0.000000126 * t2;

    let am = m1.to_radians();
    let at = true_anomaly(am, ec);
    let _ae = eccentric_anomaly(am, ec);

    let a = 62.55209472 * t;
    let b = 360.0 * (a - (a).floor());

    let a1 = (153.23 + b).to_radians();
    let a = 125.1041894 * t;
    let b = 360.0 * (a - (a).floor());

    let b1 = (216.57 + b).to_radians();
    let a = 91.56766028 * t;
    let b = 360.0 * (a - (a).floor());

    let c1 = (312.69 + b).to_radians();
    let a = 1236.853095 * t;
    let b = 360.0 * (a - (a).floor());

    let d1 = (350.74 - 0.00144 * t2 + b).to_radians();
    let e1 = (231.19 + 20.2 * t).to_radians();
    let a = 183.1353208 * t;
    let b = 360.0 * (a - (a).floor());
    let h1 = (353.4 + b).to_radians();

    let d2 = 0.00134 * a1.cos() + 0.00154 * b1.cos() + 0.002 * c1.cos();
    let d2 = d2 + 0.00179 * d1.sin() + 0.00178 * e1.sin();
    let d3 = 0.00000543 * a1.sin() + 0.00001575 * b1.sin();
    let d3 = d3 + 0.00001627 * c1.sin() + 0.00003076 * d1.cos();
    let _d3 = d3 + 0.00000927 * h1.sin();

    let sr = at + (l - m1 + d2).to_radians();
    let tp = 6.283185308;

    let sr = sr - tp * (sr / tp).floor();
    return degrees(sr);
}

/// Solve Kepler's equation, and return value of the true anomaly in radians
///
/// Original macro name: TrueAnomaly
pub fn true_anomaly(am: f64, ec: f64) -> f64 {
    let tp = 6.283185308;
    let m = am - tp * (am / tp).floor();
    let mut ae = m;

    while 1 == 1 {
        let d = ae - (ec * (ae).sin()) - m;
        if d.abs() < 0.000001 {
            break;
        }
        let d = d / (1.0 - (ec * (ae).cos()));
        ae = ae - d;
    }

    let a = ((1.0 + ec) / (1.0 - ec)).sqrt() * (ae / 2.0).tan();
    let at = 2.0 * a.atan();

    return at;
}

/// Solve Kepler's equation, and return value of the eccentric anomaly in radians
///
/// Original macro name: EccentricAnomaly
pub fn eccentric_anomaly(am: f64, ec: f64) -> f64 {
    let tp = 6.283185308;
    let m = am - tp * (am / tp).floor();
    let mut ae = m;

    while 1 == 1 {
        let d = ae - (ec * (ae).sin()) - m;

        if d.abs() < 0.000001 {
            break;
        }

        let d = d / (1.0 - (ec * (ae).cos()));
        ae = ae - d;
    }

    return ae;
}

/// Calculate effects of refraction
///
/// Original macro name: Refract
pub fn refract(y2: f64, sw: String, pr: f64, tr: f64) -> f64 {
    let y = y2.to_radians();

    let d = if &sw[..1].to_string().to_lowercase() == "t" {
        -1.0
    } else {
        1.0
    };

    if d == -1.0 {
        let y3 = y;
        let y1 = y;
        let mut r1 = 0.0;

        while 1 == 1 {
            let y = y1 + r1;
            let _q = y;
            let rf = refract_l3035(pr, tr, y, d);
            if y < -0.087 {
                return 0.0;
            }
            let r2 = rf;

            if (r2 == 0.0) || ((r2 - r1).abs() < 0.000001) {
                let q = y3;
                return degrees(q + rf);
            }

            r1 = r2;
        }
    }

    let rf = refract_l3035(pr, tr, y, d);

    if y < -0.087 {
        return 0.0;
    }

    let q = y;

    return degrees(q + rf);
}

/// Helper function for refract
pub fn refract_l3035(pr: f64, tr: f64, y: f64, d: f64) -> f64 {
    if y < 0.2617994 {
        if y < -0.087 {
            return 0.0;
        }

        let yd = degrees(y);
        let a = ((0.00002 * yd + 0.0196) * yd + 0.1594) * pr;
        let b = (273.0 + tr) * ((0.0845 * yd + 0.505) * yd + 1.0);

        return (-(a / b) * d).to_radians();
    }

    return -d * 0.00007888888 * pr / ((273.0 + tr) * (y).tan());
}

/// Calculate corrected hour angle in decimal hours
///
/// Original macro name: ParallaxHA
pub fn parallax_ha(
    hh: f64,
    hm: f64,
    hs: f64,
    dd: f64,
    dm: f64,
    ds: f64,
    sw: String,
    gp: f64,
    ht: f64,
    hp: f64,
) -> f64 {
    let a = gp.to_radians();
    let c1 = a.cos();
    let s1 = a.sin();

    let u = (0.996647 * s1 / c1).atan();
    let c2 = u.cos();
    let s2 = u.sin();
    let b = ht / 6378160.0;

    let rs = (0.996647 * s2) + (b * s1);

    let rc = c2 + (b * c1);
    let tp = 6.283185308;

    let rp = 1.0 / hp.to_radians().sin();

    let x = (dh_dd(hms_dh(hh, hm, hs))).to_radians();
    let x1 = x;
    let y = (dms_dd(dd, dm, ds)).to_radians();
    let y1 = y;

    let d = if &sw[..1].to_string().to_lowercase() == "t" {
        1.0
    } else {
        -1.0
    };

    if d == 1.0 {
        let (p, _q) = parallax_ha_l2870(x, y, rc, rp, rs, tp);
        return dd_dh(degrees(p));
    }

    let mut p1 = 0.0;
    let mut q1 = 0.0;
    let mut x_loop = x;
    let mut y_loop = y;
    while 1 == 1 {
        let (p, q) = parallax_ha_l2870(x_loop, y_loop, rc, rp, rs, tp);
        let p2 = p - x_loop;
        let q2 = q - y_loop;

        let aa = (p2 - p1).abs();
        let bb = (q2 - q1).abs();

        if (aa < 0.000001) && (bb < 0.000001) {
            let p = x1 - p2;
            let _q = y1 - q2;
            let _x_loop = x1;
            let _y_loop = y1;

            return dd_dh(degrees(p));
        }
        x_loop = x1 - p2;
        y_loop = y1 - q2;
        p1 = p2;
        q1 = q2;
    }

    return dd_dh(degrees(0.0));
}

/// Helper function for parallax_ha
pub fn parallax_ha_l2870(x: f64, y: f64, rc: f64, rp: f64, rs: f64, tp: f64) -> (f64, f64) {
    let cx = x.cos();
    let sy = y.sin();
    let cy = y.cos();

    let aa = (rc * x.sin()) / ((rp * cy) - (rc * cx));

    let dx = aa.atan();
    let p = x + dx;
    let cp = p.cos();

    let p = p - tp * (p / tp).floor();
    let q = (cp * (rp * sy - rs) / (rp * cy * cx - rc)).atan();

    return (p, q);
}

/// Calculate corrected declination in decimal degrees
///
/// Original macro name: ParallaxDec
/// HH,HM,HS,DD,DM,DS,SW,GP,HT,HP
pub fn parallax_dec(
    hh: f64,
    hm: f64,
    hs: f64,
    dd: f64,
    dm: f64,
    ds: f64,
    sw: String,
    gp: f64,
    ht: f64,
    hp: f64,
) -> f64 {
    let a = gp.to_radians();
    let c1 = a.cos();
    let s1 = a.sin();

    let u = (0.996647 * s1 / c1).atan();

    let c2 = u.cos();
    let s2 = u.sin();
    let b = ht / 6378160.0;
    let rs = (0.996647 * s2) + (b * s1);

    let rc = c2 + (b * c1);
    let tp = 6.283185308;

    let rp = 1.0 / hp.to_radians().sin();

    let x = (dh_dd(hms_dh(hh, hm, hs))).to_radians();
    let x1 = x;

    let y = (dms_dd(dd, dm, ds)).to_radians();
    let y1 = y;
    let d = if &sw[..1].to_string().to_lowercase() == "t" {
        1.0
    } else {
        -1.0
    };

    if d == 1.0 {
        let (_p, q) = parallax_dec_l2870(x, y, rc, rp, rs, tp);
        return degrees(q);
    }

    let mut p1 = 0.0;
    let mut q1 = 0.0;

    let mut x_loop = x;
    let mut y_loop = y;
    while 1 == 1 {
        let (p, q) = parallax_dec_l2870(x_loop, y_loop, rc, rp, rs, tp);
        let p2 = p - x_loop;
        let q2 = q - y_loop;
        let aa = (p2 - p1).abs();
        let _bb = (q2 - q1).abs();
        if (aa < 0.000001) && (b < 0.000001) {
            let _p = x1 - p2;
            let q = y1 - q2;
            let _x_loop = x1;
            let _y_loop = y1;
            return degrees(q);
        }
        x_loop = x1 - p2;
        y_loop = y1 - q2;
        p1 = p2;
        q1 = q2;
    }

    return degrees(0.0);
}

/// Helper function for parallax_dec
pub fn parallax_dec_l2870(x: f64, y: f64, rc: f64, rp: f64, rs: f64, tp: f64) -> (f64, f64) {
    let cx = x.cos();
    let sy = y.sin();
    let cy = y.cos();

    let aa = (rc * x.sin()) / ((rp * cy) - (rc * cx));
    let dx = aa.atan();
    let p = x + dx;
    let cp = p.cos();

    let p = p - tp * (p / tp).floor();
    let q = (cp * (rp * sy - rs) / (rp * cy * cx - rc)).atan();

    return (p, q);
}

/// Calculate Sun's angular diameter in decimal degrees
///
/// Original macro name: SunDia
pub fn sun_dia(lch: f64, lcm: f64, lcs: f64, ds: i32, zc: i32, ld: f64, lm: u32, ly: u32) -> f64 {
    let a = sun_dist(lch, lcm, lcs, ds, zc, ld, lm, ly);

    return 0.533128 / a;
}

/// Calculate Sun's distance from the Earth in astronomical units
///
/// Original macro name: SunDist
pub fn sun_dist(lch: f64, lcm: f64, lcs: f64, ds: i32, zc: i32, ld: f64, lm: u32, ly: u32) -> f64 {
    let aa = lct_gday(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let bb = lct_gmonth(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let cc = lct_gyear(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let ut = lct_ut(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let dj = cd_jd(aa, bb, cc) - 2415020.0;

    let t = (dj / 36525.0) + (ut / 876600.0);
    let t2 = t * t;

    let a = 100.0021359 * t;
    let b = 360.0 * (a - a.floor());
    let _l = 279.69668 + 0.0003025 * t2 + b;
    let a = 99.99736042 * t;
    let b = 360.0 * (a - (a).floor());
    let m1 = 358.47583 - (0.00015 + 0.0000033 * t) * t2 + b;
    let ec = 0.01675104 - 0.0000418 * t - 0.000000126 * t2;

    let am = m1.to_radians();
    let _at = true_anomaly(am, ec);
    let ae = eccentric_anomaly(am, ec);

    let a = 62.55209472 * t;
    let b = 360.0 * (a - a.floor());
    let a1 = (153.23 + b).to_radians();
    let a = 125.1041894 * t;
    let b = 360.0 * (a - a.floor());
    let b1 = (216.57 + b).to_radians();
    let a = 91.56766028 * t;
    let b = 360.0 * (a - a.floor());
    let c1 = (312.69 + b).to_radians();
    let a = 1236.853095 * t;
    let b = 360.0 * (a - a.floor());
    let d1 = (350.74 - 0.00144 * t2 + b).to_radians();
    let e1 = (231.19 + 20.2 * t).to_radians();
    let a = 183.1353208 * t;
    let b = 360.0 * (a - a.floor());
    let h1 = (353.4 + b).to_radians();

    let _d2 = (0.00134 * a1.cos() + 0.00154 * b1.cos() + 0.002 * c1.cos())
        + (0.00179 * d1.sin() + 0.00178 * e1.sin());
    let d3 = (0.00000543 * a1.sin() + 0.00001575 * b1.sin())
        + (0.00001627 * c1.sin() + 0.00003076 * d1.cos())
        + (0.00000927 * h1.sin());

    return 1.0000002 * (1.0 - ec * ae.cos()) + d3;
}

/// Calculate geocentric ecliptic longitude for the Moon
///
/// Original macro name: MoonLong
pub fn moon_long(lh: f64, lm: f64, ls: f64, ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> f64 {
    let ut = lct_ut(lh, lm, ls, ds, zc, dy, mn, yr);
    let gd = lct_gday(lh, lm, ls, ds, zc, dy, mn, yr);
    let gm = lct_gmonth(lh, lm, ls, ds, zc, dy, mn, yr);
    let gy = lct_gyear(lh, lm, ls, ds, zc, dy, mn, yr);
    let t = ((cd_jd(gd, gm, gy) - 2415020.0) / 36525.0) + (ut / 876600.0);
    let t2 = t * t;

    let m1 = 27.32158213;
    let m2 = 365.2596407;
    let m3 = 27.55455094;
    let m4 = 29.53058868;
    let m5 = 27.21222039;
    let m6 = 6798.363307;
    let q = cd_jd(gd, gm, gy) - 2415020.0 + (ut / 24.0);
    let m1 = q / m1;
    let m2 = q / m2;
    let m3 = q / m3;
    let m4 = q / m4;
    let m5 = q / m5;
    let m6 = q / m6;
    let m1 = 360.0 * (m1 - (m1).floor());
    let m2 = 360.0 * (m2 - (m2).floor());
    let m3 = 360.0 * (m3 - (m3).floor());
    let m4 = 360.0 * (m4 - (m4).floor());
    let m5 = 360.0 * (m5 - (m5).floor());
    let m6 = 360.0 * (m6 - (m6).floor());

    let ml = 270.434164 + m1 - (0.001133 - 0.0000019 * t) * t2;
    let ms = 358.475833 + m2 - (0.00015 + 0.0000033 * t) * t2;
    let md = 296.104608 + m3 + (0.009192 + 0.0000144 * t) * t2;
    let me1 = 350.737486 + m4 - (0.001436 - 0.0000019 * t) * t2;
    let mf = 11.250889 + m5 - (0.003211 + 0.0000003 * t) * t2;
    let na = 259.183275 - m6 + (0.002078 + 0.0000022 * t) * t2;
    let a = (51.2 + 20.2 * t).to_radians();
    let s1 = a.sin();
    let s2 = ((na).to_radians()).sin();
    let b = 346.56 + (132.87 - 0.0091731 * t) * t;
    let s3 = 0.003964 * ((b).to_radians()).sin();
    let c = (na + 275.05 - 2.3 * t).to_radians();
    let s4 = c.sin();
    let ml = ml + 0.000233 * s1 + s3 + 0.001964 * s2;
    let ms = ms - 0.001778 * s1;
    let md = md + 0.000817 * s1 + s3 + 0.002541 * s2;
    let mf = mf + s3 - 0.024691 * s2 - 0.004328 * s4;
    let me1 = me1 + 0.002011 * s1 + s3 + 0.001964 * s2;
    let e = 1.0 - (0.002495 + 0.00000752 * t) * t;
    let e2 = e * e;
    let ml = (ml).to_radians();
    let ms = ms.to_radians();
    let _na = na.to_radians();
    let me1 = me1.to_radians();
    let mf = mf.to_radians();
    let md = md.to_radians();

    let l = 6.28875 * (md).sin() + 1.274018 * (2.0 * me1 - md).sin();
    let l = l + 0.658309 * (2.0 * me1).sin() + 0.213616 * (2.0 * md).sin();
    let l = l - e * 0.185596 * (ms).sin() - 0.114336 * (2.0 * mf).sin();
    let l = l + 0.058793 * (2.0 * (me1 - md)).sin();
    let l = l + 0.057212 * e * (2.0 * me1 - ms - md).sin() + 0.05332 * (2.0 * me1 + md).sin();
    let l = l + 0.045874 * e * (2.0 * me1 - ms).sin() + 0.041024 * e * (md - ms).sin();
    let l = l - 0.034718 * (me1).sin() - e * 0.030465 * (ms + md).sin();
    let l = l + 0.015326 * (2.0 * (me1 - mf)).sin() - 0.012528 * (2.0 * mf + md).sin();
    let l = l - 0.01098 * (2.0 * mf - md).sin() + 0.010674 * (4.0 * me1 - md).sin();
    let l = l + 0.010034 * (3.0 * md).sin() + 0.008548 * (4.0 * me1 - 2.0 * md).sin();
    let l = l - e * 0.00791 * (ms - md + 2.0 * me1).sin() - e * 0.006783 * (2.0 * me1 + ms).sin();
    let l = l + 0.005162 * (md - me1).sin() + e * 0.005 * (ms + me1).sin();
    let l = l + 0.003862 * (4.0 * me1).sin() + e * 0.004049 * (md - ms + 2.0 * me1).sin();
    let l = l + 0.003996 * (2.0 * (md + me1)).sin() + 0.003665 * (2.0 * me1 - 3.0 * md).sin();
    let l = l + e * 0.002695 * (2.0 * md - ms).sin() + 0.002602 * (md - 2.0 * (mf + me1)).sin();
    let l = l + e * 0.002396 * (2.0 * (me1 - md) - ms).sin() - 0.002349 * (md + me1).sin();
    let l = l + e2 * 0.002249 * (2.0 * (me1 - ms)).sin() - e * 0.002125 * (2.0 * md + ms).sin();
    let l = l - e2 * 0.002079 * (2.0 * ms).sin() + e2 * 0.002059 * (2.0 * (me1 - ms) - md).sin();
    let l = l - 0.001773 * (md + 2.0 * (me1 - mf)).sin() - 0.001595 * (2.0 * (mf + me1)).sin();
    let l = l + e * 0.00122 * (4.0 * me1 - ms - md).sin() - 0.00111 * (2.0 * (md + mf)).sin();
    let l = l + 0.000892 * (md - 3.0 * me1).sin() - e * 0.000811 * (ms + md + 2.0 * me1).sin();
    let l = l + e * 0.000761 * (4.0 * me1 - ms - 2.0 * md).sin();
    let l = l + e2 * 0.000704 * (md - 2.0 * (ms + me1)).sin();
    let l = l + e * 0.000693 * (ms - 2.0 * (md - me1)).sin();
    let l = l + e * 0.000598 * (2.0 * (me1 - mf) - ms).sin();
    let l = l + 0.00055 * (md + 4.0 * me1).sin() + 0.000538 * (4.0 * md).sin();
    let l = l + e * 0.000521 * (4.0 * me1 - ms).sin() + 0.000486 * (2.0 * md - me1).sin();
    let l = l + e2 * 0.000717 * (md - 2.0 * ms).sin();
    let mm = unwind(ml + l.to_radians());

    return degrees(mm);
}

/// Calculate geocentric ecliptic latitude for the Moon
///
/// Original macro name: MoonLat
pub fn moon_lat(lh: f64, lm: f64, ls: f64, ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> f64 {
    let ut = lct_ut(lh, lm, ls, ds, zc, dy, mn, yr);
    let gd = lct_gday(lh, lm, ls, ds, zc, dy, mn, yr);
    let gm = lct_gmonth(lh, lm, ls, ds, zc, dy, mn, yr);
    let gy = lct_gyear(lh, lm, ls, ds, zc, dy, mn, yr);
    let t = ((cd_jd(gd, gm, gy) - 2415020.0) / 36525.0) + (ut / 876600.0);
    let t2 = t * t;

    let m1 = 27.32158213;
    let m2 = 365.2596407;
    let m3 = 27.55455094;
    let m4 = 29.53058868;
    let m5 = 27.21222039;
    let m6 = 6798.363307;
    let q = cd_jd(gd, gm, gy) - 2415020.0 + (ut / 24.0);
    let m1 = q / m1;
    let m2 = q / m2;
    let m3 = q / m3;
    let m4 = q / m4;
    let m5 = q / m5;
    let m6 = q / m6;
    let m1 = 360.0 * (m1 - (m1).floor());
    let m2 = 360.0 * (m2 - (m2).floor());
    let m3 = 360.0 * (m3 - (m3).floor());
    let m4 = 360.0 * (m4 - (m4).floor());
    let m5 = 360.0 * (m5 - (m5).floor());
    let m6 = 360.0 * (m6 - (m6).floor());

    let ml = 270.434164 + m1 - (0.001133 - 0.0000019 * t) * t2;
    let ms = 358.475833 + m2 - (0.00015 + 0.0000033 * t) * t2;
    let md = 296.104608 + m3 + (0.009192 + 0.0000144 * t) * t2;
    let me1 = 350.737486 + m4 - (0.001436 - 0.0000019 * t) * t2;
    let mf = 11.250889 + m5 - (0.003211 + 0.0000003 * t) * t2;
    let na = 259.183275 - m6 + (0.002078 + 0.0000022 * t) * t2;
    let a = (51.2 + 20.2 * t).to_radians();
    let s1 = (a).sin();
    let s2 = na.to_radians().sin();
    let b = 346.56 + (132.87 - 0.0091731 * t) * t;
    let s3 = 0.003964 * b.to_radians().sin();
    let c = (na + 275.05 - 2.3 * t).to_radians();
    let s4 = (c).sin();
    let ml = ml + 0.000233 * s1 + s3 + 0.001964 * s2;
    let ms = ms - 0.001778 * s1;
    let md = md + 0.000817 * s1 + s3 + 0.002541 * s2;
    let mf = mf + s3 - 0.024691 * s2 - 0.004328 * s4;
    let me1 = me1 + 0.002011 * s1 + s3 + 0.001964 * s2;
    let e = 1.0 - (0.002495 + 0.00000752 * t) * t;
    let e2 = e * e;
    let _ml = (ml).to_radians();
    let ms = (ms).to_radians();
    let na = (na).to_radians();
    let me1 = (me1).to_radians();
    let mf = (mf).to_radians();
    let md = (md).to_radians();

    let g = 5.128189 * (mf).sin() + 0.280606 * (md + mf).sin();
    let g = g + 0.277693 * (md - mf).sin() + 0.173238 * (2.0 * me1 - mf).sin();
    let g = g + 0.055413 * (2.0 * me1 + mf - md).sin() + 0.046272 * (2.0 * me1 - mf - md).sin();
    let g = g + 0.032573 * (2.0 * me1 + mf).sin() + 0.017198 * (2.0 * md + mf).sin();
    let g = g + 0.009267 * (2.0 * me1 + md - mf).sin() + 0.008823 * (2.0 * md - mf).sin();
    let g =
        g + e * 0.008247 * (2.0 * me1 - ms - mf).sin() + 0.004323 * (2.0 * (me1 - md) - mf).sin();
    let g = g + 0.0042 * (2.0 * me1 + mf + md).sin() + e * 0.003372 * (mf - ms - 2.0 * me1).sin();
    let g = g + e * 0.002472 * (2.0 * me1 + mf - ms - md).sin();
    let g = g + e * 0.002222 * (2.0 * me1 + mf - ms).sin();
    let g = g + e * 0.002072 * (2.0 * me1 - mf - ms - md).sin();
    let g = g + e * 0.001877 * (mf - ms + md).sin() + 0.001828 * (4.0 * me1 - mf - md).sin();
    let g = g - e * 0.001803 * (mf + ms).sin() - 0.00175 * (3.0 * mf).sin();
    let g = g + e * 0.00157 * (md - ms - mf).sin() - 0.001487 * (mf + me1).sin();
    let g = g - e * 0.001481 * (mf + ms + md).sin() + e * 0.001417 * (mf - ms - md).sin();
    let g = g + e * 0.00135 * (mf - ms).sin() + 0.00133 * (mf - me1).sin();
    let g = g + 0.001106 * (mf + 3.0 * md).sin() + 0.00102 * (4.0 * me1 - mf).sin();
    let g = g + 0.000833 * (mf + 4.0 * me1 - md).sin() + 0.000781 * (md - 3.0 * mf).sin();
    let g =
        g + 0.00067 * (mf + 4.0 * me1 - 2.0 * md).sin() + 0.000606 * (2.0 * me1 - 3.0 * mf).sin();
    let g = g + 0.000597 * (2.0 * (me1 + md) - mf).sin();
    let g = g
        + e * 0.000492 * (2.0 * me1 + md - ms - mf).sin()
        + 0.00045 * (2.0 * (md - me1) - mf).sin();
    let g = g + 0.000439 * (3.0 * md - mf).sin() + 0.000423 * (mf + 2.0 * (me1 + md)).sin();
    let g = g + 0.000422 * (2.0 * me1 - mf - 3.0 * md).sin()
        - e * 0.000367 * (ms + mf + 2.0 * me1 - md).sin();
    let g = g - e * 0.000353 * (ms + mf + 2.0 * me1).sin() + 0.000331 * (mf + 4.0 * me1).sin();
    let g = g + e * 0.000317 * (2.0 * me1 + mf - ms + md).sin();
    let g = g + e2 * 0.000306 * (2.0 * (me1 - ms) - mf).sin() - 0.000283 * (md + 3.0 * mf).sin();
    let w1 = 0.0004664 * (na).cos();
    let w2 = 0.0000754 * (c).cos();
    let bm = (g).to_radians() * (1.0 - w1 - w2);

    return degrees(bm);
}

/// Calculate horizontal parallax for the Moon
///
/// Original macro name: MoonHP
pub fn moon_hp(lh: f64, lm: f64, ls: f64, ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> f64 {
    let ut = lct_ut(lh, lm, ls, ds, zc, dy, mn, yr);
    let gd = lct_gday(lh, lm, ls, ds, zc, dy, mn, yr);
    let gm = lct_gmonth(lh, lm, ls, ds, zc, dy, mn, yr);
    let gy = lct_gyear(lh, lm, ls, ds, zc, dy, mn, yr);
    let t = ((cd_jd(gd, gm, gy) - 2415020.0) / 36525.0) + (ut / 876600.0);
    let t2 = t * t;

    let m1 = 27.32158213;
    let m2 = 365.2596407;
    let m3 = 27.55455094;
    let m4 = 29.53058868;
    let m5 = 27.21222039;
    let m6 = 6798.363307;
    let q = cd_jd(gd, gm, gy) - 2415020.0 + (ut / 24.0);
    let m1 = q / m1;
    let m2 = q / m2;
    let m3 = q / m3;
    let m4 = q / m4;
    let m5 = q / m5;
    let m6 = q / m6;
    let m1 = 360.0 * (m1 - (m1).floor());
    let m2 = 360.0 * (m2 - (m2).floor());
    let m3 = 360.0 * (m3 - (m3).floor());
    let m4 = 360.0 * (m4 - (m4).floor());
    let m5 = 360.0 * (m5 - (m5).floor());
    let m6 = 360.0 * (m6 - (m6).floor());

    let ml = 270.434164 + m1 - (0.001133 - 0.0000019 * t) * t2;
    let ms = 358.475833 + m2 - (0.00015 + 0.0000033 * t) * t2;
    let md = 296.104608 + m3 + (0.009192 + 0.0000144 * t) * t2;
    let me1 = 350.737486 + m4 - (0.001436 - 0.0000019 * t) * t2;
    let mf = 11.250889 + m5 - (0.003211 + 0.0000003 * t) * t2;
    let na = 259.183275 - m6 + (0.002078 + 0.0000022 * t) * t2;
    let a = (51.2 + 20.2 * t).to_radians();
    let s1 = a.sin();
    let s2 = na.to_radians().sin();
    let b = 346.56 + (132.87 - 0.0091731 * t) * t;
    let s3 = 0.003964 * b.to_radians().sin();
    let c = (na + 275.05 - 2.3 * t).to_radians();
    let s4 = c.sin();
    let ml = ml + 0.000233 * s1 + s3 + 0.001964 * s2;
    let ms = ms - 0.001778 * s1;
    let md = md + 0.000817 * s1 + s3 + 0.002541 * s2;
    let mf = mf + s3 - 0.024691 * s2 - 0.004328 * s4;
    let me1 = me1 + 0.002011 * s1 + s3 + 0.001964 * s2;
    let e = 1.0 - (0.002495 + 0.00000752 * t) * t;
    let e2 = e * e;
    let _ml = (ml).to_radians();
    let ms = (ms).to_radians();
    let _na = (na).to_radians();
    let me1 = (me1).to_radians();
    let mf = (mf).to_radians();
    let md = (md).to_radians();

    let pm = 0.950724 + 0.051818 * (md).cos() + 0.009531 * (2.0 * me1 - md).cos();
    let pm = pm + 0.007843 * (2.0 * me1).cos() + 0.002824 * (2.0 * md).cos();
    let pm = pm + 0.000857 * (2.0 * me1 + md).cos() + e * 0.000533 * (2.0 * me1 - ms).cos();
    let pm = pm + e * 0.000401 * (2.0 * me1 - md - ms).cos();
    let pm = pm + e * 0.00032 * (md - ms).cos() - 0.000271 * (me1).cos();
    let pm = pm - e * 0.000264 * (ms + md).cos() - 0.000198 * (2.0 * mf - md).cos();
    let pm = pm + 0.000173 * (3.0 * md).cos() + 0.000167 * (4.0 * me1 - md).cos();
    let pm = pm - e * 0.000111 * (ms).cos() + 0.000103 * (4.0 * me1 - 2.0 * md).cos();
    let pm = pm - 0.000084 * (2.0 * md - 2.0 * me1).cos() - e * 0.000083 * (2.0 * me1 + ms).cos();
    let pm = pm + 0.000079 * (2.0 * me1 + 2.0 * md).cos() + 0.000072 * (4.0 * me1).cos();
    let pm = pm + e * 0.000064 * (2.0 * me1 - ms + md).cos()
        - e * 0.000063 * (2.0 * me1 + ms - md).cos();
    let pm = pm + e * 0.000041 * (ms + me1).cos() + e * 0.000035 * (2.0 * md - ms).cos();
    let pm = pm - 0.000033 * (3.0 * md - 2.0 * me1).cos() - 0.00003 * (md + me1).cos();
    let pm = pm - 0.000029 * (2.0 * (mf - me1)).cos() - e * 0.000029 * (2.0 * md + ms).cos();
    let pm =
        pm + e2 * 0.000026 * (2.0 * (me1 - ms)).cos() - 0.000023 * (2.0 * (mf - me1) + md).cos();
    let pm = pm + e * 0.000019 * (4.0 * me1 - ms - md).cos();

    return pm;
}

/// Calculate distance from the Earth to the Moon (km)
///
/// Original macro name: MoonDist
pub fn moon_dist(lh: f64, lm: f64, ls: f64, ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> f64 {
    let hp = (moon_hp(lh, lm, ls, ds, zc, dy, mn, yr)).to_radians();
    let r = 6378.14 / hp.sin();

    return r;
}

/// Calculate the Moon's angular diameter (degrees)
///
/// Original macro name: MoonSize
pub fn moon_size(lh: f64, lm: f64, ls: f64, ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> f64 {
    let hp = (moon_hp(lh, lm, ls, ds, zc, dy, mn, yr)).to_radians();
    let r = 6378.14 / hp.sin();
    let th = 384401.0 * 0.5181 / r;

    return th;
}

/// Convert angle in radians to equivalent angle in degrees.
///
/// Original macro name: Unwind
pub fn unwind(w: f64) -> f64 {
    return w - 6.283185308 * (w / 6.283185308).floor();
}

/// Convert angle in degrees to equivalent angle in the range 0 to 360 degrees.
///
/// Original macro name: UnwindDeg
pub fn unwind_deg(w: f64) -> f64 {
    return w - 360.0 * (w / 360.0).floor();
}

/// Convert angle in radians to equivalent angle in degrees.
///
/// Original macro name: UnwindRad
#[allow(dead_code)]
pub fn unwind_rad(w: f64) -> f64 {
    return w - 6.283185308 * (w / 6.283185308).floor();
}

/// Mean ecliptic longitude of the Sun at the epoch
///
/// Original macro name: SunElong
pub fn sun_e_long(gd: f64, gm: u32, gy: u32) -> f64 {
    let t = (cd_jd(gd, gm, gy) - 2415020.0) / 36525.0;
    let t2 = t * t;
    let x = 279.6966778 + 36000.76892 * t + 0.0003025 * t2;

    return x - 360.0 * (x / 360.0).floor();
}

/// Longitude of the Sun at perigee
///
/// Original macro name: SunPeri
pub fn sun_peri(gd: f64, gm: u32, gy: u32) -> f64 {
    let t = (cd_jd(gd, gm, gy) - 2415020.0) / 36525.0;
    let t2 = t * t;
    let x = 281.2208444 + 1.719175 * t + 0.000452778 * t2;

    return x - 360.0 * (x / 360.0).floor();
}

/// Eccentricity of the Sun-Earth orbit
///
/// Original macro name: SunEcc
pub fn sun_ecc(gd: f64, gm: u32, gy: u32) -> f64 {
    let t = (cd_jd(gd, gm, gy) - 2415020.0) / 36525.0;
    let t2 = t * t;

    return 0.01675104 - 0.0000418 * t - 0.000000126 * t2;
}

/// Ecliptic - Declination (degrees)
///
/// Original macro name: ECDec
pub fn ec_dec(
    eld: f64,
    elm: f64,
    els: f64,
    bd: f64,
    bm: f64,
    bs: f64,
    gd: f64,
    gm: u32,
    gy: u32,
) -> f64 {
    let a = (dms_dd(eld, elm, els)).to_radians();
    let b = (dms_dd(bd, bm, bs)).to_radians();
    let c = (obliq(gd, gm, gy)).to_radians();
    let d = b.sin() * c.cos() + b.cos() * c.sin() * a.sin();
    return degrees(d.asin());
}

/// Ecliptic - Right Ascension (degrees)
///
/// Original macro name: ECRA
pub fn ec_ra(
    eld: f64,
    elm: f64,
    els: f64,
    bd: f64,
    bm: f64,
    bs: f64,
    gd: f64,
    gm: u32,
    gy: u32,
) -> f64 {
    let a = (dms_dd(eld, elm, els)).to_radians();
    let b = (dms_dd(bd, bm, bs)).to_radians();
    let c = (obliq(gd, gm, gy)).to_radians();
    let d = a.sin() * c.cos() - b.tan() * c.sin();
    let e = a.cos();
    let f = degrees(d.atan2(e));

    return f - 360.0 * (f / 360.0).floor();
}

/// Calculate Sun's true anomaly, i.e., how much its orbit deviates from a true circle to an ellipse.
///
/// Original macro name: SunTrueAnomaly
pub fn sun_true_anomaly(
    lch: f64,
    lcm: f64,
    lcs: f64,
    ds: i32,
    zc: i32,
    ld: f64,
    lm: u32,
    ly: u32,
) -> f64 {
    let aa = lct_gday(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let bb = lct_gmonth(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let cc = lct_gyear(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let ut = lct_ut(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let dj = cd_jd(aa, bb, cc) - 2415020.0;

    let t = (dj / 36525.0) + (ut / 876600.0);
    let t2 = t * t;

    let a = 100.0021359 * t;
    let b = 360.0 * (a - a.floor());
    let _l = 279.69668 + 0.0003025 * t2 + b;

    let a = 99.99736042 * t;
    let b = 360.0 * (a - a.floor());

    let m1 = 358.47583 - (0.00015 + 0.0000033 * t) * t2 + b;
    let ec = 0.01675104 - 0.0000418 * t - 0.000000126 * t2;

    let am = m1.to_radians();

    return degrees(true_anomaly(am, ec));
}

/// Calculate the Sun's mean anomaly.
///
/// Original macro name: SunMeanAnomaly
pub fn sun_mean_anomaly(
    lch: f64,
    lcm: f64,
    lcs: f64,
    ds: i32,
    zc: i32,
    ld: f64,
    lm: u32,
    ly: u32,
) -> f64 {
    let aa = lct_gday(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let bb = lct_gmonth(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let cc = lct_gyear(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let ut = lct_ut(lch, lcm, lcs, ds, zc, ld, lm, ly);
    let dj = cd_jd(aa, bb, cc) - 2415020.0;
    let t = (dj / 36525.0) + (ut / 876600.0);
    let t2 = t * t;
    let a = 100.0021359 * t;
    let b = 360.0 * (a - a.floor());
    let m1 = 358.47583 - (0.00015 + 0.0000033 * t) * t2 + b;
    let am = unwind((m1).to_radians());

    return am;
}

/// Calculate local civil time of sunrise.
///
/// Original macro name: SunriseLCT
pub fn sunrise_lct(ld: f64, lm: u32, ly: u32, ds: i32, zc: i32, gl: f64, gp: f64) -> f64 {
    let di = 0.8333333;
    let gd = lct_gday(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gm = lct_gmonth(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gy = lct_gyear(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let sr = sun_long(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);

    let (_a, _x, _y, la, s) = sunrise_lct_l3710(gd, gm, gy, sr, di, gp);

    let xx: f64;
    if s != "OK" {
        xx = -99.0;
    } else {
        let x = lst_gst(la, 0.0, 0.0, gl);
        let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

        if e_gst_ut(x, 0.0, 0.0, gd, gm, gy) != "OK" {
            xx = -99.0;
        } else {
            let sr = sun_long(ut, 0.0, 0.0, 0, 0, gd, gm, gy);
            let (_a, _x, _y, la, s) = sunrise_lct_l3710(gd, gm, gy, sr, di, gp);

            if s != "OK" {
                xx = -99.0;
            } else {
                let x = lst_gst(la, 0.0, 0.0, gl);
                let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);
                xx = ut_lct(ut, 0.0, 0.0, ds, zc, gd, gm, gy);
            }
        }
    }

    return xx;
}

/// Helper function for sunrise_lct()
pub fn sunrise_lct_l3710(
    gd: f64,
    gm: u32,
    gy: u32,
    sr: f64,
    di: f64,
    gp: f64,
) -> (f64, f64, f64, f64, String) {
    let a = sr + nutat_long(gd, gm, gy) - 0.005694;
    let x = ec_ra(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let y = ec_dec(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let la = rise_set_local_sidereal_time_rise(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
    let s = e_rs(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);

    return (a, x, y, la, s.to_string());
}

/// Calculate local civil time of sunset.
///
/// Original macro name: SunsetLCT
pub fn sunset_lct(ld: f64, lm: u32, ly: u32, ds: i32, zc: i32, gl: f64, gp: f64) -> f64 {
    let di = 0.8333333;
    let gd = lct_gday(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gm = lct_gmonth(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gy = lct_gyear(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let sr = sun_long(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);

    let (_a, _x, _y, la, s) = sunset_lct_l3710(gd, gm, gy, sr, di, gp);

    let xx: f64;
    if s != "OK" {
        xx = -99.0;
    } else {
        let x = lst_gst(la, 0.0, 0.0, gl);
        let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

        if e_gst_ut(x, 0.0, 0.0, gd, gm, gy) != "OK" {
            xx = -99.0;
        } else {
            let sr = sun_long(ut, 0.0, 0.0, 0, 0, gd, gm, gy);
            let (_a, _x, _y, la, s) = sunset_lct_l3710(gd, gm, gy, sr, di, gp);

            if s != "OK" {
                xx = -99.0;
            } else {
                let x = lst_gst(la, 0.0, 0.0, gl);
                let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);
                xx = ut_lct(ut, 0.0, 0.0, ds, zc, gd, gm, gy);
            }
        }
    }
    return xx;
}

/// Helper function for sunset_lct().
pub fn sunset_lct_l3710(
    gd: f64,
    gm: u32,
    gy: u32,
    sr: f64,
    di: f64,
    gp: f64,
) -> (f64, f64, f64, f64, String) {
    let a = sr + nutat_long(gd, gm, gy) - 0.005694;
    let x = ec_ra(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let y = ec_dec(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let la = rise_set_local_sidereal_time_set(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
    let s = e_rs(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);

    return (a, x, y, la, s);
}

/// Local sidereal time of rise, in hours.
///
/// Original macro name: RSLSTR
pub fn rise_set_local_sidereal_time_rise(
    rah: f64,
    ram: f64,
    ras: f64,
    dd: f64,
    dm: f64,
    ds: f64,
    vd: f64,
    g: f64,
) -> f64 {
    let a = hms_dh(rah, ram, ras);
    let b = (dh_dd(a)).to_radians();
    let c = (dms_dd(dd, dm, ds)).to_radians();
    let d = (vd).to_radians();
    let e = (g).to_radians();
    let f = -((d).sin() + (e).sin() * (c).sin()) / ((e).cos() * (c).cos());
    let h = if f.abs() < 1.0 { f.acos() } else { 0.0 };
    let i = dd_dh(degrees(b - h));

    return i - 24.0 * (i / 24.0).floor();
}

/// Azimuth of rising, in degrees.
///
/// Original macro name: RSAZR
pub fn rise_set_azimuth_rise(
    rah: f64,
    ram: f64,
    ras: f64,
    dd: f64,
    dm: f64,
    ds: f64,
    vd: f64,
    g: f64,
) -> f64 {
    let a = hms_dh(rah, ram, ras);
    let _b = (dh_dd(a)).to_radians();;
    let c = (dms_dd(dd, dm, ds)).to_radians();
    let d = vd.to_radians();
    let e = g.to_radians();
    let f = (c.sin() + d.sin() * e.sin()) / (d.cos() * e.cos());
    let h = if e_rs(rah, ram, ras, dd, dm, ds, vd, g) == "OK" {
        f.acos()
    } else {
        0.0
    };
    let i = degrees(h);

    return i - 360.0 * (i / 360.0).floor();
}

/// Local sidereal time of setting, in hours.
///
/// Original macro name: RSLSTS
pub fn rise_set_local_sidereal_time_set(
    rah: f64,
    ram: f64,
    ras: f64,
    dd: f64,
    dm: f64,
    ds: f64,
    vd: f64,
    g: f64,
) -> f64 {
    let a = hms_dh(rah, ram, ras);
    let b = (dh_dd(a)).to_radians();
    let c = (dms_dd(dd, dm, ds)).to_radians();
    let d = vd.to_radians();
    let e = g.to_radians();
    let f = -(d.sin() + e.sin() * c.sin()) / (e.cos() * c.cos());
    let h = if f.abs() < 1.0 { f.acos() } else { 0.0 };
    let i = dd_dh(degrees(b + h));

    return i - 24.0 * (i / 24.0).floor();
}

/// Azimuth of setting, in degrees.
///
/// Original macro name: RSAZS
pub fn rise_set_azimuth_set(
    rah: f64,
    ram: f64,
    ras: f64,
    dd: f64,
    dm: f64,
    ds: f64,
    vd: f64,
    g: f64,
) -> f64 {
    let a = hms_dh(rah, ram, ras);
    let _b = (dh_dd(a)).to_radians();
    let c = (dms_dd(dd, dm, ds)).to_radians();
    let d = vd.to_radians();
    let e = g.to_radians();
    let f = (c.sin() + d.sin() * e.sin()) / (d.cos() * e.cos());
    let h = if e_rs(rah, ram, ras, dd, dm, ds, vd, g) == "OK" {
        f.acos()
    } else {
        0.0
    };
    let i = 360.0 - degrees(h);

    return i - 360.0 * (i / 360.0).floor();
}

/// Rise/Set status
///
/// Possible values: "OK", "** never rises", "** circumpolar"
///
/// Original macro name: eRS
pub fn e_rs(rah: f64, ram: f64, ras: f64, dd: f64, dm: f64, ds: f64, vd: f64, g: f64) -> String {
    let a = hms_dh(rah, ram, ras);
    let _b = dh_dd(a).to_radians();
    let c = (dms_dd(dd, dm, ds)).to_radians();
    let d = vd.to_radians();
    let e = g.to_radians();
    let f = -(d.sin() + e.sin() * c.sin()) / (e.cos() * c.cos());

    let mut return_value = "OK";
    if f >= 1.0 {
        return_value = "** never rises";
    }
    if f <= -1.0 {
        return_value = "** circumpolar"
    }

    return return_value.to_string();
}

/// Sunrise/Sunset calculation status.
///
/// Original macro name: eSunRS
pub fn e_sun_rs(ld: f64, lm: u32, ly: u32, ds: i32, zc: i32, gl: f64, gp: f64) -> String {
    // S = ""
    let di = 0.8333333;
    let gd = lct_gday(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gm = lct_gmonth(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gy = lct_gyear(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let sr = sun_long(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);

    let (_a, _x, _y, la, s) = e_sun_rs_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return s;
    } else {
        let x = lst_gst(la, 0.0, 0.0, gl);
        let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);
        let sr = sun_long(ut, 0.0, 0.0, 0, 0, gd, gm, gy);
        let (_a, _x, _y, la, s) = e_sun_rs_l3710(gd, gm, gy, sr, di, gp);
        if s != "OK" {
            return s;
        } else {
            let x = lst_gst(la, 0.0, 0.0, gl);
            let _ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

            if e_gst_ut(x, 0.0, 0.0, gd, gm, gy) != "OK" {
                let s = s + " GST to UT conversion warning";
                return s;
            }
            return s;
        }
    }
}

/// Helper function for e_sun_rs()
pub fn e_sun_rs_l3710(
    gd: f64,
    gm: u32,
    gy: u32,
    sr: f64,
    di: f64,
    gp: f64,
) -> (f64, f64, f64, f64, String) {
    let a = sr + nutat_long(gd, gm, gy) - 0.005694;
    let x = ec_ra(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let y = ec_dec(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let la = rise_set_local_sidereal_time_rise(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
    let s = e_rs(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);

    return (a, x, y, la, s);
}

/// Calculate azimuth of sunrise.
///
/// Original macro name: SunriseAz
pub fn sunrise_az(ld: f64, lm: u32, ly: u32, ds: i32, zc: i32, gl: f64, gp: f64) -> f64 {
    let di = 0.8333333;
    let gd = lct_gday(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gm = lct_gmonth(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gy = lct_gyear(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let sr = sun_long(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);

    let (_a, _x, _y, la, s) = sunrise_az_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return -99.0;
    }

    let x = lst_gst(la, 0.0, 0.0, gl);
    let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

    if e_gst_ut(x, 0.0, 0.0, gd, gm, gy) != "OK" {
        return -99.0;
    }

    let sr = sun_long(ut, 0.0, 0.0, 0, 0, gd, gm, gy);
    let (_a, x, y, _la, s) = sunrise_az_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return -99.0;
    }

    return rise_set_azimuth_rise(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
}

/// Helper function for sunrise_az()
pub fn sunrise_az_l3710(
    gd: f64,
    gm: u32,
    gy: u32,
    sr: f64,
    di: f64,
    gp: f64,
) -> (f64, f64, f64, f64, String) {
    let a = sr + nutat_long(gd, gm, gy) - 0.005694;
    let x = ec_ra(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let y = ec_dec(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let la = rise_set_local_sidereal_time_rise(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
    let s = e_rs(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);

    return (a, x, y, la, s);
}

/// Calculate azimuth of sunset.
///
/// Original macro name: SunsetAz
pub fn sunset_az(ld: f64, lm: u32, ly: u32, ds: i32, zc: i32, gl: f64, gp: f64) -> f64 {
    let di = 0.8333333;
    let gd = lct_gday(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gm = lct_gmonth(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gy = lct_gyear(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let sr = sun_long(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);

    let (_a, _x, _y, la, s) = sunset_az_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return -99.0;
    }

    let x = lst_gst(la, 0.0, 0.0, gl);
    let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

    if e_gst_ut(x, 0.0, 0.0, gd, gm, gy) != "OK" {
        return -99.0;
    }

    let sr = sun_long(ut, 0.0, 0.0, 0, 0, gd, gm, gy);

    let (_a, x, y, _la, s) = sunset_az_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return -99.0;
    }
    return rise_set_azimuth_set(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
}

/// Helper function for sunset_az()
pub fn sunset_az_l3710(
    gd: f64,
    gm: u32,
    gy: u32,
    sr: f64,
    di: f64,
    gp: f64,
) -> (f64, f64, f64, f64, String) {
    let a = sr + nutat_long(gd, gm, gy) - 0.005694;
    let x = ec_ra(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let y = ec_dec(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let la = rise_set_local_sidereal_time_set(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
    let s = e_rs(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);

    return (a, x, y, la, s);
}

/// Calculate morning twilight start, in local time.
///
/// Twilight type (TT) can be one of "C" (civil), "N" (nautical), or "A" (astronomical)
///
/// Original macro name: TwilightAMLCT
pub fn twilight_am_lct(
    ld: f64,
    lm: u32,
    ly: u32,
    ds: i32,
    zc: i32,
    gl: f64,
    gp: f64,
    tt: &pa_types::TwilightType,
) -> f64 {
    let di = match tt {
        pa_types::TwilightType::Astronomical => 18.0,
        pa_types::TwilightType::Civil => 6.0,
        pa_types::TwilightType::Nautical => 12.0,
    };

    let gd = lct_gday(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gm = lct_gmonth(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gy = lct_gyear(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let sr = sun_long(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);

    let (_a, _x, _y, la, s) = twilight_am_lct_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return -99.0;
    }

    let x = lst_gst(la, 0.0, 0.0, gl);
    let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

    if e_gst_ut(x, 0.0, 0.0, gd, gm, gy) != "OK" {
        return -99.0;
    }

    let sr = sun_long(ut, 0.0, 0.0, 0, 0, gd, gm, gy);

    let (_a, _x, _y, la, s) = twilight_am_lct_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return -99.0;
    }

    let x = lst_gst(la, 0.0, 0.0, gl);
    let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

    let xx = ut_lct(ut, 0.0, 0.0, ds, zc, gd, gm, gy);

    return xx;
}

/// Helper function for twilight_am_lct()
pub fn twilight_am_lct_l3710(
    gd: f64,
    gm: u32,
    gy: u32,
    sr: f64,
    di: f64,
    gp: f64,
) -> (f64, f64, f64, f64, String) {
    let a = sr + nutat_long(gd, gm, gy) - 0.005694;
    let x = ec_ra(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let y = ec_dec(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let la = rise_set_local_sidereal_time_rise(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
    let s = e_rs(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);

    return (a, x, y, la, s);
}

/// Calculate evening twilight end, in local time.
///
/// Twilight type can be one of "C" (civil), "N" (nautical), or "A" (astronomical)
///
/// Original macro name: TwilightPMLCT
pub fn twilight_pm_lct(
    ld: f64,
    lm: u32,
    ly: u32,
    ds: i32,
    zc: i32,
    gl: f64,
    gp: f64,
    tt: &pa_types::TwilightType,
) -> f64 {
    let di = match tt {
        pa_types::TwilightType::Astronomical => 18.0,
        pa_types::TwilightType::Civil => 6.0,
        pa_types::TwilightType::Nautical => 12.0,
    };

    let gd = lct_gday(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gm = lct_gmonth(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gy = lct_gyear(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let sr = sun_long(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);

    let (_a, _x, _y, la, s) = twilight_pm_lct_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return 0.0;
    }

    let x = lst_gst(la, 0.0, 0.0, gl);
    let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

    if e_gst_ut(x, 0.0, 0.0, gd, gm, gy) != "OK" {
        return 0.0;
    }

    let sr = sun_long(ut, 0.0, 0.0, 0, 0, gd, gm, gy);

    let (_a, _x, _y, la, s) = twilight_pm_lct_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return 0.0;
    }

    let x = lst_gst(la, 0.0, 0.0, gl);
    let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

    return ut_lct(ut, 0.0, 0.0, ds, zc, gd, gm, gy);
}

/// Helper function for twilight_pm_lct()
pub fn twilight_pm_lct_l3710(
    gd: f64,
    gm: u32,
    gy: u32,
    sr: f64,
    di: f64,
    gp: f64,
) -> (f64, f64, f64, f64, String) {
    let a = sr + nutat_long(gd, gm, gy) - 0.005694;
    let x = ec_ra(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let y = ec_dec(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let la = rise_set_local_sidereal_time_set(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
    let s = e_rs(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);

    return (a, x, y, la, s);
}

/// Twilight calculation status.
///
/// Twilight type can be one of "C" (civil), "N" (nautical), or "A" (astronomical)
///
/// Original macro name: eTwilight
///
/// ## Returns
/// One of: "OK", "** lasts all night", or "** Sun too far below horizon"
pub fn e_twilight(
    ld: f64,
    lm: u32,
    ly: u32,
    ds: i32,
    zc: i32,
    gl: f64,
    gp: f64,
    tt: &pa_types::TwilightType,
) -> String {
    // S = ""

    let di = match tt {
        pa_types::TwilightType::Astronomical => 18.0,
        pa_types::TwilightType::Civil => 6.0,
        pa_types::TwilightType::Nautical => 12.0,
    };

    let gd = lct_gday(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gm = lct_gmonth(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let gy = lct_gyear(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);
    let sr = sun_long(12.0, 0.0, 0.0, ds, zc, ld, lm, ly);

    let (_a, _x, _y, la, s) = e_twilight_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return s;
    }

    let x = lst_gst(la, 0.0, 0.0, gl);
    let ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);
    let sr = sun_long(ut, 0.0, 0.0, 0, 0, gd, gm, gy);

    let (_a, _x, _y, la, s) = e_twilight_l3710(gd, gm, gy, sr, di, gp);

    if s != "OK" {
        return s;
    }

    let x = lst_gst(la, 0.0, 0.0, gl);
    let _ut = gst_ut(x, 0.0, 0.0, gd, gm, gy);

    if e_gst_ut(x, 0.0, 0.0, gd, gm, gy) != "OK" {
        let s = s + " GST to UT conversion warning";

        return s;
    }

    return s;
}

/// Helper function for e_twilight()
pub fn e_twilight_l3710(
    gd: f64,
    gm: u32,
    gy: u32,
    sr: f64,
    di: f64,
    gp: f64,
) -> (f64, f64, f64, f64, String) {
    let a = sr + nutat_long(gd, gm, gy) - 0.005694;
    let x = ec_ra(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let y = ec_dec(a, 0.0, 0.0, 0.0, 0.0, 0.0, gd, gm, gy);
    let la = rise_set_local_sidereal_time_rise(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);
    let mut s = e_rs(dd_dh(x), 0.0, 0.0, y, 0.0, 0.0, di, gp);

    if s.len() > 2 {
        if &s[0..3].to_string() == "** c" {
            s = "** lasts all night".to_string();
        } else {
            if &s[0..3].to_string() == "** n" {
                s = "** Sun too far below horizon".to_string();
            }
        }
    }

    return (a, x, y, la, s);
}

/// Calculate the angle between two celestial objects
///
/// Original macro name: Angle
pub fn angle(
    xx1: f64,
    xm1: f64,
    xs1: f64,
    dd1: f64,
    dm1: f64,
    ds1: f64,
    xx2: f64,
    xm2: f64,
    xs2: f64,
    dd2: f64,
    dm2: f64,
    ds2: f64,
    s: pa_types::AngleMeasure,
) -> f64 {
    let s_value = match s {
        pa_types::AngleMeasure::Degrees => "D",
        pa_types::AngleMeasure::Hours => "H",
    };

    let a = if s_value == "H" {
        dh_dd(hms_dh(xx1, xm1, xs1))
    } else {
        dms_dd(xx1, xm1, xs1)
    };
    let b = a.to_radians();
    let c = dms_dd(dd1, dm1, ds1);
    let d = c.to_radians();
    let e = if s_value == "H" {
        dh_dd(hms_dh(xx2, xm2, xs2))
    } else {
        dms_dd(xx2, xm2, xs2)
    };
    let f = e.to_radians();
    let g = dms_dd(dd2, dm2, ds2);
    let h = g.to_radians();
    let i = (d.sin() * h.sin() + d.cos() * h.cos() * (b - f).cos()).acos();

    return degrees(i);
}

#[derive(Clone)]
pub struct PlDataStruct {
    pub value1: f64,
    pub value2: f64,
    pub value3: f64,
    pub value4: f64,
    pub value5: f64,
    pub value6: f64,
    pub value7: f64,
    pub value8: f64,
    pub value9: f64,
}

/// Calculate several planetary properties.
///
/// Original macro names: PlanetLong, PlanetLat, PlanetDist, PlanetHLong1, PlanetHLong2, PlanetHLat, PlanetRVect
///
/// ## Arguments
/// * `lh` -- Local civil time, hour part.
/// * `lm` -- Local civil time, minutes part.
/// * `ls` -- Local civil time, seconds part.
/// * `ds` -- Daylight Savings offset.
/// * `zc` -- Time zone correction, in hours.
/// * `dy` -- Local date, day part.
/// * `mn` -- Local date, month part.
/// * `yr` -- Local date, year part.
/// * `s` -- Planet name.
///
/// ## Returns
/// * `planet_longitude` -- Ecliptic longitude, in degrees.
/// * `planet_latitude` -- Ecliptic latitude, in degrees.
/// * `planet_distance_au` -- Earth-planet distance, in AU.
/// * `planet_h_long1` -- Heliocentric orbital longitude, in degrees.
/// * `planet_h_long2` -- NOT USED
/// * `planet_h_lat` -- NOT USED
/// * `planet_r_vect` -- Sun-planet distance (length of radius vector), in AU.
pub fn planet_coordinates(
    lh: f64,
    lm: f64,
    ls: f64,
    ds: i32,
    zc: i32,
    dy: f64,
    mn: u32,
    yr: u32,
    s: String,
) -> (f64, f64, f64, f64, f64, f64, f64) {
    let a11 = 178.179078;
    let a12 = 415.2057519;
    let a13 = 0.0003011;
    let a14 = 0.0;
    let a21 = 75.899697;
    let a22 = 1.5554889;
    let a23 = 0.0002947;
    let a24 = 0.0;
    let a31 = 0.20561421;
    let a32 = 0.00002046;
    let a33 = -0.00000003;
    let a34 = 0.0;
    let a41 = 7.002881;
    let a42 = 0.0018608;
    let a43 = -0.0000183;
    let a44 = 0.0;
    let a51 = 47.145944;
    let a52 = 1.1852083;
    let a53 = 0.0001739;
    let a54 = 0.0;
    let a61 = 0.3870986;
    let a62 = 6.74;
    let a63 = -0.42;

    let b11 = 342.767053;
    let b12 = 162.5533664;
    let b13 = 0.0003097;
    let b14 = 0.0;
    let b21 = 130.163833;
    let b22 = 1.4080361;
    let b23 = -0.0009764;
    let b24 = 0.0;
    let b31 = 0.00682069;
    let b32 = -0.00004774;
    let b33 = 0.000000091;
    let b34 = 0.0;
    let b41 = 3.393631;
    let b42 = 0.0010058;
    let b43 = -0.000001;
    let b44 = 0.0;
    let b51 = 75.779647;
    let b52 = 0.89985;
    let b53 = 0.00041;
    let b54 = 0.0;
    let b61 = 0.7233316;
    let b62 = 16.92;
    let b63 = -4.4;

    let c11 = 293.737334;
    let c12 = 53.17137642;
    let c13 = 0.0003107;
    let c14 = 0.0;
    let c21 = 334.218203;
    let c22 = 1.8407584;
    let c23 = 0.0001299;
    let c24 = -0.00000119;
    let c31 = 0.0933129;
    let c32 = 0.000092064;
    let c33 = -0.000000077;
    let c34 = 0.0;
    let c41 = 1.850333;
    let c42 = -0.000675;
    let c43 = 0.0000126;
    let c44 = 0.0;
    let c51 = 48.786442;
    let c52 = 0.7709917;
    let c53 = -0.0000014;
    let c54 = -0.00000533;
    let c61 = 1.5236883;
    let c62 = 9.36;
    let c63 = -1.52;

    let d11 = 238.049257;
    let d12 = 8.434172183;
    let d13 = 0.0003347;
    let d14 = -0.00000165;
    let d21 = 12.720972;
    let d22 = 1.6099617;
    let d23 = 0.00105627;
    let d24 = -0.00000343;
    let d31 = 0.04833475;
    let d32 = 0.00016418;
    let d33 = -0.0000004676;
    let d34 = -0.0000000017;
    let d41 = 1.308736;
    let d42 = -0.0056961;
    let d43 = 0.0000039;
    let d44 = 0.0;
    let d51 = 99.443414;
    let d52 = 1.01053;
    let d53 = 0.00035222;
    let d54 = -0.00000851;
    let d61 = 5.202561;
    let d62 = 196.74;
    let d63 = -9.4;

    let e11 = 266.564377;
    let e12 = 3.398638567;
    let e13 = 0.0003245;
    let e14 = -0.0000058;
    let e21 = 91.098214;
    let e22 = 1.9584158;
    let e23 = 0.00082636;
    let e24 = 0.00000461;
    let e31 = 0.05589232;
    let e32 = -0.0003455;
    let e33 = -0.000000728;
    let e34 = 0.00000000074;
    let e41 = 2.492519;
    let e42 = -0.0039189;
    let e43 = -0.00001549;
    let e44 = 0.00000004;
    let e51 = 112.790414;
    let e52 = 0.8731951;
    let e53 = -0.00015218;
    let e54 = -0.00000531;
    let e61 = 9.554747;
    let e62 = 165.6;
    let e63 = -8.88;

    let f11 = 244.19747;
    let f12 = 1.194065406;
    let f13 = 0.000316;
    let f14 = -0.0000006;
    let f21 = 171.548692;
    let f22 = 1.4844328;
    let f23 = 0.0002372;
    let f24 = -0.00000061;
    let f31 = 0.0463444;
    let f32a = -0.00002658;
    let f33 = 0.000000077;
    let f34 = 0.0;
    let f41 = 0.772464;
    let f42 = 0.0006253;
    let f43 = 0.0000395;
    let f44 = 0.0;
    let f51 = 73.477111;
    let f52 = 0.4986678;
    let f53 = 0.0013117;
    let f54 = 0.0;
    let f61 = 19.21814;
    let f62 = 65.8;
    let f63 = -7.19;

    let g11 = 84.457994;
    let g12 = 0.6107942056;
    let g13 = 0.0003205;
    let g14 = -0.0000006;
    let g21 = 46.727364;
    let g22 = 1.4245744;
    let g23 = 0.00039082;
    let g24 = -0.000000605;
    let g31 = 0.00899704;
    let g32 = 0.00000633;
    let g33 = -0.000000002;
    let g34 = 0.0;
    let g41 = 1.779242;
    let g42 = -0.0095436;
    let g43 = -0.0000091;
    let g44 = 0.0;
    let g51 = 130.681389;
    let g52 = 1.098935;
    let g53 = 0.00024987;
    let g54 = -0.000004718;
    let g61 = 30.10957;
    let g62 = 62.2;
    let g63 = -6.87;

    let mut pl: Vec<PlDataStruct> = Vec::new();
    pl.push(PlDataStruct {
        value1: 0.0,
        value2: 0.0,
        value3: 0.0,
        value4: 0.0,
        value5: 0.0,
        value6: 0.0,
        value7: 0.0,
        value8: 0.0,
        value9: 0.0,
    });

    let mut ip = 0;
    let b = lct_ut(lh, lm, ls, ds, zc, dy, mn, yr);
    let gd = lct_gday(lh, lm, ls, ds, zc, dy, mn, yr);
    let gm = lct_gmonth(lh, lm, ls, ds, zc, dy, mn, yr);
    let gy = lct_gyear(lh, lm, ls, ds, zc, dy, mn, yr);
    let a = cd_jd(gd, gm, gy);
    let t = ((a - 2415020.0) / 36525.0) + (b / 876600.0);

    let u_s = s.to_lowercase();

    if u_s == "mercury" {
        ip = 1;
    }
    if u_s == "venus" {
        ip = 2;
    }
    if u_s == "mars" {
        ip = 3;
    }
    if u_s == "jupiter" {
        ip = 4;
    }
    if u_s == "saturn" {
        ip = 5;
    }
    if u_s == "uranus" {
        ip = 6;
    }
    if u_s == "neptune" {
        ip = 7;
    }
    if ip == 0 {
        return (
            degrees(unwind(0.0)),
            degrees(unwind(0.0)),
            degrees(unwind(0.0)),
            degrees(unwind(0.0)),
            degrees(unwind(0.0)),
            degrees(unwind(0.0)),
            degrees(unwind(0.0)),
        );
    }

    let a0 = a11;
    let a1 = a12;
    let a2 = a13;
    let a3 = a14;
    let b0 = a21;
    let b1 = a22;
    let b2 = a23;
    let b3 = a24;
    let c0 = a31;
    let c1 = a32;
    let c2 = a33;
    let c3 = a34;
    let d0 = a41;
    let d1 = a42;
    let d2 = a43;
    let d3 = a44;
    let e0 = a51;
    let e1 = a52;
    let e2 = a53;
    let e3 = a54;
    let f = a61;
    let g = a62;
    let h = a63;
    let aa = a1 * t;
    let b = 360.0 * (aa - aa.floor());
    let c = a0 + b + (a3 * t + a2) * t * t;

    pl.push(PlDataStruct {
        value1: c - 360.0 * (c / 360.0).floor(),
        value2: (a1 * 0.009856263) + (a2 + a3) / 36525.0,
        value3: ((b3 * t + b2) * t + b1) * t + b0,
        value4: ((c3 * t + c2) * t + c1) * t + c0,
        value5: ((d3 * t + d2) * t + d1) * t + d0,
        value6: ((e3 * t + e2) * t + e1) * t + e0,
        value7: f,
        value8: g,
        value9: h,
    });

    let a0 = b11;
    let a1 = b12;
    let a2 = b13;
    let a3 = b14;
    let b0 = b21;
    let b1 = b22;
    let b2 = b23;
    let b3 = b24;
    let c0 = b31;
    let c1 = b32;
    let c2 = b33;
    let c3 = b34;
    let d0 = b41;
    let d1 = b42;
    let d2 = b43;
    let d3 = b44;
    let e0 = b51;
    let e1 = b52;
    let e2 = b53;
    let e3 = b54;
    let f = b61;
    let g = b62;
    let h = b63;
    let aa = a1 * t;
    let b = 360.0 * (aa - (aa).floor());
    let c = a0 + b + (a3 * t + a2) * t * t;
    pl.push(PlDataStruct {
        value1: c - 360.0 * (c / 360.0).floor(),
        value2: (a1 * 0.009856263) + (a2 + a3) / 36525.0,
        value3: ((b3 * t + b2) * t + b1) * t + b0,
        value4: ((c3 * t + c2) * t + c1) * t + c0,
        value5: ((d3 * t + d2) * t + d1) * t + d0,
        value6: ((e3 * t + e2) * t + e1) * t + e0,
        value7: f,
        value8: g,
        value9: h,
    });

    let a0 = c11;
    let a1 = c12;
    let a2 = c13;
    let a3 = c14;
    let b0 = c21;
    let b1 = c22;
    let b2 = c23;
    let b3 = c24;
    let c0 = c31;
    let c1 = c32;
    let c2 = c33;
    let c3 = c34;
    let d0 = c41;
    let d1 = c42;
    let d2 = c43;
    let d3 = c44;
    let e0 = c51;
    let e1 = c52;
    let e2 = c53;
    let e3 = c54;
    let f = c61;
    let g = c62;
    let h = c63;

    let aa = a1 * t;
    let b = 360.0 * (aa - (aa).floor());
    let c = a0 + b + (a3 * t + a2) * t * t;
    pl.push(PlDataStruct {
        value1: c - 360.0 * (c / 360.0).floor(),
        value2: (a1 * 0.009856263) + (a2 + a3) / 36525.0,
        value3: ((b3 * t + b2) * t + b1) * t + b0,
        value4: ((c3 * t + c2) * t + c1) * t + c0,
        value5: ((d3 * t + d2) * t + d1) * t + d0,
        value6: ((e3 * t + e2) * t + e1) * t + e0,
        value7: f,
        value8: g,
        value9: h,
    });

    let a0 = d11;
    let a1 = d12;
    let a2 = d13;
    let a3 = d14;
    let b0 = d21;
    let b1 = d22;
    let b2 = d23;
    let b3 = d24;
    let c0 = d31;
    let c1 = d32;
    let c2 = d33;
    let c3 = d34;
    let d0 = d41;
    let d1 = d42;
    let d2 = d43;
    let d3 = d44;
    let e0 = d51;
    let e1 = d52;
    let e2 = d53;
    let e3 = d54;
    let f = d61;
    let g = d62;
    let h = d63;

    let aa = a1 * t;
    let b = 360.0 * (aa - (aa).floor());
    let c = a0 + b + (a3 * t + a2) * t * t;
    pl.push(PlDataStruct {
        value1: c - 360.0 * (c / 360.0).floor(),
        value2: (a1 * 0.009856263) + (a2 + a3) / 36525.0,
        value3: ((b3 * t + b2) * t + b1) * t + b0,
        value4: ((c3 * t + c2) * t + c1) * t + c0,
        value5: ((d3 * t + d2) * t + d1) * t + d0,
        value6: ((e3 * t + e2) * t + e1) * t + e0,
        value7: f,
        value8: g,
        value9: h,
    });

    let a0 = e11;
    let a1 = e12;
    let a2 = e13;
    let a3 = e14;
    let b0 = e21;
    let b1 = e22;
    let b2 = e23;
    let b3 = e24;
    let c0 = e31;
    let c1 = e32;
    let c2 = e33;
    let c3 = e34;
    let d0 = e41;
    let d1 = e42;
    let d2 = e43;
    let d3 = e44;
    let e0 = e51;
    let e1 = e52;
    let e2 = e53;
    let e3 = e54;
    let f = e61;
    let g = e62;
    let h = e63;

    let aa = a1 * t;
    let b = 360.0 * (aa - (aa).floor());
    let c = a0 + b + (a3 * t + a2) * t * t;
    pl.push(PlDataStruct {
        value1: c - 360.0 * (c / 360.0).floor(),
        value2: (a1 * 0.009856263) + (a2 + a3) / 36525.0,
        value3: ((b3 * t + b2) * t + b1) * t + b0,
        value4: ((c3 * t + c2) * t + c1) * t + c0,
        value5: ((d3 * t + d2) * t + d1) * t + d0,
        value6: ((e3 * t + e2) * t + e1) * t + e0,
        value7: f,
        value8: g,
        value9: h,
    });

    let a0 = f11;
    let a1 = f12;
    let a2 = f13;
    let a3 = f14;
    let b0 = f21;
    let b1 = f22;
    let b2 = f23;
    let b3 = f24;
    let c0 = f31;
    let c1 = f32a;
    let c2 = f33;
    let c3 = f34;
    let d0 = f41;
    let d1 = f42;
    let d2 = f43;
    let d3 = f44;
    let e0 = f51;
    let e1 = f52;
    let e2 = f53;
    let e3 = f54;
    let f = f61;
    let g = f62;
    let h = f63;

    let aa = a1 * t;
    let b = 360.0 * (aa - (aa).floor());
    let c = a0 + b + (a3 * t + a2) * t * t;
    pl.push(PlDataStruct {
        value1: c - 360.0 * (c / 360.0).floor(),
        value2: (a1 * 0.009856263) + (a2 + a3) / 36525.0,
        value3: ((b3 * t + b2) * t + b1) * t + b0,
        value4: ((c3 * t + c2) * t + c1) * t + c0,
        value5: ((d3 * t + d2) * t + d1) * t + d0,
        value6: ((e3 * t + e2) * t + e1) * t + e0,
        value7: f,
        value8: g,
        value9: h,
    });

    let a0 = g11;
    let a1 = g12;
    let a2 = g13;
    let a3 = g14;
    let b0 = g21;
    let b1 = g22;
    let b2 = g23;
    let b3 = g24;
    let c0 = g31;
    let c1 = g32;
    let c2 = g33;
    let c3 = g34;
    let d0 = g41;
    let d1 = g42;
    let d2 = g43;
    let d3 = g44;
    let e0 = g51;
    let e1 = g52;
    let e2 = g53;
    let e3 = g54;
    let f = g61;
    let g = g62;
    let h = g63;

    let aa = a1 * t;
    let b = 360.0 * (aa - (aa).floor());
    let c = a0 + b + (a3 * t + a2) * t * t;
    pl.push(PlDataStruct {
        value1: c - 360.0 * (c / 360.0).floor(),
        value2: (a1 * 0.009856263) + (a2 + a3) / 36525.0,
        value3: ((b3 * t + b2) * t + b1) * t + b0,
        value4: ((c3 * t + c2) * t + c1) * t + c0,
        value5: ((d3 * t + d2) * t + d1) * t + d0,
        value6: ((e3 * t + e2) * t + e1) * t + e0,
        value7: f,
        value8: g,
        value9: h,
    });

    let mut li = 0.0;
    let _tp = 2.0 * std::f64::consts::PI;
    let ms = sun_mean_anomaly(lh, lm, ls, ds, zc, dy, mn, yr);
    let sr = (sun_long(lh, lm, ls, ds, zc, dy, mn, yr)).to_radians();
    let re = sun_dist(lh, lm, ls, ds, zc, dy, mn, yr);
    let lg = sr + std::f64::consts::PI;

    let mut l0 = 0.0;
    let _v0 = 0.0;
    let mut s0 = 0.0;
    let mut p0 = 0.0;
    let mut vo = 0.0;
    let mut lp1 = 0.0;
    let mut ll = 0.0;
    let mut rd = 0.0;
    let mut pd = 0.0;
    let mut sp = 0.0;
    let mut ci = 0.0;

    for k in 1..3 {
        let pl_instance = pl.clone();
        let mut ap: Vec<f64> = Vec::new();
        ap.push(0.0);
        for j in 1..8 {
            let pl_loop_instance = pl.clone();

            ap.push(
                (pl_loop_instance[j as usize].value1
                    - pl_loop_instance[j as usize].value3
                    - li * pl_loop_instance[j as usize].value2)
                    .to_radians(),
            );
        }

        let mut qa = 0.0;
        let mut qb = 0.0;
        let mut qc = 0.0;
        let mut qd = 0.0;
        let mut qe = 0.0;
        let mut qf = 0.0;
        let mut qg = 0.0;
        let _a = 0.0;
        let _sa = 0.0;
        let _ca = 0.0;

        if ip == 1 {
            let (qa_temp, qb_temp) = planet_long_l4685(ap.clone());
            qa = qa_temp;
            qb = qb_temp;
        }
        if ip == 2 {
            let (qa_temp, qb_temp, qc_temp, qe_temp) = planet_long_l4735(ap.clone(), ms, t);
            qa = qa_temp;
            qb = qb_temp;
            qc = qc_temp;
            qe = qe_temp;
        }
        if ip == 3 {
            let (_a_temp, _sa_temp, _ca_temp, qc_temp, qe_temp, qa_temp, qb_temp) =
                planet_long_l4810(ap.clone(), ms);
            // a = a_temp;
            // sa = sa_temp;
            // ca = ca_temp;
            qc = qc_temp;
            qe = qe_temp;
            qa = qa_temp;
            qb = qb_temp;
        }
        if [4, 5, 6, 7].contains(&ip) {
            let (qa_temp, qb_temp, qc_temp, qd_temp, qe_temp, qf_temp, qg_temp) =
                planet_long_l4945(t, ip, pl_instance.clone());
            qa = qa_temp;
            qb = qb_temp;
            qc = qc_temp;
            qd = qd_temp;
            qe = qe_temp;
            qf = qf_temp;
            qg = qg_temp;
        }
        let ec = pl_instance[ip as usize].value4 + qd;
        let am = ap[ip as usize] + qe;
        let at = true_anomaly(am, ec);
        let pvv =
            (pl_instance[ip as usize].value7 + qf) * (1.0 - ec * ec) / (1.0 + ec * (at).cos());
        let lp = degrees(at) + pl_instance[ip as usize].value3 + degrees(qc - qe);
        let lp = lp.to_radians();
        let om = (pl_instance[ip as usize].value6).to_radians();
        let lo = lp - om;
        let so = (lo).sin();
        let co = (lo).cos();
        let inn = (pl_instance[ip as usize].value5).to_radians();
        let pvv = pvv + qb;
        sp = so * (inn).sin();
        let y = so * (inn).cos();
        let ps = (sp).asin() + qg;
        sp = (ps).sin();
        pd = y.atan2(co) + om + (qa).to_radians();
        pd = unwind(pd);
        ci = (ps).cos();
        rd = pvv * ci;
        ll = pd - lg;
        let rh = re * re + pvv * pvv - 2.0 * re * pvv * ci * (ll).cos();
        let rh = (rh).sqrt();
        li = rh * 0.005775518;

        if k == 1 {
            l0 = pd;
            // v0 = rh;
            s0 = ps;
            p0 = pvv;
            vo = rh;
            lp1 = lp;
        }
    }

    let l1 = (ll).sin();
    let l2 = (ll).cos();

    // let mut ep = 0.0;
    let ep: f64;
    if ip < 3 {
        ep = (-1.0 * rd * l1 / (re - rd * l2)).atan() + lg + std::f64::consts::PI;
    } else {
        ep = (re * l1 / (rd - re * l2)).atan() + pd
    }

    let ep = unwind(ep);
    let bp = (rd * sp * (ep - pd).sin() / (ci * re * l1)).atan();

    let planet_longitude = degrees(unwind(ep));
    let planet_latitude = degrees(unwind(bp));
    let planet_distance_au = vo;
    let planet_h_long1 = degrees(lp1);
    let planet_h_long2 = degrees(l0);
    let planet_h_lat = degrees(s0);
    let planet_r_vect = p0;

    return (
        planet_longitude,
        planet_latitude,
        planet_distance_au,
        planet_h_long1,
        planet_h_long2,
        planet_h_lat,
        planet_r_vect,
    );
}

/// Helper function for planet_long_lat()
pub fn planet_long_l4685(ap: Vec<f64>) -> (f64, f64) {
    let qa = 0.00204 * (5.0 * ap[2] - 2.0 * ap[1] + 0.21328).cos();
    let qa = qa + 0.00103 * (2.0 * ap[2] - ap[1] - 2.8046).cos();
    let qa = qa + 0.00091 * (2.0 * ap[4] - ap[1] - 0.64582).cos();
    let qa = qa + 0.00078 * (5.0 * ap[2] - 3.0 * ap[1] + 0.17692).cos();

    let qb = 0.000007525 * (2.0 * ap[4] - ap[1] + 0.925251).cos();
    let qb = qb + 0.000006802 * (5.0 * ap[2] - 3.0 * ap[1] - 4.53642).cos();
    let qb = qb + 0.000005457 * (2.0 * ap[2] - 2.0 * ap[1] - 1.24246).cos();
    let qb = qb + 0.000003569 * (5.0 * ap[2] - ap[1] - 1.35699).cos();

    return (qa, qb);
}

/// Helper function for planet_long_lat()
pub fn planet_long_l4735(ap: Vec<f64>, ms: f64, t: f64) -> (f64, f64, f64, f64) {
    let qc = 0.00077 * (4.1406 + t * 2.6227).sin();
    let qc = qc.to_radians();
    let qe = qc;

    let qa = 0.00313 * (2.0 * ms - 2.0 * ap[2] - 2.587).cos();
    let qa = qa + 0.00198 * (3.0 * ms - 3.0 * ap[2] + 0.044768).cos();
    let qa = qa + 0.00136 * (ms - ap[2] - 2.0788).cos();
    let qa = qa + 0.00096 * (3.0 * ms - 2.0 * ap[2] - 2.3721).cos();
    let qa = qa + 0.00082 * (ap[4] - ap[2] - 3.6318).cos();

    let qb = 0.000022501 * (2.0 * ms - 2.0 * ap[2] - 1.01592).cos();
    let qb = qb + 0.000019045 * (3.0 * ms - 3.0 * ap[2] + 1.61577).cos();
    let qb = qb + 0.000006887 * (ap[4] - ap[2] - 2.06106).cos();
    let qb = qb + 0.000005172 * (ms - ap[2] - 0.508065).cos();
    let qb = qb + 0.00000362 * (5.0 * ms - 4.0 * ap[2] - 1.81877).cos();
    let qb = qb + 0.000003283 * (4.0 * ms - 4.0 * ap[2] + 1.10851).cos();
    let qb = qb + 0.000003074 * (2.0 * ap[4] - 2.0 * ap[2] - 0.962846).cos();

    return (qa, qb, qc, qe);
}

/// Helper function for planet_long_lat()
pub fn planet_long_l4810(ap: Vec<f64>, ms: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let a = 3.0 * ap[4] - 8.0 * ap[3] + 4.0 * ms;
    let sa = a.sin();
    let ca = a.cos();
    let qc = -(0.01133 * sa + 0.00933 * ca);
    let qc = qc.to_radians();
    let qe = qc;

    let qa = 0.00705 * (ap[4] - ap[3] - 0.85448).cos();
    let qa = qa + 0.00607 * (2.0 * ap[4] - ap[3] - 3.2873).cos();
    let qa = qa + 0.00445 * (2.0 * ap[4] - 2.0 * ap[3] - 3.3492).cos();
    let qa = qa + 0.00388 * (ms - 2.0 * ap[3] + 0.35771).cos();
    let qa = qa + 0.00238 * (ms - ap[3] + 0.61256).cos();
    let qa = qa + 0.00204 * (2.0 * ms - 3.0 * ap[3] + 2.7688).cos();
    let qa = qa + 0.00177 * (3.0 * ap[3] - ap[2] - 1.0053).cos();
    let qa = qa + 0.00136 * (2.0 * ms - 4.0 * ap[3] + 2.6894).cos();
    let qa = qa + 0.00104 * (ap[4] + 0.30749).cos();

    let qb = 0.000053227 * (ap[4] - ap[3] + 0.717864).cos();
    let qb = qb + 0.000050989 * (2.0 * ap[4] - 2.0 * ap[3] - 1.77997).cos();
    let qb = qb + 0.000038278 * (2.0 * ap[4] - ap[3] - 1.71617).cos();
    let qb = qb + 0.000015996 * (ms - ap[3] - 0.969618).cos();
    let qb = qb + 0.000014764 * (2.0 * ms - 3.0 * ap[3] + 1.19768).cos();
    let qb = qb + 0.000008966 * (ap[4] - 2.0 * ap[3] + 0.761225).cos();
    let qb = qb + 0.000007914 * (3.0 * ap[4] - 2.0 * ap[3] - 2.43887).cos();
    let qb = qb + 0.000007004 * (2.0 * ap[4] - 3.0 * ap[3] - 1.79573).cos();
    let qb = qb + 0.00000662 * (ms - 2.0 * ap[3] + 1.97575).cos();
    let qb = qb + 0.00000493 * (3.0 * ap[4] - 3.0 * ap[3] - 1.33069).cos();
    let qb = qb + 0.000004693 * (3.0 * ms - 5.0 * ap[3] + 3.32665).cos();
    let qb = qb + 0.000004571 * (2.0 * ms - 4.0 * ap[3] + 4.27086).cos();
    let qb = qb + 0.000004409 * (3.0 * ap[4] - ap[3] - 2.02158).cos();

    return (a, sa, ca, qc, qe, qa, qb);
}

/// Helper function for planet_long_lat()
pub fn planet_long_l4945(
    t: f64,
    ip: i32,
    pl: Vec<PlDataStruct>,
) -> (f64, f64, f64, f64, f64, f64, f64) {
    let qa = 0.0;
    let qb = 0.0;
    let qc = 0.0;
    let qd = 0.0;
    let qe = 0.0;
    let qf = 0.0;
    let qg = 0.0;

    let j1 = t / 5.0 + 0.1;
    let j2 = unwind(4.14473 + 52.9691 * t);
    let j3 = unwind(4.641118 + 21.32991 * t);
    let j4 = unwind(4.250177 + 7.478172 * t);
    let j5 = 5.0 * j3 - 2.0 * j2;
    let j6 = 2.0 * j2 - 6.0 * j3 + 3.0 * j4;

    if [1, 2, 3, 8].contains(&ip) {
        return (qa, qb, qc, qd, qe, qf, qg);
    }
    if [4, 5].contains(&ip) {
        let j7 = j3 - j2;
        let u1 = (j3).sin();
        let u2 = (j3).cos();
        let u3 = (2.0 * j3).sin();
        let u4 = (2.0 * j3).cos();
        let u5 = (j5).sin();
        let u6 = (j5).cos();
        let u7 = (2.0 * j5).sin();
        let u8a = (j6).sin();
        let u9 = (j7).sin();
        let ua = (j7).cos();
        let ub = (2.0 * j7).sin();
        let uc = (2.0 * j7).cos();
        let ud = (3.0 * j7).sin();
        let ue = (3.0 * j7).cos();
        let uf = (4.0 * j7).sin();
        let ug = (4.0 * j7).cos();
        let vh = (5.0 * j7).cos();

        if ip == 5 {
            let ui = (3.0 * j3).sin();
            let uj = (3.0 * j3).cos();
            let uk = (4.0 * j3).sin();
            let ul = (4.0 * j3).cos();
            let vi = (2.0 * j5).cos();
            let un = (5.0 * j7).sin();
            let j8 = j4 - j3;
            let uo = (2.0 * j8).sin();
            let up = (2.0 * j8).cos();
            let uq = (3.0 * j8).sin();
            let ur = (3.0 * j8).cos();

            let qc = 0.007581 * u7 - 0.007986 * u8a - 0.148811 * u9;
            let qc = qc - (0.814181 - (0.01815 - 0.016714 * j1) * j1) * u5;
            let qc = qc - (0.010497 - (0.160906 - 0.0041 * j1) * j1) * u6;
            let qc = qc - 0.015208 * ud - 0.006339 * uf - 0.006244 * u1;
            let qc = qc - 0.0165 * ub * u1 - 0.040786 * ub;
            let qc = qc + (0.008931 + 0.002728 * j1) * u9 * u1 - 0.005775 * ud * u1;
            let qc = qc + (0.081344 + 0.003206 * j1) * ua * u1 + 0.015019 * uc * u1;
            let qc = qc + (0.085581 + 0.002494 * j1) * u9 * u2 + 0.014394 * uc * u2;
            let qc = qc + (0.025328 - 0.003117 * j1) * ua * u2 + 0.006319 * ue * u2;
            let qc = qc + 0.006369 * u9 * u3 + 0.009156 * ub * u3 + 0.007525 * uq * u3;
            let qc = qc - 0.005236 * ua * u4 - 0.007736 * uc * u4 - 0.007528 * ur * u4;
            let qc = qc.to_radians();

            let qd = (-7927.0 + (2548.0 + 91.0 * j1) * j1) * u5;
            let qd = qd + (13381.0 + (1226.0 - 253.0 * j1) * j1) * u6 + (248.0 - 121.0 * j1) * u7;
            let qd = qd - (305.0 + 91.0 * j1) * vi + 412.0 * ub + 12415.0 * u1;
            let qd = qd + (390.0 - 617.0 * j1) * u9 * u1 + (165.0 - 204.0 * j1) * ub * u1;
            let qd = qd + 26599.0 * ua * u1 - 4687.0 * uc * u1 - 1870.0 * ue * u1 - 821.0 * ug * u1;
            let qd = qd - 377.0 * vh * u1 + 497.0 * up * u1 + (163.0 - 611.0 * j1) * u2;
            let qd = qd - 12696.0 * u9 * u2 - 4200.0 * ub * u2 - 1503.0 * ud * u2 - 619.0 * uf * u2;
            let qd = qd - 268.0 * un * u2 - (282.0 + 1306.0 * j1) * ua * u2;
            let qd = qd + (-86.0 + 230.0 * j1) * uc * u2 + 461.0 * uo * u2 - 350.0 * u3;
            let qd = qd + (2211.0 - 286.0 * j1) * u9 * u3 - 2208.0 * ub * u3 - 568.0 * ud * u3;
            let qd = qd - 346.0 * uf * u3 - (2780.0 + 222.0 * j1) * ua * u3;
            let qd = qd + (2022.0 + 263.0 * j1) * uc * u3 + 248.0 * ue * u3 + 242.0 * uq * u3;
            let qd = qd + 467.0 * ur * u3 - 490.0 * u4 - (2842.0 + 279.0 * j1) * u9 * u4;
            let qd = qd + (128.0 + 226.0 * j1) * ub * u4 + 224.0 * ud * u4;
            let qd = qd + (-1594.0 + 282.0 * j1) * ua * u4 + (2162.0 - 207.0 * j1) * uc * u4;
            let qd = qd + 561.0 * ue * u4 + 343.0 * ug * u4 + 469.0 * uq * u4 - 242.0 * ur * u4;
            let qd = qd - 205.0 * u9 * ui + 262.0 * ud * ui + 208.0 * ua * uj - 271.0 * ue * uj;
            let qd = qd - 382.0 * ue * uk - 376.0 * ud * ul;
            let qd = qd * 0.0000001;

            let vk = (0.077108 + (0.007186 - 0.001533 * j1) * j1) * u5;
            let vk = vk - 0.007075 * u9;
            let vk = vk + (0.045803 - (0.014766 + 0.000536 * j1) * j1) * u6;
            let vk = vk - 0.072586 * u2 - 0.075825 * u9 * u1 - 0.024839 * ub * u1;
            let vk = vk - 0.008631 * ud * u1 - 0.150383 * ua * u2;
            let vk = vk + 0.026897 * uc * u2 + 0.010053 * ue * u2;
            let vk = vk - (0.013597 + 0.001719 * j1) * u9 * u3 + 0.011981 * ub * u4;
            let vk = vk - (0.007742 - 0.001517 * j1) * ua * u3;
            let vk = vk + (0.013586 - 0.001375 * j1) * uc * u3;
            let vk = vk - (0.013667 - 0.001239 * j1) * u9 * u4;
            let vk = vk + (0.014861 + 0.001136 * j1) * ua * u4;
            let vk = vk - (0.013064 + 0.001628 * j1) * uc * u4;
            let qe = qc - ((vk).to_radians() / pl[ip as usize].value4);

            let qf = 572.0 * u5 - 1590.0 * ub * u2 + 2933.0 * u6 - 647.0 * ud * u2;
            let qf = qf + 33629.0 * ua - 344.0 * uf * u2 - 3081.0 * uc + 2885.0 * ua * u2;
            let qf = qf - 1423.0 * ue + (2172.0 + 102.0 * j1) * uc * u2 - 671.0 * ug;
            let qf = qf + 296.0 * ue * u2 - 320.0 * vh - 267.0 * ub * u3 + 1098.0 * u1;
            let qf = qf - 778.0 * ua * u3 - 2812.0 * u9 * u1 + 495.0 * uc * u3 + 688.0 * ub * u1;
            let qf = qf + 250.0 * ue * u3 - 393.0 * ud * u1 - 856.0 * u9 * u4 - 228.0 * uf * u1;
            let qf = qf + 441.0 * ub * u4 + 2138.0 * ua * u1 + 296.0 * uc * u4 - 999.0 * uc * u1;
            let qf = qf + 211.0 * ue * u4 - 642.0 * ue * u1 - 427.0 * u9 * ui - 325.0 * ug * u1;
            let qf = qf + 398.0 * ud * ui - 890.0 * u2 + 344.0 * ua * uj + 2206.0 * u9 * u2;
            let qf = qf - 427.0 * ue * uj;
            let qf = qf * 0.000001;

            let qg = 0.000747 * ua * u1 + 0.001069 * ua * u2 + 0.002108 * ub * u3;
            let qg = qg + 0.001261 * uc * u3 + 0.001236 * ub * u4 - 0.002075 * uc * u4;
            let qg = qg.to_radians();

            return (qa, qb, qc, qd, qe, qf, qg);
        }

        let qc = (0.331364 - (0.010281 + 0.004692 * j1) * j1) * u5;
        let qc = qc + (0.003228 - (0.064436 - 0.002075 * j1) * j1) * u6;
        let qc = qc - (0.003083 + (0.000275 - 0.000489 * j1) * j1) * u7;
        let qc = qc + 0.002472 * u8a + 0.013619 * u9 + 0.018472 * ub;
        let qc = qc + 0.006717 * ud + 0.002775 * uf + 0.006417 * ub * u1;
        let qc = qc + (0.007275 - 0.001253 * j1) * u9 * u1 + 0.002439 * ud * u1;
        let qc = qc - (0.035681 + 0.001208 * j1) * u9 * u2 - 0.003767 * uc * u1;
        let qc = qc - (0.033839 + 0.001125 * j1) * ua * u1 - 0.004261 * ub * u2;
        let qc = qc + (0.001161 * j1 - 0.006333) * ua * u2 + 0.002178 * u2;
        let qc = qc - 0.006675 * uc * u2 - 0.002664 * ue * u2 - 0.002572 * u9 * u3;
        let qc = qc - 0.003567 * ub * u3 + 0.002094 * ua * u4 + 0.003342 * uc * u4;
        let qc = qc.to_radians();

        let qd = (3606.0 + (130.0 - 43.0 * j1) * j1) * u5 + (1289.0 - 580.0 * j1) * u6;
        let qd = qd - 6764.0 * u9 * u1 - 1110.0 * ub * u1 - 224.0 * ud * u1 - 204.0 * u1;
        let qd = qd + (1284.0 + 116.0 * j1) * ua * u1 + 188.0 * uc * u1;
        let qd = qd + (1460.0 + 130.0 * j1) * u9 * u2 + 224.0 * ub * u2 - 817.0 * u2;
        let qd = qd + 6074.0 * u2 * ua + 992.0 * uc * u2 + 508.0 * ue * u2 + 230.0 * ug * u2;
        let qd = qd + 108.0 * vh * u2 - (956.0 + 73.0 * j1) * u9 * u3 + 448.0 * ub * u3;
        let qd = qd + 137.0 * ud * u3 + (108.0 * j1 - 997.0) * ua * u3 + 480.0 * uc * u3;
        let qd = qd + 148.0 * ue * u3 + (99.0 * j1 - 956.0) * u9 * u4 + 490.0 * ub * u4;
        let qd = qd + 158.0 * ud * u4 + 179.0 * u4 + (1024.0 + 75.0 * j1) * ua * u4;
        let qd = qd - 437.0 * uc * u4 - 132.0 * ue * u4;
        let qd = qd * 0.0000001;

        let vk = (0.007192 - 0.003147 * j1) * u5 - 0.004344 * u1;
        let vk = vk + (j1 * (0.000197 * j1 - 0.000675) - 0.020428) * u6;
        let vk = vk + 0.034036 * ua * u1 + (0.007269 + 0.000672 * j1) * u9 * u1;
        let vk = vk + 0.005614 * uc * u1 + 0.002964 * ue * u1 + 0.037761 * u9 * u2;
        let vk = vk + 0.006158 * ub * u2 - 0.006603 * ua * u2 - 0.005356 * u9 * u3;
        let vk = vk + 0.002722 * ub * u3 + 0.004483 * ua * u3;
        let vk = vk - 0.002642 * uc * u3 + 0.004403 * u9 * u4;
        let vk = vk - 0.002536 * ub * u4 + 0.005547 * ua * u4 - 0.002689 * uc * u4;
        let qe = qc - (vk.to_radians() / pl[ip as usize].value4);

        let qf = 205.0 * ua - 263.0 * u6 + 693.0 * uc + 312.0 * ue + 147.0 * ug + 299.0 * u9 * u1;
        let qf = qf + 181.0 * uc * u1 + 204.0 * ub * u2 + 111.0 * ud * u2 - 337.0 * ua * u2;
        let qf = qf - 111.0 * uc * u2;
        let qf = qf * 0.000001;

        return (qa, qb, qc, qd, qe, qf, qg);
    }

    if [6, 7].contains(&ip) {
        let j8 = unwind(1.46205 + 3.81337 * t);
        let j9 = 2.0 * j8 - j4;
        let vj = (j9).sin();
        let uu = (j9).cos();
        let uv = (2.0 * j9).sin();
        let uw = (2.0 * j9).cos();

        if ip == 7 {
            let ja = j8 - j2;
            let jb = j8 - j3;
            let jc = j8 - j4;
            let qc = (0.001089 * j1 - 0.589833) * vj;
            let qc = qc + (0.004658 * j1 - 0.056094) * uu - 0.024286 * uv;
            let qc = qc.to_radians();

            let vk = 0.024039 * vj - 0.025303 * uu + 0.006206 * uv;
            let vk = vk - 0.005992 * uw;
            let qe = qc - (vk.to_radians() / pl[ip as usize].value4);

            let qd = 4389.0 * vj + 1129.0 * uv + 4262.0 * uu + 1089.0 * uw;
            let qd = qd * 0.0000001;

            let qf = 8189.0 * uu - 817.0 * vj + 781.0 * uw;
            let qf = qf * 0.000001;

            let vd = (2.0 * jc).sin();
            let ve = (2.0 * jc).cos();
            let vf = (j8).sin();
            let vg = (j8).cos();
            let qa = -0.009556 * (ja).sin() - 0.005178 * (jb).sin();
            let qa = qa + 0.002572 * vd - 0.002972 * ve * vf - 0.002833 * vd * vg;

            let qg = 0.000336 * ve * vf + 0.000364 * vd * vg;
            let qg = qg.to_radians();

            let qb = -40596.0 + 4992.0 * (ja).cos() + 2744.0 * (jb).cos();
            let qb = qb + 2044.0 * (jc).cos() + 1051.0 * ve;
            let qb = qb * 0.000001;

            return (qa, qb, qc, qd, qe, qf, qg);
        }

        let ja = j4 - j2;
        let jb = j4 - j3;
        let jc = j8 - j4;
        let qc = (0.864319 - 0.001583 * j1) * vj;
        let qc = qc + (0.082222 - 0.006833 * j1) * uu + 0.036017 * uv;
        let qc = qc - 0.003019 * uw + 0.008122 * (j6).sin();
        let qc = qc.to_radians();

        let vk = 0.120303 * vj + 0.006197 * uv;
        let vk = vk + (0.019472 - 0.000947 * j1) * uu;
        let qe = qc - ((vk).to_radians() / pl[ip as usize].value4);

        let qd = (163.0 * j1 - 3349.0) * vj + 20981.0 * uu + 1311.0 * uw;
        let qd = qd * 0.0000001;

        let qf = -0.003825 * uu;

        let qa = (-0.038581 + (0.002031 - 0.00191 * j1) * j1) * (j4 + jb).cos();
        let qa = qa + (0.010122 - 0.000988 * j1) * (j4 + jb).sin();
        let a = (0.034964 - (0.001038 - 0.000868 * j1) * j1) * (2.0 * j4 + jb).cos();
        let qa = a + qa + 0.005594 * (j4 + 3.0 * jc).sin() - 0.014808 * (ja).sin();
        let qa = qa - 0.005794 * (jb).sin() + 0.002347 * (jb).cos();
        let qa = qa + 0.009872 * (jc).sin() + 0.008803 * (2.0 * jc).sin();
        let qa = qa - 0.004308 * (3.0 * jc).sin();

        let ux = jb.sin();
        let uy = jb.cos();
        let uz = j4.sin();
        let va = j4.cos();
        let vb = (2.0 * j4).sin();
        let vc = (2.0 * j4).cos();
        let qg = (0.000458 * ux - 0.000642 * uy - 0.000517 * (4.0 * jc).cos()) * uz;
        let qg = qg - (0.000347 * ux + 0.000853 * uy + 0.000517 * (4.0 * jb).sin()) * va;
        let qg = qg + 0.000403 * ((2.0 * jc).cos() * vb + (2.0 * jc).sin() * vc);
        let qg = qg.to_radians();

        let qb = -25948.0 + 4985.0 * (ja).cos() - 1230.0 * va + 3354.0 * uy;
        let qb = qb + 904.0 * (2.0 * jc).cos() + 894.0 * ((jc).cos() - (3.0 * jc).cos());
        let qb = qb + (5795.0 * va - 1165.0 * uz + 1388.0 * vc) * ux;
        let qb = qb + (1351.0 * va + 5702.0 * uz + 1388.0 * vb) * uy;
        let qb = qb * 0.000001;

        return (qa, qb, qc, qd, qe, qf, qg);
    }

    return (qa, qb, qc, qd, qe, qf, qg);
}

/// For W, in radians, return S, also in radians.
///
/// Original macro name: SolveCubic
pub fn solve_cubic(w: f64) -> f64 {
    let mut s = w / 3.0;

    while 1 == 1 {
        let s2 = s * s;
        let d = (s2 + 3.0) * s - w;

        if d.abs() < 0.000001 {
            return s;
        }

        s = ((2.0 * s * s2) + w) / (3.0 * (s2 + 1.0));
    }

    return s;
}

/// Calculate longitude, latitude, and distance of parabolic-orbit comet.
///
/// Original macro names: PcometLong, PcometLat, PcometDist
///
/// ## Arguments
/// * `lh` -- Local civil time, hour part.
/// * `lm` -- Local civil time, minutes part.
/// * `ls` -- Local civil time, seconds part.
/// * `ds` -- Daylight Savings offset.
/// * `zc` -- Time zone correction, in hours.
/// * `dy` -- Local date, day part.
/// * `mn` -- Local date, month part.
/// * `yr` -- Local date, year part.
/// * `td` -- Perihelion epoch (day)
/// * `tm` -- Perihelion epoch (month)
/// * `ty` -- Perihelion epoch (year)
/// * `q` -- q (AU)
/// * `i` -- Inclination (degrees)
/// * `p` -- Perihelion (degrees)
/// * `n` -- Node (degrees)
///
/// ## Returns
/// * `comet_long_deg` -- Comet longitude (degrees)
/// * `comet_lat_deg` -- Comet lat (degrees)
/// * `comet_dist_au` -- Comet distance from Earth (AU)
pub fn p_comet_long_lat_dist(
    lh: f64,
    lm: f64,
    ls: f64,
    ds: i32,
    zc: i32,
    dy: f64,
    mn: u32,
    yr: u32,
    td: f64,
    tm: u32,
    ty: u32,
    q: f64,
    i: f64,
    p: f64,
    n: f64,
) -> (f64, f64, f64) {
    let gd = lct_gday(lh, lm, ls, ds, zc, dy, mn, yr);
    let gm = lct_gmonth(lh, lm, ls, ds, zc, dy, mn, yr);
    let gy = lct_gyear(lh, lm, ls, ds, zc, dy, mn, yr);
    let ut = lct_ut(lh, lm, ls, ds, zc, dy, mn, yr);
    let tpe = (ut / 365.242191) + cd_jd(gd, gm, gy) - cd_jd(td, tm, ty);
    let lg = (sun_long(lh, lm, ls, ds, zc, dy, mn, yr) + 180.0).to_radians();
    let re = sun_dist(lh, lm, ls, ds, zc, dy, mn, yr);

    let mut _li = 0.0;
    let mut rh2 = 0.0;
    let mut rd = 0.0;
    let mut s3 = 0.0;
    let mut c3 = 0.0;
    let mut lc = 0.0;
    let mut s2 = 0.0;
    let mut c2 = 0.0;
    for k in 1..3 {
        let s = solve_cubic(0.0364911624 * tpe / (q * (q).sqrt()));
        let nu = 2.0 * s.atan();
        let r = q * (1.0 + s * s);
        let l = nu + p.to_radians();
        let s1 = l.sin();
        let c1 = l.cos();
        let i1 = i.to_radians();
        s2 = s1 * i1.sin();
        let ps = s2.asin();
        let y = s1 * i1.cos();
        lc = y.atan2(c1) + n.to_radians();
        c2 = ps.cos();
        rd = r * c2;
        let ll = lc - lg;
        c3 = ll.cos();
        s3 = ll.sin();
        let rh = ((re * re) + (r * r) - (2.0 * re * rd * c3 * (ps).cos())).sqrt();
        if k == 1 {
            rh2 = ((re * re) + (r * r)
                - (2.0 * re * r * (ps).cos() * (l + (n).to_radians() - lg).cos()))
            .sqrt();
        }

        _li = rh * 0.005775518;
    }

    let mut ep: f64;
    if rd < re {
        ep = ((-rd * s3) / (re - (rd * c3))).atan() + lg + 3.141592654;
    } else {
        ep = ((re * s3) / (rd - (re * c3))).atan() + lc;
    }

    ep = unwind(ep);
    let tb = (rd * s2 * (ep - lc).sin()) / (c2 * re * s3);
    let bp = (tb).atan();

    let comet_long_deg = degrees(ep);
    let comet_lat_deg = degrees(bp);
    let comet_dist_au = rh2;

    return (comet_long_deg, comet_lat_deg, comet_dist_au);
}

/// Calculate longitude, latitude, and horizontal parallax of the Moon.
///
/// Original macro names: MoonLong, MoonLat, MoonHP
///
/// ## Arguments
/// * `lh` -- Local civil time, hour part.
/// * `lm` -- Local civil time, minutes part.
/// * `ls` -- Local civil time, seconds part.
/// * `ds` -- Daylight Savings offset.
/// * `zc` -- Time zone correction, in hours.
/// * `dy` -- Local date, day part.
/// * `mn` -- Local date, month part.
/// * `yr` -- Local date, year part.
///
/// ## Returns
/// * `moon_long_deg` -- Moon longitude (degrees)
/// * `moon_lat_deg` -- Moon latitude (degrees)
/// * `moon_hor_para` -- Moon horizontal parallax (degrees)
pub fn moon_long_lat_hp(
    lh: f64,
    lm: f64,
    ls: f64,
    ds: i32,
    zc: i32,
    dy: f64,
    mn: u32,
    yr: u32,
) -> (f64, f64, f64) {
    let ut = lct_ut(lh, lm, ls, ds, zc, dy, mn, yr);
    let gd = lct_gday(lh, lm, ls, ds, zc, dy, mn, yr);
    let gm = lct_gmonth(lh, lm, ls, ds, zc, dy, mn, yr);
    let gy = lct_gyear(lh, lm, ls, ds, zc, dy, mn, yr);
    let t = ((cd_jd(gd, gm, gy) - 2415020.0) / 36525.0) + (ut / 876600.0);
    let t2 = t * t;

    let m1 = 27.32158213;
    let m2 = 365.2596407;
    let m3 = 27.55455094;
    let m4 = 29.53058868;
    let m5 = 27.21222039;
    let m6 = 6798.363307;
    let q = cd_jd(gd, gm, gy) - 2415020.0 + (ut / 24.0);
    let m1 = q / m1;
    let m2 = q / m2;
    let m3 = q / m3;
    let m4 = q / m4;
    let m5 = q / m5;
    let m6 = q / m6;
    let m1 = 360.0 * (m1 - m1.floor());
    let m2 = 360.0 * (m2 - m2.floor());
    let m3 = 360.0 * (m3 - m3.floor());
    let m4 = 360.0 * (m4 - m4.floor());
    let m5 = 360.0 * (m5 - m5.floor());
    let m6 = 360.0 * (m6 - m6.floor());

    let ml = 270.434164 + m1 - (0.001133 - 0.0000019 * t) * t2;
    let ms = 358.475833 + m2 - (0.00015 + 0.0000033 * t) * t2;
    let md = 296.104608 + m3 + (0.009192 + 0.0000144 * t) * t2;
    let me1 = 350.737486 + m4 - (0.001436 - 0.0000019 * t) * t2;
    let mf = 11.250889 + m5 - (0.003211 + 0.0000003 * t) * t2;
    let na = 259.183275 - m6 + (0.002078 + 0.0000022 * t) * t2;
    let a = (51.2 + 20.2 * t).to_radians();
    let s1 = a.sin();
    let s2 = na.to_radians().sin();
    let b = 346.56 + (132.87 - 0.0091731 * t) * t;
    let s3 = 0.003964 * b.to_radians().sin();
    let c = (na + 275.05 - 2.3 * t).to_radians();
    let s4 = c.sin();
    let ml = ml + 0.000233 * s1 + s3 + 0.001964 * s2;
    let ms = ms - 0.001778 * s1;
    let md = md + 0.000817 * s1 + s3 + 0.002541 * s2;
    let mf = mf + s3 - 0.024691 * s2 - 0.004328 * s4;
    let me1 = me1 + 0.002011 * s1 + s3 + 0.001964 * s2;
    let e = 1.0 - (0.002495 + 0.00000752 * t) * t;
    let e2 = e * e;
    let ml = ml.to_radians();
    let ms = ms.to_radians();
    let na = na.to_radians();
    let me1 = me1.to_radians();
    let mf = mf.to_radians();
    let md = md.to_radians();

    // Longitude-specific
    let l = 6.28875 * md.sin() + 1.274018 * (2.0 * me1 - md).sin();
    let l = l + 0.658309 * (2.0 * me1).sin() + 0.213616 * (2.0 * md).sin();
    let l = l - e * 0.185596 * ms.sin() - 0.114336 * (2.0 * mf).sin();
    let l = l + 0.058793 * (2.0 * (me1 - md)).sin();
    let l = l + 0.057212 * e * (2.0 * me1 - ms - md).sin() + 0.05332 * (2.0 * me1 + md).sin();
    let l = l + 0.045874 * e * (2.0 * me1 - ms).sin() + 0.041024 * e * (md - ms).sin();
    let l = l - 0.034718 * me1.sin() - e * 0.030465 * (ms + md).sin();
    let l = l + 0.015326 * (2.0 * (me1 - mf)).sin() - 0.012528 * (2.0 * mf + md).sin();
    let l = l - 0.01098 * (2.0 * mf - md).sin() + 0.010674 * (4.0 * me1 - md).sin();
    let l = l + 0.010034 * (3.0 * md).sin() + 0.008548 * (4.0 * me1 - 2.0 * md).sin();
    let l = l - e * 0.00791 * (ms - md + 2.0 * me1).sin() - e * 0.006783 * (2.0 * me1 + ms).sin();
    let l = l + 0.005162 * (md - me1).sin() + e * 0.005 * (ms + me1).sin();
    let l = l + 0.003862 * (4.0 * me1).sin() + e * 0.004049 * (md - ms + 2.0 * me1).sin();
    let l = l + 0.003996 * (2.0 * (md + me1)).sin() + 0.003665 * (2.0 * me1 - 3.0 * md).sin();
    let l = l + e * 0.002695 * (2.0 * md - ms).sin() + 0.002602 * (md - 2.0 * (mf + me1)).sin();
    let l = l + e * 0.002396 * (2.0 * (me1 - md) - ms).sin() - 0.002349 * (md + me1).sin();
    let l = l + e2 * 0.002249 * (2.0 * (me1 - ms)).sin() - e * 0.002125 * (2.0 * md + ms).sin();
    let l = l - e2 * 0.002079 * (2.0 * ms).sin() + e2 * 0.002059 * (2.0 * (me1 - ms) - md).sin();
    let l = l - 0.001773 * (md + 2.0 * (me1 - mf)).sin() - 0.001595 * (2.0 * (mf + me1)).sin();
    let l = l + e * 0.00122 * (4.0 * me1 - ms - md).sin() - 0.00111 * (2.0 * (md + mf)).sin();
    let l = l + 0.000892 * (md - 3.0 * me1).sin() - e * 0.000811 * (ms + md + 2.0 * me1).sin();
    let l = l + e * 0.000761 * (4.0 * me1 - ms - 2.0 * md).sin();
    let l = l + e2 * 0.000704 * (md - 2.0 * (ms + me1)).sin();
    let l = l + e * 0.000693 * (ms - 2.0 * (md - me1)).sin();
    let l = l + e * 0.000598 * (2.0 * (me1 - mf) - ms).sin();
    let l = l + 0.00055 * (md + 4.0 * me1).sin() + 0.000538 * (4.0 * md).sin();
    let l = l + e * 0.000521 * (4.0 * me1 - ms).sin() + 0.000486 * (2.0 * md - me1).sin();
    let l = l + e2 * 0.000717 * (md - 2.0 * ms).sin();
    let mm = unwind(ml + l.to_radians());

    // Latitude-specific
    let g = 5.128189 * mf.sin() + 0.280606 * (md + mf).sin();
    let g = g + 0.277693 * (md - mf).sin() + 0.173238 * (2.0 * me1 - mf).sin();
    let g = g + 0.055413 * (2.0 * me1 + mf - md).sin() + 0.046272 * (2.0 * me1 - mf - md).sin();
    let g = g + 0.032573 * (2.0 * me1 + mf).sin() + 0.017198 * (2.0 * md + mf).sin();
    let g = g + 0.009267 * (2.0 * me1 + md - mf).sin() + 0.008823 * (2.0 * md - mf).sin();
    let g =
        g + e * 0.008247 * (2.0 * me1 - ms - mf).sin() + 0.004323 * (2.0 * (me1 - md) - mf).sin();
    let g = g + 0.0042 * (2.0 * me1 + mf + md).sin() + e * 0.003372 * (mf - ms - 2.0 * me1).sin();
    let g = g + e * 0.002472 * (2.0 * me1 + mf - ms - md).sin();
    let g = g + e * 0.002222 * (2.0 * me1 + mf - ms).sin();
    let g = g + e * 0.002072 * (2.0 * me1 - mf - ms - md).sin();
    let g = g + e * 0.001877 * (mf - ms + md).sin() + 0.001828 * (4.0 * me1 - mf - md).sin();
    let g = g - e * 0.001803 * (mf + ms).sin() - 0.00175 * (3.0 * mf).sin();
    let g = g + e * 0.00157 * (md - ms - mf).sin() - 0.001487 * (mf + me1).sin();
    let g = g - e * 0.001481 * (mf + ms + md).sin() + e * 0.001417 * (mf - ms - md).sin();
    let g = g + e * 0.00135 * (mf - ms).sin() + 0.00133 * (mf - me1).sin();
    let g = g + 0.001106 * (mf + 3.0 * md).sin() + 0.00102 * (4.0 * me1 - mf).sin();
    let g = g + 0.000833 * (mf + 4.0 * me1 - md).sin() + 0.000781 * (md - 3.0 * mf).sin();
    let g =
        g + 0.00067 * (mf + 4.0 * me1 - 2.0 * md).sin() + 0.000606 * (2.0 * me1 - 3.0 * mf).sin();
    let g = g + 0.000597 * (2.0 * (me1 + md) - mf).sin();
    let g = g
        + e * 0.000492 * (2.0 * me1 + md - ms - mf).sin()
        + 0.00045 * (2.0 * (md - me1) - mf).sin();
    let g = g + 0.000439 * (3.0 * md - mf).sin() + 0.000423 * (mf + 2.0 * (me1 + md)).sin();
    let g = g + 0.000422 * (2.0 * me1 - mf - 3.0 * md).sin()
        - e * 0.000367 * (ms + mf + 2.0 * me1 - md).sin();
    let g = g - e * 0.000353 * (ms + mf + 2.0 * me1).sin() + 0.000331 * (mf + 4.0 * me1).sin();
    let g = g + e * 0.000317 * (2.0 * me1 + mf - ms + md).sin();
    let g = g + e2 * 0.000306 * (2.0 * (me1 - ms) - mf).sin() - 0.000283 * (md + 3.0 * mf).sin();
    let w1 = 0.0004664 * na.cos();
    let w2 = 0.0000754 * c.cos();
    let bm = g.to_radians() * (1.0 - w1 - w2);

    // Horizontal parallax-specific
    let pm = 0.950724 + 0.051818 * md.cos() + 0.009531 * (2.0 * me1 - md).cos();
    let pm = pm + 0.007843 * (2.0 * me1).cos() + 0.002824 * (2.0 * md).cos();
    let pm = pm + 0.000857 * (2.0 * me1 + md).cos() + e * 0.000533 * (2.0 * me1 - ms).cos();
    let pm = pm + e * 0.000401 * (2.0 * me1 - md - ms).cos();
    let pm = pm + e * 0.00032 * (md - ms).cos() - 0.000271 * me1.cos();
    let pm = pm - e * 0.000264 * (ms + md).cos() - 0.000198 * (2.0 * mf - md).cos();
    let pm = pm + 0.000173 * (3.0 * md).cos() + 0.000167 * (4.0 * me1 - md).cos();
    let pm = pm - e * 0.000111 * ms.cos() + 0.000103 * (4.0 * me1 - 2.0 * md).cos();
    let pm = pm - 0.000084 * (2.0 * md - 2.0 * me1).cos() - e * 0.000083 * (2.0 * me1 + ms).cos();
    let pm = pm + 0.000079 * (2.0 * me1 + 2.0 * md).cos() + 0.000072 * (4.0 * me1).cos();
    let pm = pm + e * 0.000064 * (2.0 * me1 - ms + md).cos()
        - e * 0.000063 * (2.0 * me1 + ms - md).cos();
    let pm = pm + e * 0.000041 * (ms + me1).cos() + e * 0.000035 * (2.0 * md - ms).cos();
    let pm = pm - 0.000033 * (3.0 * md - 2.0 * me1).cos() - 0.00003 * (md + me1).cos();
    let pm = pm - 0.000029 * (2.0 * (mf - me1)).cos() - e * 0.000029 * (2.0 * md + ms).cos();
    let pm =
        pm + e2 * 0.000026 * (2.0 * (me1 - ms)).cos() - 0.000023 * (2.0 * (mf - me1) + md).cos();
    let pm = pm + e * 0.000019 * (4.0 * me1 - ms - md).cos();

    let moon_long_deg = degrees(mm);
    let moon_lat_deg = degrees(bm);
    let moon_hor_para = pm;

    return (moon_long_deg, moon_lat_deg, moon_hor_para);
}

/// Calculate current phase of Moon.
///
/// Original macro name: MoonPhase
pub fn moon_phase(lh: f64, lm: f64, ls: f64, ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> f64 {
    let (moon_long_deg, moon_lat_deg, _moon_hor_para) =
        moon_long_lat_hp(lh, lm, ls, ds, zc, dy, mn, yr);

    let cd = ((moon_long_deg - sun_long(lh, lm, ls, ds, zc, dy, mn, yr)).to_radians()).cos()
        * ((moon_lat_deg).to_radians()).cos();
    let d = cd.acos();
    let sd = d.sin();
    let i =
        0.1468 * sd * (1.0 - 0.0549 * (moon_mean_anomaly(lh, lm, ls, ds, zc, dy, mn, yr)).sin());
    let i = i / (1.0 - 0.0167 * (sun_mean_anomaly(lh, lm, ls, ds, zc, dy, mn, yr)).sin());
    let i = 3.141592654 - d - i.to_radians();
    let k = (1.0 + (i).cos()) / 2.0;

    return utils::round_f64(k, 2);
}

/// Calculate the Moon's mean anomaly.
///
/// Original macro name: MoonMeanAnomaly
pub fn moon_mean_anomaly(
    lh: f64,
    lm: f64,
    ls: f64,
    ds: i32,
    zc: i32,
    dy: f64,
    mn: u32,
    yr: u32,
) -> f64 {
    let ut = lct_ut(lh, lm, ls, ds, zc, dy, mn, yr);
    let gd = lct_gday(lh, lm, ls, ds, zc, dy, mn, yr);
    let gm = lct_gmonth(lh, lm, ls, ds, zc, dy, mn, yr);
    let gy = lct_gyear(lh, lm, ls, ds, zc, dy, mn, yr);
    let t = ((cd_jd(gd, gm, gy) - 2415020.0) / 36525.0) + (ut / 876600.0);
    let t2 = t * t;

    let m1 = 27.32158213;
    let m2 = 365.2596407;
    let m3 = 27.55455094;
    let m4 = 29.53058868;
    let m5 = 27.21222039;
    let m6 = 6798.363307;
    let q = cd_jd(gd, gm, gy) - 2415020.0 + (ut / 24.0);
    let m1 = q / m1;
    let m2 = q / m2;
    let m3 = q / m3;
    let m4 = q / m4;
    let m5 = q / m5;
    let m6 = q / m6;
    let m1 = 360.0 * (m1 - m1.floor());
    let m2 = 360.0 * (m2 - m2.floor());
    let m3 = 360.0 * (m3 - m3.floor());
    let m4 = 360.0 * (m4 - m4.floor());
    let m5 = 360.0 * (m5 - m5.floor());
    let m6 = 360.0 * (m6 - m6.floor());

    let ml = 270.434164 + m1 - (0.001133 - 0.0000019 * t) * t2;
    let ms = 358.475833 + m2 - (0.00015 + 0.0000033 * t) * t2;
    let md = 296.104608 + m3 + (0.009192 + 0.0000144 * t) * t2;
    let _me1 = 350.737486 + m4 - (0.001436 - 0.0000019 * t) * t2;
    let _mf = 11.250889 + m5 - (0.003211 + 0.0000003 * t) * t2;
    let na = 259.183275 - m6 + (0.002078 + 0.0000022 * t) * t2;
    let a = (51.2 + 20.2 * t).to_radians();
    let s1 = a.sin();
    let s2 = na.to_radians().sin();
    let b = 346.56 + (132.87 - 0.0091731 * t) * t;
    let s3 = 0.003964 * b.to_radians().sin();
    let c = (na + 275.05 - 2.3 * t).to_radians();
    let _s4 = c.sin();
    let _ml = ml + 0.000233 * s1 + s3 + 0.001964 * s2;
    let _ms = ms - 0.001778 * s1;
    let md = md + 0.000817 * s1 + s3 + 0.002541 * s2;

    return md.to_radians();
}

/// Calculate Julian date of New Moon.
///
/// Original macro name: NewMoon
///
/// ## Arguments
/// * `ds` -- Daylight Savings offset.
/// * `zc` -- Time zone correction, in hours.
/// * `dy` -- Local date, day part.
/// * `mn` -- Local date, month part.
/// * `yr` -- Local date, year part.
pub fn new_moon(ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> f64 {
    let d0 = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let m0 = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let y0 = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);

    /*  Irrelevant, because of the typing on y0
    if y0 < 0 {
        y0 = y0 + 1;
    }
    */

    let j0 = cd_jd(0.0, 1, y0) - 2415020.0;
    let dj = cd_jd(d0, m0, y0) - 2415020.0;
    let k = lint(((y0 as f64 - 1900.0 + ((dj - j0) / 365.0)) * 12.3685) + 0.5);
    let tn = k / 1236.85;
    let tf = (k + 0.5) / 1236.85;
    let t = tn;
    let (a, b, f) = new_moon_full_moon_l6855(k, t);
    let ni = a;
    let nf = b;
    let _nb = f;
    let t = tf;
    let k = k + 0.5;
    let (a, b, f) = new_moon_full_moon_l6855(k, t);
    let _fi = a;
    let _ff = b;
    let _fb = f;

    return ni + 2415020.0 + nf;
}

/// Calculate Julian date of Full Moon.
///
/// Original macro name: FullMoon
///
/// ## Arguments
/// * `ds` -- Daylight Savings offset.
/// * `zc` -- Time zone correction, in hours.
/// * `dy` -- Local date, day part.
/// * `mn` -- Local date, month part.
/// * `yr` -- Local date, year part.
pub fn full_moon(ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> f64 {
    let d0 = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let m0 = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let y0 = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);

    /*  Irrelevant, because of the typing on y0
    if y0 < 0 {
        y0 = y0 + 1;
    }
    */

    let j0 = cd_jd(0.0, 1, y0) - 2415020.0;
    let dj = cd_jd(d0, m0, y0) - 2415020.0;
    let k = lint(((y0 as f64 - 1900.0 + ((dj - j0) / 365.0)) * 12.3685) + 0.5);
    let tn = k / 1236.85;
    let tf = (k + 0.5) / 1236.85;
    let t = tn;
    let (a, b, f) = new_moon_full_moon_l6855(k, t);
    let _ni = a;
    let _nf = b;
    let _nb = f;
    let t = tf;
    let k = k + 0.5;
    let (a, b, f) = new_moon_full_moon_l6855(k, t);
    let fi = a;
    let ff = b;
    let _fb = f;

    return fi + 2415020.0 + ff;
}

/// Helper function for new_moon() and full_moon() """
pub fn new_moon_full_moon_l6855(k: f64, t: f64) -> (f64, f64, f64) {
    let t2 = t * t;
    let e = 29.53 * k;
    let c = 166.56 + (132.87 - 0.009173 * t) * t;
    let c = c.to_radians();
    let b = 0.00058868 * k + (0.0001178 - 0.000000155 * t) * t2;
    let b = b + 0.00033 * c.sin() + 0.75933;
    let a = k / 12.36886;
    let a1 = 359.2242 + 360.0 * fract(a) - (0.0000333 + 0.00000347 * t) * t2;
    let a2 = 306.0253 + 360.0 * fract(k / 0.9330851);
    let a2 = a2 + (0.0107306 + 0.00001236 * t) * t2;
    let a = k / 0.9214926;
    let f = 21.2964 + 360.0 * fract(a) - (0.0016528 + 0.00000239 * t) * t2;
    let a1 = unwind_deg(a1);
    let a2 = unwind_deg(a2);
    let f = unwind_deg(f);
    let a1 = a1.to_radians();
    let a2 = a2.to_radians();
    let f = f.to_radians();

    let dd = (0.1734 - 0.000393 * t) * a1.sin() + 0.0021 * (2.0 * a1).sin();
    let dd = dd - 0.4068 * a2.sin() + 0.0161 * (2.0 * a2).sin() - 0.0004 * (3.0 * a2).sin();
    let dd = dd + 0.0104 * (2.0 * f).sin() - 0.0051 * (a1 + a2).sin();
    let dd = dd - 0.0074 * (a1 - a2).sin() + 0.0004 * (2.0 * f + a1).sin();
    let dd = dd - 0.0004 * (2.0 * f - a1).sin() - 0.0006 * (2.0 * f + a2).sin()
        + 0.001 * (2.0 * f - a2).sin();
    let dd = dd + 0.0005 * (a1 + 2.0 * a2).sin();
    let e1 = e.floor();
    let b = b + dd + (e - e1);
    let b1 = b.floor();
    let a = e1 + b1;
    let b = b - b1;

    return (a, b, f);
}

/// Original macro name: FRACT
pub fn fract(w: f64) -> f64 {
    return w - lint(w);
}

/// Original macro name: LINT
pub fn lint(w: f64) -> f64 {
    return iint(w) + iint(((1.0 * sgn(w)) - 1.0) / 2.0);
}

/// Original macro name: IINT
pub fn iint(w: f64) -> f64 {
    return sgn(w) * w.abs().floor();
}

/// Calculate sign of number.
///
/// ## Arguments
/// * `number_to_check` -- Number to calculate the sign of.
///
/// ## Returns
/// * `sign_value` -- Sign value: -1, 0, or 1
pub fn sgn(number_to_check: f64) -> f64 {
    let mut sign_value = 0.0;

    if number_to_check < 0.0 {
        sign_value = -1.0;
    }

    if number_to_check > 0.0 {
        sign_value = 1.0;
    }

    return sign_value;
}

/// Original macro name: UTDayAdjust
pub fn ut_day_adjust(ut: f64, g1: f64) -> f64 {
    let mut return_value = ut;

    if (ut - g1) < -6.0 {
        return_value = ut + 24.0;
    }

    if (ut - g1) > 6.0 {
        return_value = ut - 24.0;
    }

    return return_value;
}

/// Original macro name: Fpart
pub fn f_part(w: f64) -> f64 {
    return w - lint(w);
}

/// Original macro name: EQElat
pub fn eq_e_lat(
    rah: f64,
    ram: f64,
    ras: f64,
    dd: f64,
    dm: f64,
    ds: f64,
    gd: f64,
    gm: u32,
    gy: u32,
) -> f64 {
    let a = (dh_dd(hms_dh(rah, ram, ras))).to_radians();
    let b = (dms_dd(dd, dm, ds)).to_radians();
    let c = (obliq(gd, gm, gy)).to_radians();
    let d = b.sin() * c.cos() - b.cos() * c.sin() * a.sin();

    return degrees((d).asin());
}

/// Original macro name: EQElong
pub fn eq_e_long(
    rah: f64,
    ram: f64,
    ras: f64,
    dd: f64,
    dm: f64,
    ds: f64,
    gd: f64,
    gm: u32,
    gy: u32,
) -> f64 {
    let a = (dh_dd(hms_dh(rah, ram, ras))).to_radians();
    let b = (dms_dd(dd, dm, ds)).to_radians();
    let c = (obliq(gd, gm, gy)).to_radians();
    let d = a.sin() * c.cos() + b.tan() * c.sin();
    let e = a.cos();
    let f = degrees(d.atan2(e));

    return f - 360.0 * (f / 360.0).floor();
}

/// Local time of moonrise.
///
/// Original macro name: MoonRiseLCT
///
/// ## Returns
/// * `hours`
pub fn moon_rise_lct(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32, g_long: f64, g_lat: f64) -> f64 {
    let mut gdy = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gmn = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gyr = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut lct = 12.0;
    let mut dy1 = dy;
    let mut mn1 = mn;
    let mut yr1 = yr;

    let (mm_temp, bm_temp, pm_temp, dp_temp, th_temp, di_temp, p_temp, q_temp, lu_temp, lct_temp) =
        moon_rise_lct_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
    let _mm = mm_temp;
    let _bm = bm_temp;
    let _pm = pm_temp;
    let _dp = dp_temp;
    let _th = th_temp;
    let _di = di_temp;
    let _p = p_temp;
    let _q = q_temp;
    let mut lu = lu_temp;
    lct = lct_temp;

    if lct == -99.0 {
        return lct;
    }
    let mut la = lu;

    let mut x: f64;
    let mut ut: f64;
    let mut g1 = 0.0;
    let mut gu = 0.0;
    for k in 1..9 {
        x = lst_gst(la, 0.0, 0.0, g_long);
        ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

        g1 = if k == 1 { ut } else { gu };

        gu = ut;
        ut = gu;

        let (_ut_temp, lct_temp, dy1_temp, mn1_temp, yr1_temp, gdy_temp, gmn_temp, gyr_temp) =
            moon_rise_lct_l6680(x, ds, zc, gdy, gmn, gyr, g1, ut);
        lct = lct_temp;
        dy1 = dy1_temp;
        mn1 = mn1_temp;
        yr1 = yr1_temp;
        gdy = gdy_temp;
        gmn = gmn_temp;
        gyr = gyr_temp;

        let (
            _mm_temp,
            _bm_temp,
            _pm_temp,
            _dp_temp,
            _th_temp,
            _di_temp,
            _p_temp,
            _q_temp,
            lu_temp,
            lct_temp,
        ) = moon_rise_lct_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
        lu = lu_temp;
        lct = lct_temp;

        if lct == -99.0 {
            return lct;
        }
        la = lu;
    }

    x = lst_gst(la, 0.0, 0.0, g_long);
    ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);

    return lct;
}

/// Helper function for moon_rise_lct
pub fn moon_rise_lct_l6680(
    x: f64,
    ds: i32,
    zc: i32,
    mut gdy: f64,
    mut gmn: u32,
    mut gyr: u32,
    g1: f64,
    mut ut: f64,
) -> (f64, f64, f64, u32, u32, f64, u32, u32) {
    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    gdy = lct_gday(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gmn = lct_gmonth(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gyr = lct_gyear(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    ut = ut - 24.0 * (ut / 24.0).floor();

    return (ut, lct, dy1, mn1, yr1, gdy, gmn, gyr);
}

/// Helper function for moon_rise_lct
pub fn moon_rise_lct_l6700(
    mut lct: f64,
    ds: i32,
    zc: i32,
    dy1: f64,
    mn1: u32,
    yr1: u32,
    gdy: f64,
    gmn: u32,
    gyr: u32,
    g_lat: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let mm = moon_long(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let bm = moon_lat(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let pm = (moon_hp(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1)).to_radians();
    let dp = nutat_long(gdy, gmn, gyr);
    let th = 0.27249 * pm.sin();
    let di = th + 0.0098902 - pm;
    let p = dd_dh(ec_ra(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr));
    let q = ec_dec(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr);
    let lu = rise_set_local_sidereal_time_rise(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);

    if e_rs(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat) != "OK" {
        lct = -99.0;
    }

    return (mm, bm, pm, dp, th, di, p, q, lu, lct);
}

/// Moonrise calculation status.
///
/// Original macro name: eMoonRise
pub fn e_moon_rise(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32, g_long: f64, g_lat: f64) -> String {
    let mut s4: String = "OK".to_string();
    let mut gdy = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gmn = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gyr = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut lct = 12.0;
    let mut dy1 = dy;
    let mut mn1 = mn;
    let mut yr1 = yr;

    let (mm_temp, bm_temp, pm_temp, dp_temp, th_temp, di_temp, p_temp, q_temp, lu_temp, s1_temp) =
        e_moon_rise_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
    let _mm = mm_temp;
    let _bm = bm_temp;
    let _pm = pm_temp;
    let _dp = dp_temp;
    let _th = th_temp;
    let _di = di_temp;
    let _p = p_temp;
    let _q = q_temp;
    let mut lu = lu_temp;
    let mut s1 = s1_temp.to_string();

    let mut la = lu;

    if s1 != "OK" {
        return s1;
    }

    let mut x: f64;
    let mut ut: f64;
    let mut s3: String;
    let mut g1: f64;
    let mut gu = 0.0;
    for k in 1..9 {
        x = lst_gst(la, 0.0, 0.0, g_long);
        ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);
        s3 = e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

        if s3 != "OK" {
            s4 = ["GST conversion:", &s3].join(" ");
        }

        g1 = if k == 1 { ut } else { gu };

        gu = ut;
        ut = gu;
        let (_ut_temp, lct_temp, dy1_temp, mn1_temp, yr1_temp, gdy_temp, gmn_temp, gyr_temp) =
            e_moon_rise_l6680(s3, g1, ut, ds, zc, gdy, gmn, gyr, dy1, mn1, yr1);
        lct = lct_temp;
        dy1 = dy1_temp;
        mn1 = mn1_temp;
        yr1 = yr1_temp;
        gdy = gdy_temp;
        gmn = gmn_temp;
        gyr = gyr_temp;

        let (
            _mm_temp,
            _bm_temp,
            _pm_temp,
            _dp_temp,
            _th_temp,
            _di_temp,
            _p_temp,
            _q_temp,
            lu_temp,
            s1_temp,
        ) = e_moon_rise_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
        lu = lu_temp;
        s1 = s1_temp;

        la = lu;

        if s1 != "OK" {
            return s1;
        }
    }

    x = lst_gst(la, 0.0, 0.0, g_long);
    s3 = e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

    if s3 != "OK" {
        s4 = ["GST conversion:", &s3].join(" ");
    }

    return s4;
}

/// Helper function for e_moon_rise()
pub fn e_moon_rise_l6680(
    s3: String,
    g1: f64,
    mut ut: f64,
    ds: i32,
    zc: i32,
    mut gdy: f64,
    mut gmn: u32,
    mut gyr: u32,
    _dy1: f64,
    _mn1: u32,
    _yr1: u32,
) -> (f64, f64, f64, u32, u32, f64, u32, u32) {
    if s3 != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    gdy = lct_gday(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gmn = lct_gmonth(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gyr = lct_gyear(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    ut = ut - 24.0 * (ut / 24.0).floor();

    return (ut, lct, dy1, mn1, yr1, gdy, gmn, gyr);
}

/// Helper function for e_moon_rise()
pub fn e_moon_rise_l6700(
    lct: f64,
    ds: i32,
    zc: i32,
    dy1: f64,
    mn1: u32,
    yr1: u32,
    gdy: f64,
    gmn: u32,
    gyr: u32,
    g_lat: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, String) {
    let mm = moon_long(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let bm = moon_lat(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let pm = (moon_hp(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1)).to_radians();
    let dp = nutat_long(gdy, gmn, gyr);
    let th = 0.27249 * pm.sin();
    let di = th + 0.0098902 - pm;
    let p = dd_dh(ec_ra(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr));
    let q = ec_dec(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr);
    let lu = rise_set_local_sidereal_time_rise(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);
    let s1 = e_rs(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);

    return (mm, bm, pm, dp, th, di, p, q, lu, s1);
}

/// Local date of moonrise.
///
/// Original macro names: MoonRiseLcDay, MoonRiseLcMonth, MoonRiseLcYear
///
/// ## Returns
/// * Local date (day)
/// * Local date (month)
/// * Local date (year)
pub fn moon_rise_lc_dmy(
    dy: f64,
    mn: u32,
    yr: u32,
    ds: i32,
    zc: i32,
    g_long: f64,
    g_lat: f64,
) -> (f64, u32, u32) {
    let mut gdy = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gmn = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gyr = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut lct = 12.0;
    let mut dy1 = dy;
    let mut mn1 = mn;
    let mut yr1 = yr;

    let (mm_temp, bm_temp, pm_temp, dp_temp, th_temp, di_temp, p_temp, q_temp, lu_temp, lct_temp) =
        moon_rise_lc_dmy_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
    let _mm = mm_temp;
    let _bm = bm_temp;
    let _pm = pm_temp;
    let _dp = dp_temp;
    let _th = th_temp;
    let _di = di_temp;
    let _p = p_temp;
    let _q = q_temp;
    let mut lu = lu_temp;
    lct = lct_temp;

    if lct == -99.0 {
        return (lct, lct as u32, lct as u32);
    }
    let mut la = lu;

    let mut x: f64;
    let mut ut: f64;
    let mut g1 = 0.0;
    let mut gu = 0.0;
    for k in 1..9 {
        x = lst_gst(la, 0.0, 0.0, g_long);
        ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

        g1 = if k == 1 { ut } else { gu };

        gu = ut;
        ut = gu;

        let (_ut_temp, lct_temp, dy1_temp, mn1_temp, yr1_temp, gdy_temp, gmn_temp, gyr_temp) =
            moon_rise_lc_dmy_l6680(x, ds, zc, gdy, gmn, gyr, g1, ut);
        lct = lct_temp;
        dy1 = dy1_temp;
        mn1 = mn1_temp;
        yr1 = yr1_temp;
        gdy = gdy_temp;
        gmn = gmn_temp;
        gyr = gyr_temp;

        let (
            _mm_temp,
            _bm_temp,
            _pm_temp,
            _dp_temp,
            _th_temp,
            _di_temp,
            _p_temp,
            _q_temp,
            lu_temp,
            lct_temp,
        ) = moon_rise_lc_dmy_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
        lu = lu_temp;
        lct = lct_temp;

        if lct == -99.0 {
            return (lct, lct as u32, lct as u32);
        }
        la = lu;
    }

    x = lst_gst(la, 0.0, 0.0, g_long);
    ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);

    return (dy1, mn1, yr1);
}

/// Helper function for moon_rise_lc_dmy
pub fn moon_rise_lc_dmy_l6680(
    x: f64,
    ds: i32,
    zc: i32,
    mut gdy: f64,
    mut gmn: u32,
    mut gyr: u32,
    g1: f64,
    mut ut: f64,
) -> (f64, f64, f64, u32, u32, f64, u32, u32) {
    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    gdy = lct_gday(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gmn = lct_gmonth(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gyr = lct_gyear(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    ut = ut - 24.0 * (ut / 24.0).floor();

    return (ut, lct, dy1, mn1, yr1, gdy, gmn, gyr);
}

/// Helper function for moon_rise_lc_dmy
pub fn moon_rise_lc_dmy_l6700(
    lct: f64,
    ds: i32,
    zc: i32,
    dy1: f64,
    mn1: u32,
    yr1: u32,
    gdy: f64,
    gmn: u32,
    gyr: u32,
    g_lat: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let mm = moon_long(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let bm = moon_lat(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let pm = (moon_hp(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1)).to_radians();
    let dp = nutat_long(gdy, gmn, gyr);
    let th = 0.27249 * pm.sin();
    let di = th + 0.0098902 - pm;
    let p = dd_dh(ec_ra(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr));
    let q = ec_dec(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr);
    let lu = rise_set_local_sidereal_time_rise(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);

    return (mm, bm, pm, dp, th, di, p, q, lu, lct);
}

/// Local azimuth of moonrise.
///
/// Original macro name: MoonRiseAz
///
/// ## Returns
/// * degrees
pub fn moon_rise_az(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32, g_long: f64, g_lat: f64) -> f64 {
    let mut gdy = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gmn = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gyr = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut lct = 12.0;
    let mut dy1 = dy;
    let mut mn1 = mn;
    let mut yr1 = yr;

    let (
        mm_temp,
        bm_temp,
        pm_temp,
        dp_temp,
        th_temp,
        di_temp,
        p_temp,
        q_temp,
        lu_temp,
        lct_temp,
        _au_temp,
    ) = moon_rise_az_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
    let _mm = mm_temp;
    let _bm = bm_temp;
    let _pm = pm_temp;
    let _dp = dp_temp;
    let _th = th_temp;
    let _di = di_temp;
    let _p = p_temp;
    let _q = q_temp;
    let mut lu = lu_temp;
    lct = lct_temp;
    let mut au: f64;

    if lct == -99.0 {
        return lct;
    }
    let mut la = lu;

    let mut x: f64;
    let mut ut: f64;
    let mut g1: f64;
    let mut gu = 0.0;
    let mut aa = 0.0;
    for k in 1..9 {
        x = lst_gst(la, 0.0, 0.0, g_long);
        ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

        g1 = if k == 1 { ut } else { gu };

        gu = ut;
        ut = gu;

        let (_ut_temp, lct_temp, dy1_temp, mn1_temp, yr1_temp, gdy_temp, gmn_temp, gyr_temp) =
            moon_rise_az_l6680(x, ds, zc, gdy, gmn, gyr, g1, ut);
        lct = lct_temp;
        dy1 = dy1_temp;
        mn1 = mn1_temp;
        yr1 = yr1_temp;
        gdy = gdy_temp;
        gmn = gmn_temp;
        gyr = gyr_temp;

        let (
            _mm_temp,
            _bm_temp,
            _pm_temp,
            _dp_temp,
            _th_temp,
            _di_temp,
            _p_temp,
            _q_temp,
            lu_temp,
            lct_temp,
            au_temp,
        ) = moon_rise_az_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
        lu = lu_temp;
        lct = lct_temp;
        au = au_temp;

        if lct == -99.0 {
            return lct;
        }
        la = lu;
        aa = au;
    }

    au = aa;

    return au;
}

/// Helper function for moon_rise_az
pub fn moon_rise_az_l6680(
    x: f64,
    ds: i32,
    zc: i32,
    mut gdy: f64,
    mut gmn: u32,
    mut gyr: u32,
    g1: f64,
    mut ut: f64,
) -> (f64, f64, f64, u32, u32, f64, u32, u32) {
    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    gdy = lct_gday(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gmn = lct_gmonth(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gyr = lct_gyear(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    ut = ut - 24.0 * (ut / 24.0).floor();

    return (ut, lct, dy1, mn1, yr1, gdy, gmn, gyr);
}

/// Helper function for moon_rise_az
pub fn moon_rise_az_l6700(
    lct: f64,
    ds: i32,
    zc: i32,
    dy1: f64,
    mn1: u32,
    yr1: u32,
    gdy: f64,
    gmn: u32,
    gyr: u32,
    g_lat: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let mm = moon_long(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let bm = moon_lat(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let pm = (moon_hp(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1)).to_radians();
    let dp = nutat_long(gdy, gmn, gyr);
    let th = 0.27249 * pm.sin();
    let di = th + 0.0098902 - pm;
    let p = dd_dh(ec_ra(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr));
    let q = ec_dec(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr);
    let lu = rise_set_local_sidereal_time_rise(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);
    let au = rise_set_azimuth_rise(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);

    return (mm, bm, pm, dp, th, di, p, q, lu, lct, au);
}

/// Local time of moonset.
///
/// Original macro name: MoonSetLCT
///
/// ## Returns
/// * hours
pub fn moon_set_lct(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32, g_long: f64, g_lat: f64) -> f64 {
    let mut gdy = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gmn = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gyr = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut lct = 12.0;
    let mut dy1 = dy;
    let mut mn1 = mn;
    let mut yr1 = yr;

    let (mm_temp, bm_temp, pm_temp, dp_temp, th_temp, di_temp, p_temp, q_temp, lu_temp, lct_temp) =
        moon_set_lct_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
    let _mm = mm_temp;
    let _bm = bm_temp;
    let _pm = pm_temp;
    let _dp = dp_temp;
    let _th = th_temp;
    let _di = di_temp;
    let _p = p_temp;
    let _q = q_temp;
    let mut lu = lu_temp;
    lct = lct_temp;

    if lct == -99.0 {
        return lct;
    }
    let mut la = lu;

    let mut x: f64;
    let mut ut: f64;
    let mut g1 = 0.0;
    let mut gu = 0.0;
    for k in 1..9 {
        x = lst_gst(la, 0.0, 0.0, g_long);
        ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

        g1 = if k == 1 { ut } else { gu };

        gu = ut;
        ut = gu;

        let (_ut_temp, lct_temp, dy1_temp, mn1_temp, yr1_temp, gdy_temp, gmn_temp, gyr_temp) =
            moon_set_lct_l6680(x, ds, zc, gdy, gmn, gyr, g1, ut);
        lct = lct_temp;
        dy1 = dy1_temp;
        mn1 = mn1_temp;
        yr1 = yr1_temp;
        gdy = gdy_temp;
        gmn = gmn_temp;
        gyr = gyr_temp;

        let (
            _mm_temp,
            _bm_temp,
            _pm_temp,
            _dp_temp,
            _th_temp,
            _di_temp,
            _p_temp,
            _q_temp,
            lu_temp,
            lct_temp,
        ) = moon_set_lct_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
        lu = lu_temp;
        lct = lct_temp;

        if lct == -99.0 {
            return lct;
        }
        la = lu;
    }

    x = lst_gst(la, 0.0, 0.0, g_long);
    ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "ok" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);

    return lct;
}

/// Helper function for moon_set_lct
pub fn moon_set_lct_l6680(
    x: f64,
    ds: i32,
    zc: i32,
    mut gdy: f64,
    mut gmn: u32,
    mut gyr: u32,
    g1: f64,
    mut ut: f64,
) -> (f64, f64, f64, u32, u32, f64, u32, u32) {
    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    gdy = lct_gday(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gmn = lct_gmonth(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gyr = lct_gyear(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    ut = ut - 24.0 * (ut / 24.0).floor();

    return (ut, lct, dy1, mn1, yr1, gdy, gmn, gyr);
}

/// Helper function for moon_set_lct
pub fn moon_set_lct_l6700(
    mut lct: f64,
    ds: i32,
    zc: i32,
    dy1: f64,
    mn1: u32,
    yr1: u32,
    gdy: f64,
    gmn: u32,
    gyr: u32,
    g_lat: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let mm = moon_long(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let bm = moon_lat(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let pm = (moon_hp(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1)).to_radians();
    let dp = nutat_long(gdy, gmn, gyr);
    let th = 0.27249 * pm.sin();
    let di = th + 0.0098902 - pm;
    let p = dd_dh(ec_ra(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr));
    let q = ec_dec(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr);
    let lu = rise_set_local_sidereal_time_set(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);

    if e_rs(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat) != "OK" {
        lct = -99.0;
    }

    return (mm, bm, pm, dp, th, di, p, q, lu, lct);
}

/// Moonset calculation status.
///
/// Original macro name: eMoonSet
pub fn e_moon_set(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32, g_long: f64, g_lat: f64) -> String {
    let mut s4: String = "OK".to_string();
    let mut gdy = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gmn = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gyr = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut lct = 12.0;
    let mut dy1 = dy;
    let mut mn1 = mn;
    let mut yr1 = yr;

    let (mm_temp, bm_temp, pm_temp, dp_temp, th_temp, di_temp, p_temp, q_temp, lu_temp, s1_temp) =
        e_moon_rise_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
    let _mm = mm_temp;
    let _bm = bm_temp;
    let _pm = pm_temp;
    let _dp = dp_temp;
    let _th = th_temp;
    let _di = di_temp;
    let _p = p_temp;
    let _q = q_temp;
    let mut lu = lu_temp;
    let mut s1 = s1_temp.to_string();

    let mut la = lu;

    if s1 != "OK" {
        return s1;
    }

    let mut x: f64;
    let mut ut: f64;
    let mut s3: String;
    let mut g1: f64;
    let mut gu = 0.0;
    for k in 1..9 {
        x = lst_gst(la, 0.0, 0.0, g_long);
        ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);
        s3 = e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

        if s3 != "OK" {
            s4 = ["GST conversion:", &s3].join(" ");
        }

        g1 = if k == 1 { ut } else { gu };

        gu = ut;
        ut = gu;

        let (_ut_temp, lct_temp, dy1_temp, mn1_temp, yr1_temp, gdy_temp, gmn_temp, gyr_temp) =
            e_moon_set_l6680(x, g1, ut, ds, zc, gdy, gmn, gyr, dy1, mn1, yr1);
        lct = lct_temp;
        dy1 = dy1_temp;
        mn1 = mn1_temp;
        yr1 = yr1_temp;
        gdy = gdy_temp;
        gmn = gmn_temp;
        gyr = gyr_temp;

        let (
            _mm_temp,
            _bm_temp,
            _pm_temp,
            _dp_temp,
            _th_temp,
            _di_temp,
            _p_temp,
            _q_temp,
            lu_temp,
            s1_temp,
        ) = e_moon_set_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
        lu = lu_temp;
        s1 = s1_temp;

        la = lu;

        if s1 != "OK" {
            return s1;
        }
    }

    x = lst_gst(la, 0.0, 0.0, g_long);
    s3 = e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

    if s3 != "OK" {
        s4 = ["GST conversion:", &s3].join(" ");
    }

    return s4;
}

/// Helper function for e_moon_set()
pub fn e_moon_set_l6680(
    x: f64,
    g1: f64,
    mut ut: f64,
    ds: i32,
    zc: i32,
    mut gdy: f64,
    mut gmn: u32,
    mut gyr: u32,
    _dy1: f64,
    _mn1: u32,
    _yr1: u32,
) -> (f64, f64, f64, u32, u32, f64, u32, u32) {
    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    gdy = lct_gday(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gmn = lct_gmonth(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gyr = lct_gyear(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    ut = ut - 24.0 * (ut / 24.0).floor();

    return (ut, lct, dy1, mn1, yr1, gdy, gmn, gyr);
}

/// Helper function for e_moon_set()
pub fn e_moon_set_l6700(
    lct: f64,
    ds: i32,
    zc: i32,
    dy1: f64,
    mn1: u32,
    yr1: u32,
    gdy: f64,
    gmn: u32,
    gyr: u32,
    g_lat: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, String) {
    let mm = moon_long(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let bm = moon_lat(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let pm = (moon_hp(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1)).to_radians();
    let dp = nutat_long(gdy, gmn, gyr);
    let th = 0.27249 * pm.sin();
    let di = th + 0.0098902 - pm;
    let p = dd_dh(ec_ra(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr));
    let q = ec_dec(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr);
    let lu = rise_set_local_sidereal_time_set(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);
    let s1 = e_rs(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);

    return (mm, bm, pm, dp, th, di, p, q, lu, s1);
}

/// Local date of moonset.
///
/// Original macro names: MoonSetLcDay, MoonSetLcMonth, MoonSetLcYear
///
/// ## Returns
/// * Local date (day)
/// * Local date (month)
/// * Local date (year)
pub fn moon_set_lc_dmy(
    dy: f64,
    mn: u32,
    yr: u32,
    ds: i32,
    zc: i32,
    g_long: f64,
    g_lat: f64,
) -> (f64, u32, u32) {
    let mut gdy = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gmn = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gyr = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut lct = 12.0;
    let mut dy1 = dy;
    let mut mn1 = mn;
    let mut yr1 = yr;

    let (mm_temp, bm_temp, pm_temp, dp_temp, th_temp, di_temp, p_temp, q_temp, lu_temp, lct_temp) =
        moon_set_lc_dmy_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
    let _mm = mm_temp;
    let _bm = bm_temp;
    let _pm = pm_temp;
    let _dp = dp_temp;
    let _th = th_temp;
    let _di = di_temp;
    let _p = p_temp;
    let _q = q_temp;
    let mut lu = lu_temp;
    lct = lct_temp;

    if lct == -99.0 {
        return (lct, lct as u32, lct as u32);
    }
    let mut la = lu;

    let mut x: f64;
    let mut ut: f64;
    let mut g1 = 0.0;
    let mut gu = 0.0;
    for k in 1..9 {
        x = lst_gst(la, 0.0, 0.0, g_long);
        ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

        g1 = if k == 1 { ut } else { gu };

        gu = ut;
        ut = gu;

        let (_ut_temp, lct_temp, dy1_temp, mn1_temp, yr1_temp, gdy_temp, gmn_temp, gyr_temp) =
            moon_set_lc_dmy_l6680(x, ds, zc, gdy, gmn, gyr, g1, ut);
        lct = lct_temp;
        dy1 = dy1_temp;
        mn1 = mn1_temp;
        yr1 = yr1_temp;
        gdy = gdy_temp;
        gmn = gmn_temp;
        gyr = gyr_temp;

        let (
            _mm_temp,
            _bm_temp,
            _pm_temp,
            _dp_temp,
            _th_temp,
            _di_temp,
            _p_temp,
            _q_temp,
            lu_temp,
            lct_temp,
        ) = moon_set_lc_dmy_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
        lu = lu_temp;
        lct = lct_temp;

        if lct == -99.0 {
            return (lct, lct as u32, lct as u32);
        }
        la = lu;
    }

    x = lst_gst(la, 0.0, 0.0, g_long);
    ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);

    return (dy1, mn1, yr1);
}

/// Helper function for moon_set_lc_dmy
pub fn moon_set_lc_dmy_l6680(
    x: f64,
    ds: i32,
    zc: i32,
    mut gdy: f64,
    mut gmn: u32,
    mut gyr: u32,
    g1: f64,
    mut ut: f64,
) -> (f64, f64, f64, u32, u32, f64, u32, u32) {
    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    gdy = lct_gday(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gmn = lct_gmonth(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gyr = lct_gyear(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    ut = ut - 24.0 * (ut / 24.0).floor();

    return (ut, lct, dy1, mn1, yr1, gdy, gmn, gyr);
}

/// Helper function for moon_set_lc_dmy
pub fn moon_set_lc_dmy_l6700(
    lct: f64,
    ds: i32,
    zc: i32,
    dy1: f64,
    mn1: u32,
    yr1: u32,
    gdy: f64,
    gmn: u32,
    gyr: u32,
    g_lat: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let mm = moon_long(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let bm = moon_lat(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let pm = (moon_hp(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1)).to_radians();
    let dp = nutat_long(gdy, gmn, gyr);
    let th = 0.27249 * pm.sin();
    let di = th + 0.0098902 - pm;
    let p = dd_dh(ec_ra(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr));
    let q = ec_dec(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr);
    let lu = rise_set_local_sidereal_time_set(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);

    return (mm, bm, pm, dp, th, di, p, q, lu, lct);
}

/// Local azimuth of moonset.
///
/// Original macro name: MoonSetAz
///
/// ## Returns
/// * degrees
pub fn moon_set_az(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32, g_long: f64, g_lat: f64) -> f64 {
    let mut gdy = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gmn = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut gyr = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let mut lct = 12.0;
    let mut dy1 = dy;
    let mut mn1 = mn;
    let mut yr1 = yr;

    let (
        mm_temp,
        bm_temp,
        pm_temp,
        dp_temp,
        th_temp,
        di_temp,
        p_temp,
        q_temp,
        lu_temp,
        lct_temp,
        _au_temp,
    ) = moon_set_az_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
    let _mm = mm_temp;
    let _bm = bm_temp;
    let _pm = pm_temp;
    let _dp = dp_temp;
    let _th = th_temp;
    let _di = di_temp;
    let _p = p_temp;
    let _q = q_temp;
    let mut lu = lu_temp;
    lct = lct_temp;
    let mut au: f64;

    if lct == -99.0 {
        return lct;
    }
    let mut la = lu;

    let mut x: f64;
    let mut ut: f64;
    let mut g1: f64;
    let mut gu = 0.0;
    let mut aa = 0.0;
    for k in 1..9 {
        x = lst_gst(la, 0.0, 0.0, g_long);
        ut = gst_ut(x, 0.0, 0.0, gdy, gmn, gyr);

        g1 = if k == 1 { ut } else { gu };

        gu = ut;
        ut = gu;

        let (_ut_temp, lct_temp, dy1_temp, mn1_temp, yr1_temp, gdy_temp, gmn_temp, gyr_temp) =
            moon_set_az_l6680(x, ds, zc, gdy, gmn, gyr, g1, ut);
        lct = lct_temp;
        dy1 = dy1_temp;
        mn1 = mn1_temp;
        yr1 = yr1_temp;
        gdy = gdy_temp;
        gmn = gmn_temp;
        gyr = gyr_temp;

        let (
            _mm_temp,
            _bm_temp,
            _pm_temp,
            _dp_temp,
            _th_temp,
            _di_temp,
            _p_temp,
            _q_temp,
            lu_temp,
            lct_temp,
            au_temp,
        ) = moon_set_az_l6700(lct, ds, zc, dy1, mn1, yr1, gdy, gmn, gyr, g_lat);
        lu = lu_temp;
        lct = lct_temp;
        au = au_temp;

        if lct == -99.0 {
            return lct;
        }
        la = lu;
        aa = au;
    }

    au = aa;

    return au;
}

/// Helper function for moon_set_az
pub fn moon_set_az_l6680(
    x: f64,
    ds: i32,
    zc: i32,
    mut gdy: f64,
    mut gmn: u32,
    mut gyr: u32,
    g1: f64,
    mut ut: f64,
) -> (f64, f64, f64, u32, u32, f64, u32, u32) {
    if e_gst_ut(x, 0.0, 0.0, gdy, gmn, gyr) != "OK" {
        if (g1 - ut).abs() > 0.5 {
            ut = ut + 23.93447;
        }
    }

    ut = ut_day_adjust(ut, g1);
    let lct = ut_lct(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let dy1 = ut_lc_day(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let mn1 = ut_lc_month(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    let yr1 = ut_lc_year(ut, 0.0, 0.0, ds, zc, gdy, gmn, gyr);
    gdy = lct_gday(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gmn = lct_gmonth(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    gyr = lct_gyear(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    ut = ut - 24.0 * (ut / 24.0).floor();

    return (ut, lct, dy1, mn1, yr1, gdy, gmn, gyr);
}

/// Helper function for moon_set_az
pub fn moon_set_az_l6700(
    lct: f64,
    ds: i32,
    zc: i32,
    dy1: f64,
    mn1: u32,
    yr1: u32,
    gdy: f64,
    gmn: u32,
    gyr: u32,
    g_lat: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let mm = moon_long(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let bm = moon_lat(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1);
    let pm = (moon_hp(lct, 0.0, 0.0, ds, zc, dy1, mn1, yr1)).to_radians();
    let dp = nutat_long(gdy, gmn, gyr);
    let th = 0.27249 * pm.sin();
    let di = th + 0.0098902 - pm;
    let p = dd_dh(ec_ra(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr));
    let q = ec_dec(mm + dp, 0.0, 0.0, bm, 0.0, 0.0, gdy, gmn, gyr);
    let lu = rise_set_local_sidereal_time_set(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);
    let au = rise_set_azimuth_set(p, 0.0, 0.0, q, 0.0, 0.0, degrees(di), g_lat);

    return (mm, bm, pm, dp, th, di, p, q, lu, lct, au);
}

/// Determine if a lunar eclipse is likely to occur.
///
/// Original macro name: LEOccurrence
pub fn lunar_eclipse_occurrence(ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> String {
    let d0 = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let m0 = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let y0 = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);

    /* Comparison is impossible because of type limits
    if y0 < 0 {
        y0 = y0 + 1;
    }
    */

    let j0 = cd_jd(0.0, 1, y0);
    let dj = cd_jd(d0, m0, y0);
    let mut k = (y0 as f64 - 1900.0 + ((dj - j0) * 1.0 / 365.0)) * 12.3685;
    k = lint(k + 0.5);
    let tn = k / 1236.85;
    let tf = (k + 0.5) / 1236.85;
    let mut t = tn;
    let (f, _dd, _e1, _b1, a, b) = lunar_eclipse_occurrence_l6855(t, k);
    let _ni = a;
    let _nf = b;
    let _nb = f;
    t = tf;
    k = k + 0.5;
    let (f, _dd, _e1, _b1, a, b) = lunar_eclipse_occurrence_l6855(t, k);
    let _fi = a;
    let _ff = b;
    let fb = f;

    let mut df = (fb - 3.141592654 * lint(fb / 3.141592654)).abs();

    if df > 0.37 {
        df = 3.141592654 - df;
    }

    let mut s = "Lunar eclipse certain";
    if df >= 0.242600766 {
        s = "Lunar eclipse possible";
        if df > 0.37 {
            s = "No lunar eclipse"
        }
    }

    return s.to_string();
}

/// Helper function for lunar_eclipse_occurrence
pub fn lunar_eclipse_occurrence_l6855(t: f64, k: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2 = t * t;
    let e = 29.53 * k;
    let c = 166.56 + (132.87 - 0.009173 * t) * t;
    let c = c.to_radians();
    let b = 0.00058868 * k + (0.0001178 - 0.000000155 * t) * t2;
    let b = b + 0.00033 * c.sin() + 0.75933;
    let a = k / 12.36886;
    let a1 = 359.2242 + 360.0 * f_part(a) - (0.0000333 + 0.00000347 * t) * t2;
    let a2 = 306.0253 + 360.0 * f_part(k / 0.9330851);
    let a2 = a2 + (0.0107306 + 0.00001236 * t) * t2;
    let a = k / 0.9214926;
    let f = 21.2964 + 360.0 * f_part(a) - (0.0016528 + 0.00000239 * t) * t2;
    let a1 = unwind_deg(a1);
    let a2 = unwind_deg(a2);
    let f = unwind_deg(f);
    let a1 = a1.to_radians();
    let a2 = a2.to_radians();
    let f = f.to_radians();

    let dd = (0.1734 - 0.000393 * t) * a1.sin() + 0.0021 * (2.0 * a1).sin();
    let dd = dd - 0.4068 * a2.sin() + 0.0161 * (2.0 * a2).sin() - 0.0004 * (3.0 * a2).sin();
    let dd = dd + 0.0104 * (2.0 * f).sin() - 0.0051 * (a1 + a2).sin();
    let dd = dd - 0.0074 * (a1 - a2).sin() + 0.0004 * (2.0 * f + a1).sin();
    let dd = dd - 0.0004 * (2.0 * f - a1).sin() - 0.0006 * (2.0 * f + a2).sin()
        + 0.001 * (2.0 * f - a2).sin();
    let dd = dd + 0.0005 * (a1 + 2.0 * a2).sin();
    let e1 = e.floor();
    let b = b + dd + (e - e1);
    let b1 = b.floor();
    let a = e1 + b1;
    let b = b - b1;

    return (f, dd, e1, b1, a, b);
}

/// Calculate time of maximum shadow for lunar eclipse (UT)
///
/// Original macro name: UTMaxLunarEclipse
pub fn ut_max_lunar_eclipse(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if lunar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No lunar eclipse" {
        return -99.0;
    }

    let dj = full_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utfm = xi * 24.0;
    let ut = utfm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utfm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utfm;
    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let sr = sr + std::f64::consts::PI - lint((sr + std::f64::consts::PI) / tp) * tp;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let _ru = (hd - rn + ps) * 1.02;
    let rp = (hd + rn + ps) * 1.02;
    let _pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rp;
    let mut dd = z1 - x0;
    dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    return z1;
}

/// Calculate time of first shadow contact for lunar eclipse (UT)
///
/// Original macro name: UTFirstContactLunarEclipse
pub fn ut_first_contact_lunar_eclipse(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if lunar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No lunar eclipse" {
        return -99.0;
    }

    let dj = full_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utfm = xi * 24.0;
    let ut = utfm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utfm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utfm;
    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let sr = sr + std::f64::consts::PI - lint((sr + std::f64::consts::PI) / tp) * tp;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let _ru = (hd - rn + ps) * 1.02;
    let rp = (hd + rn + ps) * 1.02;
    let _pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rp;
    let mut dd = z1 - x0;
    dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let mut z6 = z1 - zd;

    if z6 < 0.0 {
        z6 = z6 + 24.0;
    }

    return z6;
}

/// Calculate time of last shadow contact for lunar eclipse (UT)
///
/// Original macro name: UTLastContactLunarEclipse
pub fn ut_last_contact_lunar_eclipse(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if lunar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No lunar eclipse" {
        return -99.0;
    }

    let dj = full_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utfm = xi * 24.0;
    let ut = utfm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utfm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utfm;
    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let sr = sr + std::f64::consts::PI - lint((sr + std::f64::consts::PI) / tp) * tp;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let _ru = (hd - rn + ps) * 1.02;
    let rp = (hd + rn + ps) * 1.02;
    let _pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rp;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let _z6 = z1 - zd;
    let z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    return z7;
}

/// Calculate start time of umbra phase of lunar eclipse (UT)
///
/// Original macro name: UTStartUmbraLunarEclipse
pub fn ut_start_umbra_lunar_eclipse(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if lunar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No lunar eclipse" {
        return -99.0;
    }

    let dj = full_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utfm = xi * 24.0;
    let ut = utfm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utfm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utfm;
    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let sr = sr + std::f64::consts::PI - lint((sr + std::f64::consts::PI) / tp) * tp;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let ru = (hd - rn + ps) * 1.02;
    let rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rp;
    let mut dd = z1 - x0;
    dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let z6 = z1 - zd;
    let _z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z6 < 0.0 {
        let _z6 = z6 + 24.0;
    }

    let r = rm + ru;
    dd = z1 - x0;
    dd = dd * dd - ((z2 - (r * r)) * dd / zh);
    let _mg = (rm + rp - pj) / (2.0 * rm);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let mut z8 = z1 - zd;
    let _z9 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z8 < 0.0 {
        z8 = z8 + 24.0;
    }

    return z8;
}

/// Calculate end time of umbra phase of lunar eclipse (UT)
///
/// Original macro name: UTEndUmbraLunarEclipse
pub fn ut_end_umbra_lunar_eclipse(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if lunar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No lunar eclipse" {
        return -99.0;
    }

    let dj = full_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utfm = xi * 24.0;
    let ut = utfm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utfm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utfm;
    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let sr = sr + std::f64::consts::PI - lint((sr + std::f64::consts::PI) / tp) * tp;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let ru = (hd - rn + ps) * 1.02;
    let rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rp;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let z6 = z1 - zd;
    let _z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z6 < 0.0 {
        let _z6 = z6 + 24.0;
    }

    let r = rm + ru;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);
    let _mg = (rm + rp - pj) / (2.0 * rm);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let _z8 = z1 - zd;
    let z9 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    return z9;
}

/// Calculate start time of total phase of lunar eclipse (UT)
///
/// Original macro name: UTStartTotalLunarEclipse
pub fn ut_start_total_lunar_eclipse(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if lunar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No lunar eclipse" {
        return -99.0;
    }

    let dj = full_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utfm = xi * 24.0;
    let ut = utfm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utfm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utfm;
    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let sr = sr + std::f64::consts::PI - lint((sr + std::f64::consts::PI) / tp) * tp;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let ru = (hd - rn + ps) * 1.02;
    let rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rp;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = (dd).sqrt();
    let z6 = z1 - zd;
    let _z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z6 < 0.0 {
        let _z6 = z6 + 24.0;
    }

    let r = rm + ru;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);
    let _mg = (rm + rp - pj) / (2.0 * rm);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = (dd).sqrt();
    let z8 = z1 - zd;
    let _z9 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z8 < 0.0 {
        let _z8 = z8 + 24.0;
    }

    let r = ru - rm;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);
    let _mg = (rm + ru - pj) / (2.0 * rm);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = (dd).sqrt();
    let mut zcc = z1 - zd;
    let _zb = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if zcc < 0.0 {
        zcc = zc as f64 + 24.0;
    }

    return zcc;
}

/// Calculate end time of total phase of lunar eclipse (UT)
///
/// Original macro name: UTEndTotalLunarEclipse
pub fn ut_end_total_lunar_eclipse(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if lunar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No lunar eclipse" {
        return -99.0;
    }

    let dj = full_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utfm = xi * 24.0;
    let ut = utfm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utfm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utfm;
    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let sr = sr + std::f64::consts::PI - lint((sr + std::f64::consts::PI) / tp) * tp;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let ru = (hd - rn + ps) * 1.02;
    let rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rp;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let z6 = z1 - zd;
    let _z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z6 < 0.0 {
        let _z6 = z6 + 24.0;
    }

    let r = rm + ru;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);
    let _mg = (rm + rp - pj) / (2.0 * rm);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let z8 = z1 - zd;
    let _z9 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z8 < 0.0 {
        let _z8 = z8 + 24.0;
    }

    let r = ru - rm;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);
    let _mg = (rm + ru - pj) / (2.0 * rm);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let zb = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    return zb;
}

/// Calculate magnitude of lunar eclipse.
///
/// Original macro name: MagLunarEclipse
pub fn mag_lunar_eclipse(dy: f64, mn: u32, yr: u32, ds: i32, zc: i32) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if lunar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No lunar eclipse" {
        return -99.0;
    }

    let dj = full_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utfm = xi * 24.0;
    let ut = utfm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utfm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utfm;
    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let sr = sr + std::f64::consts::PI - lint((sr + std::f64::consts::PI) / tp) * tp;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let ru = (hd - rn + ps) * 1.02;
    let rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rp;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let z6 = z1 - zd;
    let _z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z6 < 0.0 {
        let _z6 = z6 + 24.0;
    }

    let r = rm + ru;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);
    let mg = (rm + rp - pj) / (2.0 * rm);

    if dd < 0.0 {
        return mg;
    }

    let zd = dd.sqrt();
    let z8 = z1 - zd;
    let _z9 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z8 < 0.0 {
        let _z8 = z8 + 24.0;
    }

    let r = ru - rm;
    let dd = z1 - x0;
    let _dd = dd * dd - ((z2 - (r * r)) * dd / zh);
    let mg = (rm + ru - pj) / (2.0 * rm);

    return mg;
}

/// Determine if a solar eclipse is likely to occur.
///
/// Original macro name: SEOccurrence
pub fn solar_eclipse_occurrence(ds: i32, zc: i32, dy: f64, mn: u32, yr: u32) -> String {
    let d0 = lct_gday(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let m0 = lct_gmonth(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);
    let y0 = lct_gyear(12.0, 0.0, 0.0, ds, zc, dy, mn, yr);

    let j0 = cd_jd(0.0, 1, y0);
    let dj = cd_jd(d0, m0, y0);
    let k = (y0 as f64 - 1900.0 + ((dj - j0) * 1.0 / 365.0)) * 12.3685;
    let k = lint(k + 0.5);
    let tn = k / 1236.85;
    let tf = (k + 0.5) / 1236.85;
    let t = tn;
    let (f, _dd, _e1, _b1, a, b) = solar_eclipse_occurrence_l6855(t, k);
    let _ni = a;
    let _nf = b;
    let nb = f;
    let t = tf;
    let k = k + 0.5;
    let (f, _dd, _e1, _b1, a, b) = solar_eclipse_occurrence_l6855(t, k);
    let _fi = a;
    let _ff = b;
    let _fb = f;

    let mut df = (nb - 3.141592654 * lint(nb / 3.141592654)).abs();

    if df > 0.37 {
        df = 3.141592654 - df;
    }

    let mut s = "Solar eclipse certain";
    if df >= 0.242600766 {
        s = "Solar eclipse possible";
        if df > 0.37 {
            s = "No solar eclipse";
        }
    }

    return s.to_string();
}

/// Helper function for solar_eclipse_occurrence
pub fn solar_eclipse_occurrence_l6855(t: f64, k: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2 = t * t;
    let e = 29.53 * k;
    let c = 166.56 + (132.87 - 0.009173 * t) * t;
    let c = c.to_radians();
    let b = 0.00058868 * k + (0.0001178 - 0.000000155 * t) * t2;
    let b = b + 0.00033 * c.sin() + 0.75933;
    let a = k / 12.36886;
    let a1 = 359.2242 + 360.0 * f_part(a) - (0.0000333 + 0.00000347 * t) * t2;
    let a2 = 306.0253 + 360.0 * f_part(k / 0.9330851);
    let a2 = a2 + (0.0107306 + 0.00001236 * t) * t2;
    let a = k / 0.9214926;
    let f = 21.2964 + 360.0 * f_part(a) - (0.0016528 + 0.00000239 * t) * t2;
    let a1 = unwind_deg(a1);
    let a2 = unwind_deg(a2);
    let f = unwind_deg(f);
    let a1 = a1.to_radians();
    let a2 = a2.to_radians();
    let f = f.to_radians();

    let dd = (0.1734 - 0.000393 * t) * (a1).sin() + 0.0021 * (2.0 * a1).sin();
    let dd = dd - 0.4068 * (a2).sin() + 0.0161 * (2.0 * a2).sin() - 0.0004 * (3.0 * a2).sin();
    let dd = dd + 0.0104 * (2.0 * f).sin() - 0.0051 * (a1 + a2).sin();
    let dd = dd - 0.0074 * (a1 - a2).sin() + 0.0004 * (2.0 * f + a1).sin();
    let dd = dd - 0.0004 * (2.0 * f - a1).sin() - 0.0006 * (2.0 * f + a2).sin()
        + 0.001 * (2.0 * f - a2).sin();
    let dd = dd + 0.0005 * (a1 + 2.0 * a2).sin();
    let e1 = e.floor();
    let b = b + dd + (e - e1);
    let b1 = b.floor();
    let a = e1 + b1;
    let b = b - b1;

    return (f, dd, e1, b1, a, b);
}

/// Calculate time of maximum shadow for solar eclipse (UT)
///
/// Original macro name: UTMaxSolarEclipse
pub fn ut_max_solar_eclipse(
    dy: f64,
    mn: u32,
    yr: u32,
    ds: i32,
    zc: i32,
    glong: f64,
    glat: f64,
) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if solar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No solar eclipse" {
        return -99.0;
    }

    let dj = new_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utnm = xi * 24.0;
    let ut = utnm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utnm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utnm;
    let x = my;
    let y = by;
    let tm = xh - 1.0;
    let hp = hy;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_max_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let my = p;
    let by = q;
    let x = mz;
    let y = bz;
    let tm = xh + 1.0;
    let hp = hz;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_max_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let mz = p;
    let bz = q;

    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let _q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let x = sr;
    let y = 0.0;
    let tm = ut;
    let hp = 0.00004263452 / rr;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_max_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let sr = p;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let _ru = (hd - rn + ps) * 1.02;
    let _rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rn;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let _z6 = z1 - zd;
    let _z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    /*
    if z6 < 0.0 {
        z6 = z6 + 24.0;
    }
    */

    let _mg = (rm + rn - pj) / (2.0 * rn);

    return z1;
}

/// Helper function for ut_max_solar_eclipse
pub fn ut_max_solar_eclipse_l7390(
    x: f64,
    y: f64,
    igday: f64,
    gmonth: u32,
    gyear: u32,
    tm: f64,
    glong: f64,
    glat: f64,
    hp: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let paa = ec_ra(
        degrees(x),
        0.0,
        0.0,
        degrees(y),
        0.0,
        0.0,
        igday,
        gmonth,
        gyear,
    );
    let qaa = ec_dec(
        degrees(x),
        0.0,
        0.0,
        degrees(y),
        0.0,
        0.0,
        igday,
        gmonth,
        gyear,
    );
    let xaa = ra_ha(
        dd_dh(paa),
        0.0,
        0.0,
        tm,
        0.0,
        0.0,
        0,
        0,
        igday,
        gmonth,
        gyear,
        glong,
    );
    let pbb = parallax_ha(
        xaa,
        0.0,
        0.0,
        qaa,
        0.0,
        0.0,
        "true".to_string(),
        glat,
        0.0,
        degrees(hp),
    );
    let qbb = parallax_dec(
        xaa,
        0.0,
        0.0,
        qaa,
        0.0,
        0.0,
        "true".to_string(),
        glat,
        0.0,
        degrees(hp),
    );
    let xbb = ha_ra(
        pbb, 0.0, 0.0, tm, 0.0, 0.0, 0, 0, igday, gmonth, gyear, glong,
    );
    let p = (eq_e_long(xbb, 0.0, 0.0, qbb, 0.0, 0.0, igday, gmonth, gyear)).to_radians();
    let q = (eq_e_lat(xbb, 0.0, 0.0, qbb, 0.0, 0.0, igday, gmonth, gyear)).to_radians();

    return (paa, qaa, xaa, pbb, qbb, xbb, p, q);
}

/// Calculate time of first contact for solar eclipse (UT)
///
/// Original macro name: UTFirstContactSolarEclipse
pub fn ut_first_contact_solar_eclipse(
    dy: f64,
    mn: u32,
    yr: u32,
    ds: i32,
    zc: i32,
    glong: f64,
    glat: f64,
) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if solar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No solar eclipse" {
        return -99.0;
    }

    let dj = new_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utnm = xi * 24.0;
    let ut = utnm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utnm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utnm;
    let x = my;
    let y = by;
    let tm = xh - 1.0;
    let hp = hy;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_first_contact_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let my = p;
    let by = q;
    let x = mz;
    let y = bz;
    let tm = xh + 1.0;
    let hp = hz;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_first_contact_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let mz = p;
    let bz = q;

    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let _q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let x = sr;
    let y = 0.0;
    let tm = ut;
    let hp = 0.00004263452 / rr;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_first_contact_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let sr = p;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let _ru = (hd - rn + ps) * 1.02;
    let _rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rn;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let mut z6 = z1 - zd;
    let _z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    if z6 < 0.0 {
        z6 = z6 + 24.0;
    }

    let _mg = (rm + rn - pj) / (2.0 * rn);

    return z6;
}

/// Helper function for ut_first_contact_solar_eclipse
pub fn ut_first_contact_solar_eclipse_l7390(
    x: f64,
    y: f64,
    igday: f64,
    gmonth: u32,
    gyear: u32,
    tm: f64,
    glong: f64,
    glat: f64,
    hp: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let paa = ec_ra(
        degrees(x),
        0.0,
        0.0,
        degrees(y),
        0.0,
        0.0,
        igday,
        gmonth,
        gyear,
    );
    let qaa = ec_dec(
        degrees(x),
        0.0,
        0.0,
        degrees(y),
        0.0,
        0.0,
        igday,
        gmonth,
        gyear,
    );
    let xaa = ra_ha(
        dd_dh(paa),
        0.0,
        0.0,
        tm,
        0.0,
        0.0,
        0,
        0,
        igday,
        gmonth,
        gyear,
        glong,
    );
    let pbb = parallax_ha(
        xaa,
        0.0,
        0.0,
        qaa,
        0.0,
        0.0,
        "true".to_string(),
        glat,
        0.0,
        degrees(hp),
    );
    let qbb = parallax_dec(
        xaa,
        0.0,
        0.0,
        qaa,
        0.0,
        0.0,
        "true".to_string(),
        glat,
        0.0,
        degrees(hp),
    );
    let xbb = ha_ra(
        pbb, 0.0, 0.0, tm, 0.0, 0.0, 0, 0, igday, gmonth, gyear, glong,
    );
    let p = (eq_e_long(xbb, 0.0, 0.0, qbb, 0.0, 0.0, igday, gmonth, gyear)).to_radians();
    let q = (eq_e_lat(xbb, 0.0, 0.0, qbb, 0.0, 0.0, igday, gmonth, gyear)).to_radians();

    return (paa, qaa, xaa, pbb, qbb, xbb, p, q);
}

/// Calculate time of last contact for solar eclipse (UT)
///
/// Original macro name: UTLastContactSolarEclipse
pub fn ut_last_contact_solar_eclipse(
    dy: f64,
    mn: u32,
    yr: u32,
    ds: i32,
    zc: i32,
    glong: f64,
    glat: f64,
) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if solar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No solar eclipse" {
        return -99.0;
    }

    let dj = new_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utnm = xi * 24.0;
    let ut = utnm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utnm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utnm;
    let x = my;
    let y = by;
    let tm = xh - 1.0;
    let hp = hy;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_last_contact_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let my = p;
    let by = q;
    let x = mz;
    let y = bz;
    let tm = xh + 1.0;
    let hp = hz;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_last_contact_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let mz = p;
    let bz = q;

    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let _q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let x = sr;
    let y = 0.0;
    let tm = ut;
    let hp = 0.00004263452 / rr;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        ut_last_contact_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let sr = p;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let _ru = (hd - rn + ps) * 1.02;
    let _rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rn;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let _z6 = z1 - zd;
    let z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    /*
    if z6 < 0.0 {
        z6 = z6 + 24.0;
    }
    */

    let _mg = (rm + rn - pj) / (2.0 * rn);

    return z7;
}

/// Helper function for ut_last_contact_solar_eclipse
pub fn ut_last_contact_solar_eclipse_l7390(
    x: f64,
    y: f64,
    igday: f64,
    gmonth: u32,
    gyear: u32,
    tm: f64,
    glong: f64,
    glat: f64,
    hp: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let paa = ec_ra(
        degrees(x),
        0.0,
        0.0,
        degrees(y),
        0.0,
        0.0,
        igday,
        gmonth,
        gyear,
    );
    let qaa = ec_dec(
        degrees(x),
        0.0,
        0.0,
        degrees(y),
        0.0,
        0.0,
        igday,
        gmonth,
        gyear,
    );
    let xaa = ra_ha(
        dd_dh(paa),
        0.0,
        0.0,
        tm,
        0.0,
        0.0,
        0,
        0,
        igday,
        gmonth,
        gyear,
        glong,
    );
    let pbb = parallax_ha(
        xaa,
        0.0,
        0.0,
        qaa,
        0.0,
        0.0,
        "true".to_string(),
        glat,
        0.0,
        degrees(hp),
    );
    let qbb = parallax_dec(
        xaa,
        0.0,
        0.0,
        qaa,
        0.0,
        0.0,
        "true".to_string(),
        glat,
        0.0,
        degrees(hp),
    );
    let xbb = ha_ra(
        pbb, 0.0, 0.0, tm, 0.0, 0.0, 0, 0, igday, gmonth, gyear, glong,
    );
    let p = (eq_e_long(xbb, 0.0, 0.0, qbb, 0.0, 0.0, igday, gmonth, gyear)).to_radians();
    let q = (eq_e_lat(xbb, 0.0, 0.0, qbb, 0.0, 0.0, igday, gmonth, gyear)).to_radians();

    return (paa, qaa, xaa, pbb, qbb, xbb, p, q);
}

/// Calculate magnitude of solar eclipse.
///
/// Original macro name: MagSolarEclipse
pub fn mag_solar_eclipse(
    dy: f64,
    mn: u32,
    yr: u32,
    ds: i32,
    zc: i32,
    glong: f64,
    glat: f64,
) -> f64 {
    let tp = 2.0 * std::f64::consts::PI;

    if solar_eclipse_occurrence(ds, zc, dy, mn, yr) == "No solar eclipse" {
        return -99.0;
    }

    let dj = new_moon(ds, zc, dy, mn, yr);
    let _dp = 0.0;
    let gday = jdc_day(dj);
    let gmonth = jdc_month(dj);
    let gyear = jdc_year(dj);
    let igday = gday.floor();
    let xi = gday - igday;
    let utnm = xi * 24.0;
    let ut = utnm - 1.0;
    let ly = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let my = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let by = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hy = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let ut = utnm + 1.0;
    let mut sb = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians() - ly;
    let mz = (moon_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let bz = (moon_lat(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let hz = (moon_hp(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();

    if sb < 0.0 {
        sb = sb + tp;
    }

    let xh = utnm;
    let x = my;
    let y = by;
    let tm = xh - 1.0;
    let hp = hy;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        mag_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let my = p;
    let by = q;
    let x = mz;
    let y = bz;
    let tm = xh + 1.0;
    let hp = hz;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        mag_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let mz = p;
    let bz = q;

    let x0 = xh + 1.0 - (2.0 * bz / (bz - by));
    let mut dm = mz - my;

    if dm < 0.0 {
        dm = dm + tp;
    }

    let lj = (dm - sb) / 2.0;
    let _q = 0.0;
    let mr = my + (dm * (x0 - xh + 1.0) / 2.0);
    let ut = x0 - 0.13851852;
    let rr = sun_dist(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear);
    let sr = (sun_long(ut, 0.0, 0.0, 0, 0, igday, gmonth, gyear)).to_radians();
    let sr = sr + (nutat_long(igday, gmonth, gyear) - 0.00569).to_radians();
    let x = sr;
    let y = 0.0;
    let tm = ut;
    let hp = 0.00004263452 / rr;
    let (_paa, _qaa, _xaa, _pbb, _qbb, _xbb, p, q) =
        mag_solar_eclipse_l7390(x, y, igday, gmonth, gyear, tm, glong, glat, hp);
    let sr = p;
    let by = by - q;
    let bz = bz - q;
    let p3 = 0.00004263;
    let zh = (sr - mr) / lj;
    let tc = x0 + zh;
    let sh = (((bz - by) * (tc - xh - 1.0) / 2.0) + bz) / lj;
    let s2 = sh * sh;
    let z2 = zh * zh;
    let ps = p3 / (rr * lj);
    let z1 = (zh * z2 / (z2 + s2)) + x0;
    let h0 = (hy + hz) / (2.0 * lj);
    let rm = 0.272446 * h0;
    let rn = 0.00465242 / (lj * rr);
    let hd = h0 * 0.99834;
    let _ru = (hd - rn + ps) * 1.02;
    let _rp = (hd + rn + ps) * 1.02;
    let pj = (sh * zh / (s2 + z2).sqrt()).abs();
    let r = rm + rn;
    let dd = z1 - x0;
    let dd = dd * dd - ((z2 - (r * r)) * dd / zh);

    if dd < 0.0 {
        return -99.0;
    }

    let zd = dd.sqrt();
    let _z6 = z1 - zd;
    let _z7 = z1 + zd - lint((z1 + zd) / 24.0) * 24.0;

    /*
    if z6 < 0.0 {
        z6 = z6 + 24.0;
    }
    */

    let mg = (rm + rn - pj) / (2.0 * rn);

    return mg;
}

/// Helper function for mag_solar_eclipse
pub fn mag_solar_eclipse_l7390(
    x: f64,
    y: f64,
    igday: f64,
    gmonth: u32,
    gyear: u32,
    tm: f64,
    glong: f64,
    glat: f64,
    hp: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let paa = ec_ra(
        degrees(x),
        0.0,
        0.0,
        degrees(y),
        0.0,
        0.0,
        igday,
        gmonth,
        gyear,
    );
    let qaa = ec_dec(
        degrees(x),
        0.0,
        0.0,
        degrees(y),
        0.0,
        0.0,
        igday,
        gmonth,
        gyear,
    );
    let xaa = ra_ha(
        dd_dh(paa),
        0.0,
        0.0,
        tm,
        0.0,
        0.0,
        0,
        0,
        igday,
        gmonth,
        gyear,
        glong,
    );
    let pbb = parallax_ha(
        xaa,
        0.0,
        0.0,
        qaa,
        0.0,
        0.0,
        "true".to_string(),
        glat,
        0.0,
        degrees(hp),
    );
    let qbb = parallax_dec(
        xaa,
        0.0,
        0.0,
        qaa,
        0.0,
        0.0,
        "true".to_string(),
        glat,
        0.0,
        degrees(hp),
    );
    let xbb = ha_ra(
        pbb, 0.0, 0.0, tm, 0.0, 0.0, 0, 0, igday, gmonth, gyear, glong,
    );
    let p = (eq_e_long(xbb, 0.0, 0.0, qbb, 0.0, 0.0, igday, gmonth, gyear)).to_radians();
    let q = (eq_e_lat(xbb, 0.0, 0.0, qbb, 0.0, 0.0, igday, gmonth, gyear)).to_radians();

    return (paa, qaa, xaa, pbb, qbb, xbb, p, q);
}
