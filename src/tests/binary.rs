use crate::lib::binary as BS;

pub fn test_binary_star_orbit(
    greenwich_date_day: f64,
    greenwich_date_month: u32,
    greenwich_date_year: u32,
    binary_name: String,
) {
    let (position_angle_deg, separation_arcsec) = BS::binary_star_orbit(
        greenwich_date_day,
        greenwich_date_month,
        greenwich_date_year,
        binary_name.to_string(),
    );

    println!(
		"Binary star orbit: [Greenwich Date] {}/{}/{} [Binary Name] {} = [Position Angle] {} degrees [Separation] {} arcsec",
		greenwich_date_month,
		greenwich_date_day,
		greenwich_date_year,
		binary_name.to_string(),
		position_angle_deg,
		separation_arcsec
	);

    assert_eq!(position_angle_deg, 318.5, "Position Angle (degrees)");
    assert_eq!(separation_arcsec, 0.41, "Separation (arcseconds)");
}
