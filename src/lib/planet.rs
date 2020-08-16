use crate::lib::macros;
use crate::lib::planetdata;
use crate::lib::util;
// use num;

/// Calculate approximate position of a planet.
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
/// * `planet_name` -- Name of planet, e.g., "Jupiter"
///
/// ## Returns
/// * `planet_ra_hour` -- Right ascension of planet (hour part)
/// * `planet_ra_min` -- Right ascension of planet (minutes part)
/// * `planet_ra_sec` -- Right ascension of planet (seconds part)
/// * `planet_dec_deg` -- Declination of planet (degrees part)
/// * `planet_dec_min` -- Declination of planet (minutes part)
/// * `planet_dec_sec` -- Declination of planet (seconds part)
pub fn approximate_position_of_planet(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    planet_name: String,
) -> (f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let (planet_info, _planet_info_status) = planetdata::get_planet_info_vector(planet_name);

    let planet_tp_from_table = planet_info.tp;
    let planet_long_from_table = planet_info.long;
    let planet_peri_from_table = planet_info.peri;
    let planet_ecc_from_table = planet_info.ecc;
    let planet_axis_from_table = planet_info.axis;
    let planet_incl_from_table = planet_info.incl;
    let planet_node_from_table = planet_info.node;

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
    let d_days = macros::cd_jd(gdate_day + (ut_hours / 24.0), gdate_month, gdate_year)
        - macros::cd_jd(0.0, 1, 2010);
    let np_deg1 = 360.0 * d_days / (365.242191 * planet_tp_from_table);
    let np_deg2 = np_deg1 - 360.0 * (np_deg1 / 360.0).floor();
    let mp_deg = np_deg2 + planet_long_from_table - planet_peri_from_table;
    let lp_deg1 = np_deg2
        + (360.0 * planet_ecc_from_table * mp_deg.to_radians().sin() / std::f64::consts::PI)
        + planet_long_from_table;
    let lp_deg2 = lp_deg1 - 360.0 * (lp_deg1 / 360.0).floor();
    let planet_true_anomaly_deg = lp_deg2 - planet_peri_from_table;
    let r_au = planet_axis_from_table * (1.0 - num::pow(planet_ecc_from_table, 2))
        / (1.0 + planet_ecc_from_table * planet_true_anomaly_deg.to_radians().cos());

    let (earth_info, _earth_info_status) = planetdata::get_planet_info_vector("Earth".to_string());

    let earth_tp_from_table = earth_info.tp;
    let earth_long_from_table = earth_info.long;
    let earth_peri_from_table = earth_info.peri;
    let earth_ecc_from_table = earth_info.ecc;
    let earth_axis_from_table = earth_info.axis;

    let ne_deg1 = 360.0 * d_days / (365.242191 * earth_tp_from_table);
    let ne_deg2 = ne_deg1 - 360.0 * (ne_deg1 / 360.0).floor();
    let me_deg = ne_deg2 + earth_long_from_table - earth_peri_from_table;
    let le_deg1 = ne_deg2
        + earth_long_from_table
        + 360.0 * earth_ecc_from_table * me_deg.to_radians().sin() / std::f64::consts::PI;
    let le_deg2 = le_deg1 - 360.0 * (le_deg1 / 360.0).floor();
    let earth_true_anomaly_deg = le_deg2 - earth_peri_from_table;
    let r_au2 = earth_axis_from_table * (1.0 - num::pow(earth_ecc_from_table, 2))
        / (1.0 + earth_ecc_from_table * earth_true_anomaly_deg.to_radians().cos());
    let lp_node_rad = (lp_deg2 - planet_node_from_table).to_radians();
    let psi_rad = ((lp_node_rad).sin() * planet_incl_from_table.to_radians().sin()).asin();
    let y = lp_node_rad.sin() * planet_incl_from_table.to_radians().cos();
    let x = lp_node_rad.cos();
    let ld_deg = macros::degrees(y.atan2(x)) + planet_node_from_table;
    let rd_au = r_au * psi_rad.cos();
    let le_ld_rad = (le_deg2 - ld_deg).to_radians();
    let atan2_type_1 = (rd_au * le_ld_rad.sin()).atan2(r_au2 - rd_au * le_ld_rad.cos());
    let atan2_type_2 = (r_au2 * (-le_ld_rad).sin()).atan2(rd_au - r_au2 * le_ld_rad.cos());
    let a_rad = if rd_au < 1.0 {
        atan2_type_1
    } else {
        atan2_type_2
    };
    let lamda_deg1 = if rd_au < 1.0 {
        180.0 + le_deg2 + macros::degrees(a_rad)
    } else {
        macros::degrees(a_rad) + ld_deg
    };
    let lamda_deg2 = lamda_deg1 - 360.0 * (lamda_deg1 / 360.0).floor();
    let beta_deg = macros::degrees(
        (rd_au * psi_rad.tan() * ((lamda_deg2 - ld_deg).to_radians()).sin()
            / (r_au2 * (-le_ld_rad).sin()))
        .atan(),
    );
    let ra_hours = macros::dd_dh(macros::ec_ra(
        lamda_deg2,
        0.0,
        0.0,
        beta_deg,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    ));
    let dec_deg = macros::ec_dec(
        lamda_deg2,
        0.0,
        0.0,
        beta_deg,
        0.0,
        0.0,
        gdate_day,
        gdate_month,
        gdate_year,
    );

    let planet_ra_hour = macros::dh_hour(ra_hours);
    let planet_ra_min = macros::dh_min(ra_hours);
    let planet_ra_sec = macros::dh_sec(ra_hours);
    let planet_dec_deg = macros::dd_deg(dec_deg);
    let planet_dec_min = macros::dd_min(dec_deg);
    let planet_dec_sec = macros::dd_sec(dec_deg);

    return (
        planet_ra_hour as f64,
        planet_ra_min as f64,
        planet_ra_sec,
        planet_dec_deg,
        planet_dec_min,
        planet_dec_sec,
    );
}

