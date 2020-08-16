use crate::lib::moon as M;

pub struct TestMoonPositionInfoScaffold {
    pub lct_hour: f64,
    pub lct_min: f64,
    pub lct_sec: f64,
    pub is_daylight_saving: bool,
    pub zone_correction_hours: i32,
    pub local_date_day: f64,
    pub local_date_month: u32,
    pub local_date_year: u32,
}
impl TestMoonPositionInfoScaffold {
    pub fn test_approximate_position_of_moon(&mut self) {
        let (moon_ra_hour, moon_ra_min, moon_ra_sec, moon_dec_deg, moon_dec_min, moon_dec_sec) =
            M::approximate_position_of_moon(
                self.lct_hour,
                self.lct_min,
                self.lct_sec,
                self.is_daylight_saving,
                self.zone_correction_hours,
                self.local_date_day,
                self.local_date_month,
                self.local_date_year,
            );

        println!(
			"Approximate position of moon: [Local Time] {}:{}:{} [DST?] {} [Zone Correction] {} [Local Date] {}/{}/{} = [Moon] [Right Ascension] {} hour {} minutes {} seconds [Declination] {} degrees {} minutes {} seconds",
			self.lct_hour,
			self.lct_min,
			self.lct_sec,
			self.is_daylight_saving,
			self.zone_correction_hours,
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			moon_ra_hour,
			moon_ra_min,
			moon_ra_sec,
			moon_dec_deg,
			moon_dec_min,
			moon_dec_sec
		);

        assert_eq!(moon_ra_hour, 14.0, "Moon RA (hour)");
        assert_eq!(moon_ra_min, 12.0, "Moon RA (minutes)");
        assert_eq!(moon_ra_sec, 42.31, "Moon RA (seconds)");
        assert_eq!(moon_dec_deg, -11.0, "Moon Declination (degrees)");
        assert_eq!(moon_dec_min, 31.0, "Moon Declination (minutes)");
        assert_eq!(moon_dec_sec, 38.27, "Moon Declination (seconds)");
    }

    pub fn test_precise_position_of_moon(&mut self) {
        let (
            moon_ra_hour,
            moon_ra_min,
            moon_ra_sec,
            moon_dec_deg,
            moon_dec_min,
            moon_dec_sec,
            earth_moon_dist_km,
            moon_hor_parallax_deg,
        ) = M::precise_position_of_moon(
            self.lct_hour,
            self.lct_min,
            self.lct_sec,
            self.is_daylight_saving,
            self.zone_correction_hours,
            self.local_date_day,
            self.local_date_month,
            self.local_date_year,
        );

        println!(
			"Precise position of moon: [Local Time] {}:{}:{} [DST?] {} [Zone Correction] {} [Local Date] {}/{}/{} = [Moon] [Right Ascension] {} hour {} minutes {} seconds [Declination] {} degrees {} minutes {} seconds [Distance] {} km [Horizontal Parallax] {} degrees",
			self.lct_hour,
			self.lct_min,
			self.lct_sec,
			self.is_daylight_saving,
			self.zone_correction_hours,
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			moon_ra_hour,
			moon_ra_min,
			moon_ra_sec,
			moon_dec_deg,
			moon_dec_min,
			moon_dec_sec,
			earth_moon_dist_km,
			moon_hor_parallax_deg
		);

        assert_eq!(moon_ra_hour, 14.0, "Moon RA (hour)");
        assert_eq!(moon_ra_min, 12.0, "Moon RA (minutes)");
        assert_eq!(moon_ra_sec, 10.21, "Moon RA (seconds)");
        assert_eq!(moon_dec_deg, -11.0, "Moon Declination (degrees)");
        assert_eq!(moon_dec_min, 34.0, "Moon Declination (minutes)");
        assert_eq!(moon_dec_sec, 57.83, "Moon Declination (seconds)");
        assert_eq!(earth_moon_dist_km, 367964.0, "Earth-Moon Distance (km)");
        assert_eq!(
            moon_hor_parallax_deg, 0.993191,
            "Moon Horizontal Parallax (degrees)"
        );
    }

