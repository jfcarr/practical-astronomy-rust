use crate::lib::comet as CO;

pub fn test_position_of_elliptical_comet(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    comet_name: String,
) {
    let (comet_ra_hour, comet_ra_min, comet_dec_deg, comet_dec_min, comet_dist_earth) =
        CO::position_of_elliptical_comet(
            lct_hour,
            lct_min,
            lct_sec,
            is_daylight_saving,
            zone_correction_hours,
            local_date_day,
            local_date_month,
            local_date_year,
            comet_name.to_string(),
        );

    println!(
		"Position of elliptical comet: [Local Civil Time] {}:{}:{} [DST?] {} [Zone Correction] {} hours [Local Date] {}/{}/{} [Comet Name] {} = [Right Ascension] {} hours {} minutes [Declination] {} degrees {} minutes [Distance] {} AU",
		lct_hour,
		lct_min,
		lct_sec,
		is_daylight_saving,
		zone_correction_hours,
		local_date_month,
		local_date_day,
		local_date_year,
		comet_name.to_string(),
		comet_ra_hour,
		comet_ra_min,
		comet_dec_deg,
		comet_dec_min,
		comet_dist_earth
	);

    assert_eq!(comet_ra_hour, 6.0, "Comet RA - hour");
    assert_eq!(comet_ra_min, 29.0, "Comet RA - minutes");
    assert_eq!(comet_dec_deg, 10.0, "Comet Declination - degrees");
    assert_eq!(comet_dec_min, 13.0, "Comet Declination - minutes");
    assert_eq!(comet_dist_earth, 8.13, "Comet Distance from Earth - AU");
}

pub fn test_position_of_parabolic_comet(
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    comet_name: String,
) {
    let (
        comet_ra_hour,
        comet_ra_min,
        comet_ra_sec,
        comet_dec_deg,
        comet_dec_min,
        comet_dec_sec,
        comet_dist_earth,
    ) = CO::position_of_parabolic_comet(
        lct_hour,
        lct_min,
        lct_sec,
        is_daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
        comet_name.to_string(),
    );

    println!(
		"Position of parabolic comet: [Local Civil Time] {}:{}:{} [DST?] {} [Zone Correction] {} hours [Local Date] {}/{}/{} [Comet Name] {} = [Right Ascension] {} hours {} minutes {} seconds [Declination] {} degrees {} minutes {} seconds [Distance] {} AU",
		lct_hour,
		lct_min,
		lct_sec,
		is_daylight_saving,
		zone_correction_hours,
		local_date_month,
		local_date_day,
		local_date_year,
		comet_name.to_string(),
		comet_ra_hour,
		comet_ra_min,
		comet_ra_sec,
		comet_dec_deg,
		comet_dec_min,
		comet_dec_sec,
		comet_dist_earth
	);

    assert_eq!(comet_ra_hour, 23.0, "Comet RA - hour");
    assert_eq!(comet_ra_min, 17.0, "Comet RA - minutes");
    assert_eq!(comet_ra_sec, 11.53, "Comet RA - seconds");
    assert_eq!(comet_dec_deg, -33.0, "Comet Declination - degrees");
    assert_eq!(comet_dec_min, 42.0, "Comet Declination - minutes");
    assert_eq!(comet_dec_sec, 26.42, "Comet Declination - seconds");
    assert_eq!(comet_dist_earth, 1.11, "Comet Distance from Earth - AU");
}
