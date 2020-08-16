use crate::lib::eclipses as ECL;

pub struct TestLunarEclipseScaffold {
    pub local_date_day: f64,
    pub local_date_month: u32,
    pub local_date_year: u32,
    pub is_daylight_saving: bool,
    pub zone_correction_hours: i32,
}
impl TestLunarEclipseScaffold {
    pub fn test_lunar_eclipse_occurrence(&mut self) {
        let (status, event_date_day, event_date_month, event_date_year) =
            ECL::lunar_eclipse_occurrence(
                self.local_date_day,
                self.local_date_month,
                self.local_date_year,
                self.is_daylight_saving,
                self.zone_correction_hours,
            );

        println!(
			"Lunar eclipse occurrence: [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} = [Status] {:} [Event Date] {}/{}/{}",
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			self.is_daylight_saving,
			self.zone_correction_hours,
			status,
			event_date_month,
			event_date_day,
			event_date_year
		);

        assert_eq!(status, "Lunar eclipse certain", "Lunar eclipse status");
        assert_eq!(event_date_day, 4.0, "Lunar eclipse event date (day)");
        assert_eq!(event_date_month, 4, "Lunar eclipse event date (month)");
        assert_eq!(event_date_year, 2015, "Lunar eclipse event date (year)");
    }
    pub fn test_lunar_eclipse_circumstances(&mut self) {
        let (
            lunar_eclipse_certain_date_day,
            lunar_eclipse_certain_date_month,
            lunar_eclipse_certain_date_year,
            ut_start_pen_phase_hour,
            ut_start_pen_phase_minutes,
            ut_start_umbral_phase_hour,
            ut_start_umbral_phase_minutes,
            ut_start_total_phase_hour,
            ut_start_total_phase_minutes,
            ut_mid_eclipse_hour,
            ut_mid_eclipse_minutes,
            ut_end_total_phase_hour,
            ut_end_total_phase_minutes,
            ut_end_umbral_phase_hour,
            ut_end_umbral_phase_minutes,
            ut_end_pen_phase_hour,
            ut_end_pen_phase_minutes,
            eclipse_magnitude,
        ) = ECL::lunar_eclipse_circumstances(
            self.local_date_day,
            self.local_date_month,
            self.local_date_year,
            self.is_daylight_saving,
            self.zone_correction_hours,
        );

        println!(
			"Lunar eclipse circumstances: [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} hours = [Certain Date] {}/{}/{} [Start] [Penumbral] {}:{} [Umbral] {}:{} [Total] {}:{} [Mid] {}:{} [End] [Total] {}:{} [Umbral] {}:{} [Penumbral] {}:{} [Magnitude] {}",
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			self.is_daylight_saving,
			self.zone_correction_hours,
			lunar_eclipse_certain_date_month,
			lunar_eclipse_certain_date_day,
			lunar_eclipse_certain_date_year,
			ut_start_pen_phase_hour,
			ut_start_pen_phase_minutes,
			ut_start_umbral_phase_hour,
			ut_start_umbral_phase_minutes,
			ut_start_total_phase_hour,
			ut_start_total_phase_minutes,
			ut_mid_eclipse_hour,
			ut_mid_eclipse_minutes,
			ut_end_total_phase_hour,
			ut_end_total_phase_minutes,
			ut_end_umbral_phase_hour,
			ut_end_umbral_phase_minutes,
			ut_end_pen_phase_hour,
			ut_end_pen_phase_minutes,
			eclipse_magnitude
		);

        assert_eq!(lunar_eclipse_certain_date_day, 4.0, "Eclipse Date (day)");
        assert_eq!(lunar_eclipse_certain_date_month, 4, "Eclipse Date (month)");
        assert_eq!(lunar_eclipse_certain_date_year, 2015, "Eclipse Date (year)");
        assert_eq!(ut_start_pen_phase_hour, 9.0, "Start Penumbral Phase (hour)");
        assert_eq!(
            ut_start_pen_phase_minutes, 0.0,
            "Start Penumbral Phase (minutes)"
        );
        assert_eq!(
            ut_start_umbral_phase_hour, 10.0,
            "Start Umbral Phase (hour)"
        );
        assert_eq!(
            ut_start_umbral_phase_minutes, 16.0,
            "Start Umbral Phase (minutes)"
        );
        assert_eq!(ut_start_total_phase_hour, 11.0, "Start Total Phase (hour)");
        assert_eq!(
            ut_start_total_phase_minutes, 55.0,
            "Start Total Phase (minutes)"
        );
        assert_eq!(ut_mid_eclipse_hour, 12.0, "Mid Eclipse (hour)");
        assert_eq!(ut_mid_eclipse_minutes, 1.0, "Mid Eclipse (minutes)");
        assert_eq!(ut_end_total_phase_hour, 12.0, "End Total Phase (hour)");
        assert_eq!(ut_end_total_phase_minutes, 7.0, "End Total Phase (minutes)");
        assert_eq!(ut_end_umbral_phase_hour, 13.0, "End Umbral Phase (hour)");
        assert_eq!(
            ut_end_umbral_phase_minutes, 46.0,
            "End Umbral Phase (minutes)"
        );
        assert_eq!(ut_end_pen_phase_hour, 15.0, "End Penumbral Phase (hour)");
        assert_eq!(
            ut_end_pen_phase_minutes, 1.0,
            "End Penumbral Phase (minutes)"
        );
        assert_eq!(eclipse_magnitude, 1.01, "Eclipse Magnitude");
    }
}