/// Calculate precise position of a planet.
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
/// * `planet_name` -- Name of planet, e.g., "Jupiter"
///
/// ## Returns
/// * `planet_ra_hour` -- Right ascension of planet (hour part)
/// * `planet_ra_min` -- Right ascension of planet (minutes part)
/// * `planet_ra_sec` -- Right ascension of planet (seconds part)
/// * `planet_dec_deg` -- Declination of planet (degrees part)
/// * `planet_dec_min` -- Declination of planet (minutes part)
/// * `planet_dec_sec` -- Declination of planet (seconds part)
pub fn precise_position_of_planet(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    planet_name: String,
) -> (f64, f64, f64, f64, f64, f64) {
    let daylight_saving = if is_daylight_saving == true { 1 } else { 0 };

    let _gdate_day = macros::lct_gday(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let _gdate_month = macros::lct_gmonth(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let _gdate_year = macros::lct_gyear(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let (
        planet_ecl_long_deg,
        planet_ecl_lat_deg,
        _planet_distance_au,
        _planet_h_long1,
        _planet_h_long2,
        _planet_h_lat,
        _planet_r_vect,
    ) = macros::planet_coordinates(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
        planet_name,
    );

    let planet_ra_hours = macros::dd_dh(macros::ec_ra(
        planet_ecl_long_deg,
        0.0,
        0.0,
        planet_ecl_lat_deg,
        0.0,
        0.0,
        local_date_day,
        local_date_month,
        local_date_year,
    ));
    let planet_dec_deg1 = macros::ec_dec(
        planet_ecl_long_deg,
        0.0,
        0.0,
        planet_ecl_lat_deg,
        0.0,
        0.0,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    let planet_ra_hour = macros::dh_hour(planet_ra_hours);
    let planet_ra_min = macros::dh_min(planet_ra_hours);
    let planet_ra_sec = macros::dh_sec(planet_ra_hours);
    let planet_dec_deg = macros::dd_deg(planet_dec_deg1);
    let planet_dec_min = macros::dd_min(planet_dec_deg1);
    let planet_dec_sec = macros::dd_sec(planet_dec_deg1);

    return (
        planet_ra_hour as f64,
        planet_ra_min as f64,
        planet_ra_sec,
        planet_dec_deg,
        planet_dec_min,
        planet_dec_sec,
    );
}

/// Calculate several visual aspects of a planet.
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
/// * `planet_name` -- Name of planet, e.g., "Jupiter"
///
/// ## Returns
/// * `distance_au` -- Planet's distance from Earth, in AU.
/// * `ang_dia_arcsec` -- Angular diameter of the planet.
/// * `phase` -- Illuminated fraction of the planet.
/// * `light_time_hour` -- Light travel time from planet to Earth, hour part.
/// * `light_time_minutes` -- Light travel time from planet to Earth, minutes part.
/// * `light_time_seconds` -- Light travel time from planet to Earth, seconds part.
/// * `pos_angle_bright_limb_deg` -- Position-angle of the bright limb.
/// * `approximate_magnitude` -- Apparent brightness of the planet.
pub fn visual_aspects_of_a_planet(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    planet_name: String,
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
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

    let (
        planet_ecl_long_deg,
        planet_ecl_lat_deg,
        planet_dist_au,
        planet_h_long1,
        _temp3,
        _temp4,
        planet_r_vect,
    ) = macros::planet_coordinates(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
        planet_name.to_string(),
    );

    let planet_ra_rad = (macros::ec_ra(
        planet_ecl_long_deg,
        0.0,
        0.0,
        planet_ecl_lat_deg,
        0.0,
        0.0,
        local_date_day,
        local_date_month,
        local_date_year,
    ))
    .to_radians();
    let planet_dec_rad = (macros::ec_dec(
        planet_ecl_long_deg,
        0.0,
        0.0,
        planet_ecl_lat_deg,
        0.0,
        0.0,
        local_date_day,
        local_date_month,
        local_date_year,
    ))
    .to_radians();

    let light_travel_time_hours = planet_dist_au * 0.1386;
    let (planet_info, _planet_info_status) =
        planetdata::get_planet_info_vector(planet_name.to_string());
    let angular_diameter_arcsec = planet_info.theta0 / planet_dist_au;
    let phase1 = 0.5 * (1.0 + ((planet_ecl_long_deg - planet_h_long1).to_radians()).cos());

    let sun_ecl_long_deg = macros::sun_long(
        lct_hour,
        lct_min,
        lct_sec,
        daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );
    let sun_ra_rad = (macros::ec_ra(
        sun_ecl_long_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    ))
    .to_radians();
    let sun_dec_rad = (macros::ec_dec(
        sun_ecl_long_deg,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
    ))
    .to_radians();

    let y = (sun_dec_rad).cos() * (sun_ra_rad - planet_ra_rad).sin();
    let x = (planet_dec_rad).cos() * (sun_dec_rad).sin()
        - (planet_dec_rad).sin() * (sun_dec_rad).cos() * (sun_ra_rad - planet_ra_rad).cos();

    let chi_deg = macros::degrees(y.atan2(x));
    let radius_vector_au = planet_r_vect;
    let approximate_magnitude1 =
        5.0 * (radius_vector_au * planet_dist_au / (phase1).sqrt()).log10() + planet_info.v0;

    let distance_au = util::round_f64(planet_dist_au, 5);
    let ang_dia_arcsec = util::round_f64(angular_diameter_arcsec, 1);
    let phase = util::round_f64(phase1, 2);
    let light_time_hour = macros::dh_hour(light_travel_time_hours);
    let light_time_minutes = macros::dh_min(light_travel_time_hours);
    let light_time_seconds = macros::dh_sec(light_travel_time_hours);
    let pos_angle_bright_limb_deg = util::round_f64(chi_deg, 1);
    let approximate_magnitude = util::round_f64(approximate_magnitude1, 1);

    return (
        distance_au,
        ang_dia_arcsec,
        phase,
        light_time_hour as f64,
        light_time_minutes as f64,
        light_time_seconds,
        pos_angle_bright_limb_deg,
        approximate_magnitude,
    );
}
