use crate::lib::cometdata;
use crate::lib::macros;
use crate::lib::util;

/// Calculate position of an elliptical comet.
///
/// ## Arguments
/// * `lct_hour` -- Local civil time, hour part.
/// * `lct_min` -- Local civil time, minutes part.
/// * `lct_sec` -- Local civil time, seconds part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
/// * `comet_name` -- Name of comet, e.g., "Halley"
///
/// ## Returns
/// * `comet_ra_hour` -- Right ascension of comet (hour part)
/// * `comet_ra_min` -- Right ascension of comet (minutes part)
/// * `comet_dec_deg` -- Declination of comet (degrees part)
/// * `comet_dec_min` -- Declination of comet (minutes part)
/// * `comet_dist_earth` -- Comet's distance from Earth (AU)
pub fn position_of_elliptical_comet(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    comet_name: String,
) -> (f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let greenwich_date_day = macros::lct_gday(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let greenwich_date_month = macros::lct_gmonth(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let greenwich_date_year = macros::lct_gyear(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let (comet_info, _comet_info_status) = cometdata::get_comet_info_elliptical_vector(comet_name);

    let time_since_epoch_years = (macros::cd_jd(
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    ) - macros::cd_jd(0.0, 1, greenwich_date_year))
        / 365.242191
        + greenwich_date_year as f64
        - comet_info.epoch;
    let mc_deg = 360.0 * time_since_epoch_years / comet_info.period;
    let mc_rad = (mc_deg - 360.0 * (mc_deg / 360.0).floor()).to_radians();
    let eccentricity = comet_info.ecc;
    let true_anomaly_deg = macros::degrees(macros::true_anomaly(mc_rad, eccentricity));
    let lc_deg = true_anomaly_deg + comet_info.peri;
    let r_au = comet_info.axis * (1.0 - eccentricity * eccentricity)
        / (1.0 + eccentricity * ((true_anomaly_deg).to_radians()).cos());
    let lc_node_rad = (lc_deg - comet_info.node).to_radians();
    let psi_rad = ((lc_node_rad).sin() * ((comet_info.incl).to_radians()).sin()).asin();

    let y = (lc_node_rad).sin() * ((comet_info.incl).to_radians()).cos();
    let x = (lc_node_rad).cos();

    let ld_deg = macros::degrees(y.atan2(x)) + comet_info.node;
    let rd_au = r_au * (psi_rad).cos();

    let earth_longitude_le_deg = macros::sun_long(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    ) + 180.0;
    let earth_radius_vector_au = macros::sun_dist(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let le_ld_rad = (earth_longitude_le_deg - ld_deg).to_radians();
    let a_rad = if rd_au < earth_radius_vector_au {
        (rd_au * (le_ld_rad).sin()).atan2(earth_radius_vector_au - rd_au * (le_ld_rad).cos())
    } else {
        (earth_radius_vector_au * (-le_ld_rad).sin())
            .atan2(rd_au - earth_radius_vector_au * (le_ld_rad).cos())
    };

    let comet_long_deg1 = if rd_au < earth_radius_vector_au {
        180.0 + earth_longitude_le_deg + macros::degrees(a_rad)
    } else {
        macros::degrees(a_rad) + ld_deg
    };
    let comet_long_deg = comet_long_deg1 - 360.0 * (comet_long_deg1 / 360.0).floor();
    let comet_lat_deg = macros::degrees(
        (rd_au * (psi_rad).tan() * ((comet_long_deg1 - ld_deg).to_radians()).sin()
            / (earth_radius_vector_au * (-le_ld_rad).sin()))
        .atan(),
    );
    let comet_ra_hours1 = macros::dd_dh(macros::ec_ra(
        comet_long_deg,
        0.0,
        0.0,
        comet_lat_deg,
        0.0,
        0.0,
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    ));
    let comet_dec_deg1 = macros::ec_dec(
        comet_long_deg,
        0.0,
        0.0,
        comet_lat_deg,
        0.0,
        0.0,
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    );
    let comet_distance_au = (num::pow(earth_radius_vector_au, 2) + num::pow(r_au, 2)
        - 2.0
            * earth_radius_vector_au
            * r_au
            * ((lc_deg - earth_longitude_le_deg).to_radians()).cos()
            * (psi_rad).cos())
    .sqrt();

    let comet_ra_hour = macros::dh_hour(comet_ra_hours1 + 0.008333);
    let comet_ra_min = macros::dh_min(comet_ra_hours1 + 0.008333);
    let comet_dec_deg = macros::dd_deg(comet_dec_deg1 + 0.008333);
    let comet_dec_min = macros::dd_min(comet_dec_deg1 + 0.008333);
    let comet_dist_earth = util::round_f64(comet_distance_au, 2);

    return (
        comet_ra_hour as f64,
        comet_ra_min as f64,
        comet_dec_deg,
        comet_dec_min,
        comet_dist_earth,
    );
}

/// Calculate position of a parabolic comet.
///
/// ## Arguments
/// * `lct_hour` -- Local civil time, hour part.
/// * `lct_min` -- Local civil time, minutes part.
/// * `lct_sec` -- Local civil time, seconds part.
/// * `is_daylight_saving` -- Is daylight savings in effect?
/// * `zone_correction_hours` -- Time zone correction, in hours.
/// * `local_date_day` -- Local date, day part.
/// * `local_date_month` -- Local date, month part.
/// * `local_date_year` -- Local date, year part.
/// * `comet_name` -- Name of comet, e.g., "Kohler"
///
/// ## Returns
/// * `comet_ra_hour` -- Right ascension of comet (hour part)
/// * `comet_ra_min` -- Right ascension of comet (minutes part)
/// * `comet_ra_sec` -- Right ascension of comet (seconds part)
/// * `comet_dec_deg` -- Declination of comet (degrees part)
/// * `comet_dec_min` -- Declination of comet (minutes part)
/// * `comet_dec_sec` -- Declination of comet (seconds part)
/// * `comet_dist_earth` -- Comet's distance from Earth (AU)
pub fn position_of_parabolic_comet(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    comet_name: String,
) -> (f64, f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let greenwich_date_day = macros::lct_gday(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let greenwich_date_month = macros::lct_gmonth(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let greenwich_date_year = macros::lct_gyear(
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

    let (comet_info, _comet_info_status) = cometdata::get_comet_info_parabolic_vector(comet_name);

    let perihelion_epoch_day = comet_info.epoch_peri_day;
    let perihelion_epoch_month = comet_info.epoch_peri_month;
    let perihelion_epoch_year = comet_info.epoch_peri_year;
    let q_au = comet_info.peri_dist;
    let inclination_deg = comet_info.incl;
    let perihelion_deg = comet_info.arg_peri;
    let node_deg = comet_info.node;

    let (comet_long_deg, comet_lat_deg, comet_dist_au) = macros::p_comet_long_lat_dist(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
        perihelion_epoch_day,
        perihelion_epoch_month,
        perihelion_epoch_year,
        q_au,
        inclination_deg,
        perihelion_deg,
        node_deg,
    );

    let comet_ra_hours = macros::dd_dh(macros::ec_ra(
        comet_long_deg,
        0.0,
        0.0,
        comet_lat_deg,
        0.0,
        0.0,
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    ));
    let comet_dec_deg1 = macros::ec_dec(
        comet_long_deg,
        0.0,
        0.0,
        comet_lat_deg,
        0.0,
        0.0,
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    );

    let comet_ra_hour = macros::dh_hour(comet_ra_hours);
    let comet_ra_min = macros::dh_min(comet_ra_hours);
    let comet_ra_sec = macros::dh_sec(comet_ra_hours);
    let comet_dec_deg = macros::dd_deg(comet_dec_deg1);
    let comet_dec_min = macros::dd_min(comet_dec_deg1);
    let comet_dec_sec = macros::dd_sec(comet_dec_deg1);
    let comet_dist_earth = util::round_f64(comet_dist_au, 2);

    return (
        comet_ra_hour as f64,
        comet_ra_min as f64,
        comet_ra_sec,
        comet_dec_deg,
        comet_dec_min,
        comet_dec_sec,
        comet_dist_earth,
    );
}