pub struct TestSolarEclipseScaffold {
    pub local_date_day: f64,
    pub local_date_month: u32,
    pub local_date_year: u32,
    pub is_daylight_saving: bool,
    pub zone_correction_hours: i32,
}
impl TestSolarEclipseScaffold {
    pub fn test_solar_eclipse_occurrence(&mut self) {
        let (status, event_date_day, event_date_month, event_date_year) =
            ECL::solar_eclipse_occurrence(
                self.local_date_day,
                self.local_date_month,
                self.local_date_year,
                self.is_daylight_saving,
                self.zone_correction_hours,
            );

        println!(
			"Solar eclipse occurrence: [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} = [Status] {:} [Event Date] {}/{}/{}",
			self.local_date_month,
			self.local_date_day,
			self.local_date_year,
			self.is_daylight_saving,
			self.zone_correction_hours,
			status,
			event_date_month,
			event_date_day,
			event_date_year
		);

        assert_eq!(status, "Solar eclipse certain", "Lunar eclipse status");
        assert_eq!(event_date_day, 20.0, "Solar eclipse event date (day)");
        assert_eq!(event_date_month, 3, "Solar eclipse event date (month)");
        assert_eq!(event_date_year, 2015, "Solar eclipse event date (year)");
    }
}

pub fn test_solar_eclipse_circumstances(
    local_date_day: f64,
    local_date_month: u32,
    local_date_year: u32,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
    geog_longitude_deg: f64,
    geog_latitude_deg: f64,
) {
    let (
        solar_eclipse_certain_date_day,
        solar_eclipse_certain_date_month,
        solar_eclipse_certain_date_year,
        ut_first_contact_hour,
        ut_first_contact_minutes,
        ut_mid_eclipse_hour,
        ut_mid_eclipse_minutes,
        ut_last_contact_hour,
        ut_last_contact_minutes,
        eclipse_magnitude,
    ) = ECL::solar_eclipse_circumstances(
        local_date_day,
        local_date_month,
        local_date_year,
        is_daylight_saving,
        zone_correction_hours,
        geog_longitude_deg,
        geog_latitude_deg,
    );

    println!(
		"Solar eclipse circumstances: [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} hours [Geographical Longitude/Latitude] {} degrees / {} degrees = [Certain Date] {}/{}/{} [First Contact] {}:{} [Mid Eclipse] {}:{} [Last Contact] {}:{} [Magnitude] {}",
		local_date_month,
		local_date_day,
		local_date_year,
		is_daylight_saving,
		zone_correction_hours,
		geog_longitude_deg,
		geog_latitude_deg,
		solar_eclipse_certain_date_month,
		solar_eclipse_certain_date_day,
		solar_eclipse_certain_date_year,
		ut_first_contact_hour,
		ut_first_contact_minutes,
		ut_mid_eclipse_hour,
		ut_mid_eclipse_minutes,
		ut_last_contact_hour,
		ut_last_contact_minutes,
		eclipse_magnitude
	);

    assert_eq!(solar_eclipse_certain_date_day, 20.0, "Eclipse Date (day)");
    assert_eq!(solar_eclipse_certain_date_month, 3, "Eclipse Date (month)");
    assert_eq!(solar_eclipse_certain_date_year, 2015, "Eclipse Date (year)");
    assert_eq!(ut_first_contact_hour, 8.0, "First Contact (hour)");
    assert_eq!(ut_first_contact_minutes, 55.0, "First Contact (minutes)");
    assert_eq!(ut_mid_eclipse_hour, 9.0, "Mid Eclipse (hour)");
    assert_eq!(ut_mid_eclipse_minutes, 57.0, "Mid Eclipse (minutes)");
    assert_eq!(ut_last_contact_hour, 10.0, "Last Contact (hour)");
    assert_eq!(ut_last_contact_minutes, 58.0, "Last Contact (minutes)");
    assert_eq!(eclipse_magnitude, 1.016, "Eclipse Magnitude");
}