    pub fn test_moon_phase(&mut self) {
        let (moon_phase, pa_bright_limb_deg) = M::moon_phase(
            self.lct_hour,
            self.lct_min,
            self.lct_sec,
            self.is_daylight_saving,
            self.zone_correction_hours,
            self.local_date_day,
            self.local_date_month,
            self.local_date_year,
            "A".to_string(),
        );

        println!(
			"Moon phase: [Local Time] {}:{}:{} [DST?] {} [Zone Correction] {} hours [Local Date] {}/{}/{} [Accuracy] {} = [Phase] {} [Position Angle of Bright Limb] {} degrees ",
			self.lct_hour,
			self.lct_min,
			self.lct_sec,
			self.is_daylight_saving,
			self.zone_correction_hours,
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			"A".to_string(),
			moon_phase,
			pa_bright_limb_deg
		);

        assert_eq!(moon_phase, 0.22, "Moon Phase");
        assert_eq!(pa_bright_limb_deg, -71.58, "Position Angle of Bright Limb");
    }

    pub fn test_moon_dist_ang_diam_hor_parallax(&mut self) {
        let (
            earth_moon_dist,
            ang_diameter_deg,
            ang_diameter_min,
            hor_parallax_deg,
            hor_parallax_min,
            hor_parallax_sec,
        ) = M::moon_dist_ang_diam_hor_parallax(
            self.lct_hour,
            self.lct_min,
            self.lct_sec,
            self.is_daylight_saving,
            self.zone_correction_hours,
            self.local_date_day,
            self.local_date_month,
            self.local_date_year,
        );

        println!(
			"Moon distance, angular diameter, and horizontal parallax: [Local Time] {}:{}:{} [DST?] {} [Zone Correction] {} hours [Local Date] {}/{}/{} = [Earth-Moon Distance] {} km [Angular Diameter] {} degrees {} minutes [Horizontal Parallax] {} degrees {} minutes {} seconds",
			self.lct_hour,
			self.lct_min,
			self.lct_sec,
			self.is_daylight_saving,
			self.zone_correction_hours,
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			earth_moon_dist,
			ang_diameter_deg,
			ang_diameter_min,
			hor_parallax_deg,
			hor_parallax_min,
			hor_parallax_sec
		);

        assert_eq!(earth_moon_dist, 367964.0, "Earth-Moon distance (km)");
        assert_eq!(ang_diameter_deg, 0.0, "Angular diameter (degrees part)");
        assert_eq!(ang_diameter_min, 32.0, "Angular diameter (minutes part)");
        assert_eq!(hor_parallax_deg, 0.0, "Horizontal parallax (degrees part)");
        assert_eq!(hor_parallax_min, 59.0, "Horizontal parallax (minutes part)");
        assert_eq!(
            hor_parallax_sec, 35.49,
            "Horizontal parallax (seconds part)"
        );
    }
}

