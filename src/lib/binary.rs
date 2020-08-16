use crate::lib::binarydata;
use crate::lib::macros;
use crate::lib::util;

/// Calculate orbital data for binary star.
///
/// ## Arguments
/// * `greenwich_date_day` -- Greenwich date (day)
/// * `greenwich_date_month` -- Greenwich date (month)
/// * `greenwich_date_year` -- Greenwich date (year)
/// * `binary_name` -- Abbreviated name of binary
///
/// ## Returns
/// * `position_angle_deg` -- Position angle (degrees)
/// * `separation_arcsec` -- Separation of binary members (arcseconds)
pub fn binary_star_orbit(
    greenwich_date_day: f64,
    greenwich_date_month: u32,
    greenwich_date_year: u32,
    binary_name: String,
) -> (f64, f64) {
    let (binary_info, _binary_info_status) = binarydata::get_binary_info_vector(binary_name);

    let y_years = (greenwich_date_year as f64
        + (macros::cd_jd(
            greenwich_date_day,
            greenwich_date_month,
            greenwich_date_year,
        ) - macros::cd_jd(0.0, 1, greenwich_date_year))
            / 365.242191)
        - binary_info.epoch_peri;
    let m_deg = 360.0 * y_years / binary_info.period;
    let m_rad = (m_deg - 360.0 * (m_deg / 360.0).floor()).to_radians();
    let eccentricity = binary_info.ecc;
    let true_anomaly_rad = macros::true_anomaly(m_rad, eccentricity);
    let r_arcsec = (1.0 - eccentricity * (macros::eccentric_anomaly(m_rad, eccentricity)).cos())
        * binary_info.axis;
    let ta_peri_rad = true_anomaly_rad + binary_info.long_peri.to_radians();

    let y = (ta_peri_rad).sin() * ((binary_info.incl).to_radians()).cos();
    let x = (ta_peri_rad).cos();
    let a_deg = macros::degrees(y.atan2(x));
    let theta_deg1 = a_deg + binary_info.pa_node;
    let theta_deg2 = theta_deg1 - 360.0 * (theta_deg1 / 360.0).floor();
    let rho_arcsec =
        r_arcsec * (ta_peri_rad).cos() / ((theta_deg2 - binary_info.pa_node).to_radians()).cos();

    let position_angle_deg = util::round_f64(theta_deg2, 1);
    let separation_arcsec = util::round_f64(rho_arcsec, 2);

    return (position_angle_deg, separation_arcsec);
}
