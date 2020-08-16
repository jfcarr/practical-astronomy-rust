use crate::lib::planet as CP;

pub struct TestPositionOfPlanetScaffold {
    pub lct_hour: f64,
    pub lct_minute: f64,
    pub lct_second: f64,
    pub is_daylight_saving: bool,
    pub zone_correction_hours: i32,
    pub local_date_day: f64,
    pub local_date_month: u32,
    pub local_date_year: u32,
    pub planet_name: String,
}
impl TestPositionOfPlanetScaffold {
    pub fn test_approximate_position_of_planet(&mut self) {
        let (
            planet_ra_hour,
            planet_ra_min,
            planet_ra_sec,
            planet_dec_deg,
            planet_dec_min,
            planet_dec_sec,
        ) = CP::approximate_position_of_planet(
            self.lct_hour,
            self.lct_minute,
            self.lct_second,
            self.is_daylight_saving,
            self.zone_correction_hours,
            self.local_date_day,
            self.local_date_month,
            self.local_date_year,
            self.planet_name.to_string(),
        );

        println!(
			"Approximate position of planet: [Local Civil Time] {}:{}:{} [DST?] {} [Zone Correction] {} [Local Date] {}/{}/{} [Planet Name] {} = [RA] {}h {}m {}s [Dec] {}d {}m {}s",
			self.lct_hour,
			self.lct_minute,
			self.lct_second,
			self.is_daylight_saving,
			self.zone_correction_hours,
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			self.planet_name,
			planet_ra_hour,
			planet_ra_min,
			planet_ra_sec,
			planet_dec_deg,
			planet_dec_min,
			planet_dec_sec
		);

        assert_eq!(planet_ra_hour, 11.0, "Planet Right Ascension (hour)");
        assert_eq!(planet_ra_min, 11.0, "Planet Right Ascension (minutes)");
        assert_eq!(planet_ra_sec, 13.8, "Planet Right Ascension (seconds)");
        assert_eq!(planet_dec_deg, 6.0, "Planet Declination (degrees)");
        assert_eq!(planet_dec_min, 21.0, "Planet Declination (minutes)");
        assert_eq!(planet_dec_sec, 25.1, "Planet Declination (seconds)");
    }

    pub fn test_precise_position_of_planet(&mut self) {
        let (
            planet_ra_hour,
            planet_ra_min,
            planet_ra_sec,
            planet_dec_deg,
            planet_dec_min,
            planet_dec_sec,
        ) = CP::precise_position_of_planet(
            self.lct_hour,
            self.lct_minute,
            self.lct_second,
            self.is_daylight_saving,
            self.zone_correction_hours,
            self.local_date_day,
            self.local_date_month,
            self.local_date_year,
            self.planet_name.to_string(),
        );

        println!(
			"Precise position of planet: [Local Civil Time] {}:{}:{} [DST?] {} [Zone Correction] {} [Local Date] {}/{}/{} [Planet Name] {} = [RA] {}h {}m {}s [Dec] {}d {}m {}s",
			self.lct_hour,
			self.lct_minute,
			self.lct_second,
			self.is_daylight_saving,
			self.zone_correction_hours,
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			self.planet_name,
			planet_ra_hour,
			planet_ra_min,
			planet_ra_sec,
			planet_dec_deg,
			planet_dec_min,
			planet_dec_sec
		);

        assert_eq!(planet_ra_hour, 11.0, "Planet Right Ascension (hour)");
        assert_eq!(planet_ra_min, 10.0, "Planet Right Ascension (minutes)");
        assert_eq!(planet_ra_sec, 30.99, "Planet Right Ascension (seconds)");
        assert_eq!(planet_dec_deg, 6.0, "Planet Declination (degrees)");
        assert_eq!(planet_dec_min, 25.0, "Planet Declination (minutes)");
        assert_eq!(planet_dec_sec, 49.46, "Planet Declination (seconds)");
    }

    pub fn test_visual_aspects_of_a_planet(&mut self) {
        let (
            distance_au,
            ang_dia_arcsec,
            phase,
            light_time_hour,
            light_time_minutes,
            light_time_seconds,
            pos_angle_bright_limb_deg,
            approximate_magnitude,
        ) = CP::visual_aspects_of_a_planet(
            self.lct_hour,
            self.lct_minute,
            self.lct_second,
            self.is_daylight_saving,
            self.zone_correction_hours,
            self.local_date_day,
            self.local_date_month,
            self.local_date_year,
            self.planet_name.to_string(),
        );

        println!(
			"Visual aspects of a planet: [Local Civil Time] {}:{}:{} [DST?] {} [Zone Correction] {} [Local Date] {}/{}/{} [Planet] {} = [Distance] {} au [Angular Diameter] {} arcsec [Phase] {} [Light Time] {}:{}:{} [Position Angle of Bright Limb] {} degrees [Approx Magnitude] {}",
			self.lct_hour,
			self.lct_minute,
			self.lct_second,
			self.is_daylight_saving,
			self.zone_correction_hours,
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			self.planet_name.to_string(),
			distance_au,
			ang_dia_arcsec,
			phase,
			light_time_hour,
			light_time_minutes,
			light_time_seconds,
			pos_angle_bright_limb_deg,
			approximate_magnitude
		);

        assert_eq!(distance_au, 5.59829, "Distance - AU");
        assert_eq!(ang_dia_arcsec, 35.1, "Angular Diameter - arcsec");
        assert_eq!(phase, 0.99, "Phase");
        assert_eq!(light_time_hour, 0.0, "Light Time - hour part");
        assert_eq!(light_time_minutes, 46.0, "Light Time - minutes part");
        assert_eq!(light_time_seconds, 33.32, "Light Time - seconds part");
        assert_eq!(
            pos_angle_bright_limb_deg, 113.2,
            "Position Angle of Bright Limb - degrees"
        );
        assert_eq!(approximate_magnitude, -2.0, "Approximate Magnitude");
    }
}