pub fn test_times_of_new_moon_and_full_moon(
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
) {
    let (
        nm_local_time_hour,
        nm_local_time_min,
        nm_local_date_day,
        nm_local_date_month,
        nm_local_date_year,
        fm_local_time_hour,
        fm_local_time_min,
        fm_local_date_day,
        fm_local_date_month,
        fm_local_date_year,
    ) = M::times_of_new_moon_and_full_moon(
        is_daylight_saving,
        zone_correction_hours,
        local_date_day,
        local_date_month,
        local_date_year,
    );

    println!(
		"Times of new moon and full moon: [DST?] {} [Zone Correction] {} [Local Date] {}/{}/{} = [New Moon] [Local Time] {}:{} [Local Date] {}/{}/{} [Full Moon] [Local Time] {}:{} [Local Date] {}/{}/{}",
		is_daylight_saving,
		zone_correction_hours,
		local_date_month,
		local_date_day,
		local_date_year,
		nm_local_time_hour,
		nm_local_time_min,
		nm_local_date_month,
		nm_local_date_day,
		nm_local_date_year,
		fm_local_time_hour,
		fm_local_time_min,
		fm_local_date_month,
		fm_local_date_day,
		fm_local_date_year
	);

    assert_eq!(
        nm_local_time_hour, 17.0,
        "new Moon instant - local time (hour)"
    );
    assert_eq!(
        nm_local_time_min, 27.0,
        "new Moon instant - local time (minutes)"
    );
    assert_eq!(
        nm_local_date_day, 27.0,
        "new Moon instance - local date (day)"
    );
    assert_eq!(
        nm_local_date_month, 8,
        "new Moon instance - local date (month)"
    );
    assert_eq!(
        nm_local_date_year, 2003,
        "new Moon instance - local date (year)"
    );
    assert_eq!(
        fm_local_time_hour, 16.0,
        "full Moon instant - local time (hour)"
    );
    assert_eq!(
        fm_local_time_min, 36.0,
        "full Moon instant - local time (minutes)"
    );
    assert_eq!(
        fm_local_date_day, 10.0,
        "full Moon instance - local date (day)"
    );
    assert_eq!(
        fm_local_date_month, 9,
        "full Moon instance - local date (month)"
    );
    assert_eq!(
        fm_local_date_year, 2003,
        "full Moon instance - local date (year)"
    );
}

pub fn test_moonrise_and_moonset(
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    geog_long_deg: f64,
    geog_lat_deg: f64,
) {
    let (
        mr_lt_hour,
        mr_lt_min,
        mr_local_date_day,
        mr_local_date_month,
        mr_local_date_year,
        mr_azimuth_deg,
        ms_lt_hour,
        ms_lt_min,
        ms_local_date_day,
        ms_local_date_month,
        ms_local_date_year,
        ms_azimuth_deg,
    ) = M::moonrise_and_moonset(
        local_date_day,
        local_date_month,
        local_date_year,
        is_daylight_saving,
        zone_correction_hours,
        geog_long_deg,
        geog_lat_deg,
    );

    println!(
		"Moonrise and Moonset: [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} hours [Geographical Longitude/Latitude] {} degrees / {} degrees = [Moonrise] [Local Time] {}:{} [Local Date] {}/{}/{} [Azimuth] {} degrees [Moonset] [Local Time] {}:{} [Local Date] {}/{}/{} [Azimuth] {} degrees",
		local_date_month,
		local_date_day,
		local_date_year,
		is_daylight_saving,
		zone_correction_hours,
		geog_long_deg,
		geog_lat_deg,
		mr_lt_hour,
		mr_lt_min,
		mr_local_date_month,
		mr_local_date_day,
		mr_local_date_year,
		mr_azimuth_deg,
		ms_lt_hour,
		ms_lt_min,
		ms_local_date_month,
		ms_local_date_day,
		ms_local_date_year,
		ms_azimuth_deg
	);

    assert_eq!(mr_lt_hour, 4.0, "Moonrise - Local Time (hours)");
    assert_eq!(mr_lt_min, 21.0, "Moonrise - Local Time (minutes)");
    assert_eq!(mr_local_date_day, 6.0, "Moonrise - Local Date (day)");
    assert_eq!(mr_local_date_month, 3, "Moonrise - Local Date (month)");
    assert_eq!(mr_local_date_year, 1986, "Moonrise - Local Date (year)");
    assert_eq!(mr_azimuth_deg, 127.34, "Moonrise - Azimuth (degrees)");
    assert_eq!(ms_lt_hour, 13.0, "Moonset - Local Time (hours)");
    assert_eq!(ms_lt_min, 8.0, "Moonset - Local Time (minutes)");
    assert_eq!(ms_local_date_day, 6.0, "Moonset - Local Date (day)");
    assert_eq!(ms_local_date_month, 3, "Moonset - Local Date (month)");
    assert_eq!(ms_local_date_year, 1986, "Moonset - Local Date (year)");
    assert_eq!(ms_azimuth_deg, 234.05, "Moonset - Azimuth (degrees)");
}
