use crate::lib::datetime as DT;
use crate::lib::macros as MA;
use crate::lib::util as UT;

/// Test date of Easter.
///
/// ## Input
///
/// 2003
///
/// ## Expected Output
///
/// 4/20/2003
pub fn test_easter(expected_month: u32, expected_day: u32, input_year: u32) {
    let (month, day, year) = DT::get_date_of_easter(input_year);

    println!(
        "Date of Easter: [Year] {} = [Date of Easter] {}/{}/{}",
        input_year, month, day, year
    );

    assert_eq!(month, expected_month, "Month of Easter");
    assert_eq!(day, expected_day, "Day of Easter");
    assert_eq!(year, input_year, "Year of Easter");
}

/// Test day numbers (various)
pub fn test_day_numbers() {
    test_day_number(1, 1, 2000, 1);
    test_day_number(3, 1, 2000, 61);
    test_day_number(6, 1, 2003, 152);
    test_day_number(11, 27, 2009, 331);
}

/// Tests calculation of a day number.
///
/// Asserts that the calculated result matches the expected result.
///
/// ## Input
///
/// input_month, input_day, input_year, expected_result
fn test_day_number(input_month: u32, input_day: u32, input_year: u32, expected_result: u32) {
    let result = DT::civil_date_to_day_number(input_month, input_day, input_year);

    println!(
        "Day number: [Date] {}/{}/{} = [Day Number] {}",
        input_month, input_day, input_year, result
    );

    assert_eq!(result, expected_result, "Day Number");
}

/// Civil Time tests.
pub struct TestCivilTimeScaffold {
    pub civil_hours: f64,
    pub civil_minutes: f64,
    pub civil_seconds: f64,
}

impl TestCivilTimeScaffold {
    /// Test conversion of civil time to decimal hours.
    pub fn test_civil_time_to_decimal_hours(&mut self) {
        let decimal_hours = DT::civil_time_to_decimal_hours(
            self.civil_hours,
            self.civil_minutes,
            self.civil_seconds,
        );

        println!(
            "Civil time to decimal hours: [Time] {}:{}:{} = [Decimal Hours] {}",
            self.civil_hours, self.civil_minutes, self.civil_seconds, decimal_hours
        );

        assert_eq!(UT::round_f64(decimal_hours, 8), 18.52416667);
    }
}

impl TestCivilTimeScaffold {
    /// Test conversion of decimal hours to civil time.
    pub fn test_decimal_hours_to_civil_time(&mut self) {
        let decimal_hours = DT::civil_time_to_decimal_hours(
            self.civil_hours,
            self.civil_minutes,
            self.civil_seconds,
        );
        let (civil_hours, civil_minutes, civil_seconds) =
            DT::decimal_hours_to_civil_time(decimal_hours);

        println!(
            "Decimal hours to civil time: [Decimal Hours] {} = [Civil Time] {}:{}:{}",
            decimal_hours, civil_hours, civil_minutes, civil_seconds
        );

        assert_eq!(civil_hours, self.civil_hours);
        assert_eq!(civil_minutes, self.civil_minutes);
        assert_eq!(civil_seconds, self.civil_seconds);
    }
}

impl TestCivilTimeScaffold {
    /// Test extraction of hour part, minutes part, and seconds part from decimal hours.
    pub fn test_decimal_time_parts(&mut self) {
        let decimal_hours = DT::civil_time_to_decimal_hours(
            self.civil_hours,
            self.civil_minutes,
            self.civil_seconds,
        );

        let hour_part = MA::dh_hour(decimal_hours);
        let minutes_part = MA::dh_min(decimal_hours);
        let seconds_part = MA::dh_sec(decimal_hours);

        println!(
			"Extract time parts from decimal hours: [Decimal Hours] {} = [Hour] {} [Minutes] {} [Seconds] {}",
			decimal_hours, hour_part, minutes_part, seconds_part
		);

        assert_eq!(hour_part as f64, self.civil_hours, "Hour Part");
        assert_eq!(minutes_part as f64, self.civil_minutes, "Minutes Part");
        assert_eq!(seconds_part, self.civil_seconds as f64, "Seconds Part");
    }
}

/// Local Civil Time tests.
pub struct TestLocalCivilTimeScaffold {
    pub lct_hours: f64,
    pub lct_minutes: f64,
    pub lct_seconds: f64,
    pub is_daylight_savings: bool,
    pub zone_correction: i32,
    pub local_day: f64,
    pub local_month: u32,
    pub local_year: u32,
}

impl TestLocalCivilTimeScaffold {
    /// Test conversion of local civil time to universal time
    pub fn test_local_civil_time_to_universal_time(&mut self) {
        let (ut_hours, ut_minutes, ut_seconds, gw_day, gw_month, gw_year) =
            DT::local_civil_time_to_universal_time(
                // return utHours,utMinutes,utSeconds,warningFlag
                self.lct_hours,
                self.lct_minutes,
                self.lct_seconds,
                self.is_daylight_savings,
                self.zone_correction,
                self.local_day,
                self.local_month,
                self.local_year,
            );

        println!(
			"Civil time to universal time: [LCT] {}:{}:{} [DST?] {} [ZC] {} [Local Date] {}/{}/{} = [UT] {}:{}:{} [GWD] {}/{}/{}",
			self.lct_hours,
			self.lct_minutes,
			self.lct_seconds,
			self.is_daylight_savings,
			self.zone_correction,
			self.local_month,
			self.local_day,
			self.local_year,
			ut_hours,
			ut_minutes,
			ut_seconds,
			gw_month,
			gw_day,
			gw_year
		);

        assert_eq!(ut_hours, 22, "UT Hours");
        assert_eq!(ut_minutes, 37, "UT Minutes");
        assert_eq!(ut_seconds, 0, "UT Seconds");
        assert_eq!(gw_day, 30, "Greenwich Day");
        assert_eq!(gw_month, 6, "Greenwich Month");
        assert_eq!(gw_year, 2013, "Greenwich Year");
    }
}

impl TestLocalCivilTimeScaffold {
    /// Test conversion of local civil time to universal time
    pub fn test_universal_time_to_local_civil_time(&mut self) {
        let (ut_hours, ut_minutes, ut_seconds, gw_day, gw_month, gw_year) =
            DT::local_civil_time_to_universal_time(
                self.lct_hours,
                self.lct_minutes,
                self.lct_seconds,
                self.is_daylight_savings,
                self.zone_correction,
                self.local_day,
                self.local_month,
                self.local_year,
            );

        let (
            revert_lct_hours,
            revert_lct_minutes,
            revert_lct_seconds,
            revert_day,
            revert_month,
            revert_year,
        ) = DT::universal_time_to_local_civil_time(
            ut_hours as f64,
            ut_minutes as f64,
            ut_seconds as f64,
            self.is_daylight_savings,
            self.zone_correction,
            gw_day,
            gw_month,
            gw_year,
        );

        println!(
			"Universal time to civil time: [UT] {}:{}:{} [DST?] {} [ZC] {} [GWD] {}/{}/{} = [Local Time] {}:{}:{} [Local Date] {}/{}/{}",
			ut_hours,
			ut_minutes,
			ut_seconds,
			self.is_daylight_savings,
			self.zone_correction,
			gw_month,
			gw_day,
			gw_year,
			revert_lct_hours,
			revert_lct_minutes,
			revert_lct_seconds,
			revert_month,
			revert_day,
			revert_year
		);

        assert_eq!(revert_lct_hours, 3, "LCT Hours");
        assert_eq!(revert_lct_minutes, 37, "LCT Minutes");
        assert_eq!(revert_lct_seconds, 0, "LCT Seconds");
        assert_eq!(revert_day, 1, "Local Day");
        assert_eq!(revert_month, 7, "Local Month");
        assert_eq!(revert_year, 2013, "Local Year");
    }
}

/// Universal Time <-> Sidereal Time tests.
pub struct TestUniversalTimeSiderealTimeScaffold {
    pub ut_hours: f64,
    pub ut_minutes: f64,
    pub ut_seconds: f64,
    pub gw_day: f64,
    pub gw_month: u32,
    pub gw_year: u32,
}

impl TestUniversalTimeSiderealTimeScaffold {
    /// Test conversion of universal time to greenwich sidereal time
    pub fn test_universal_time_to_greenwich_sidereal_time(&mut self) {
        let (gst_hours, gst_minutes, gst_seconds) = DT::universal_time_to_greenwich_sidereal_time(
            self.ut_hours,
            self.ut_minutes,
            self.ut_seconds,
            self.gw_day,
            self.gw_month,
            self.gw_year,
        );

        println!(
			"Universal time to greenwich sidereal time: [UT] {}:{}:{} [GWD] {}/{}/{} = [GST] {}:{}:{}",
			self.ut_hours,
			self.ut_minutes,
			self.ut_seconds,
			self.gw_month,
			self.gw_day,
			self.gw_year,
			gst_hours,
			gst_minutes,
			gst_seconds
		);

        assert_eq!(gst_hours, 4, "GST Hours");
        assert_eq!(gst_minutes, 40, "GST Minutes");
        assert_eq!(gst_seconds, 5.23, "GST Seconds");
    }

    /// Test conversion of greenwich sidereal time to universal time
    pub fn test_greenwich_sidereal_time_to_universal_time(&mut self) {
        let (gst_hours, gst_minutes, gst_seconds) = DT::universal_time_to_greenwich_sidereal_time(
            self.ut_hours,
            self.ut_minutes,
            self.ut_seconds,
            self.gw_day,
            self.gw_month,
            self.gw_year,
        );

        let (ut_hours, ut_minutes, ut_seconds, warning_flag) =
            DT::greenwich_sidereal_time_to_universal_time(
                gst_hours as f64,
                gst_minutes as f64,
                gst_seconds,
                self.gw_day,
                self.gw_month,
                self.gw_year,
            );

        println!(
			"Greenwich sidereal time to universal time: [GST] {}:{}:{} [GWD] {}/{}/{} = [UT] {}:{}:{} [Warning Flag] {}",
			gst_hours,
			gst_minutes,
			gst_seconds,
			self.gw_month,
			self.gw_day,
			self.gw_year,
			ut_hours,
			ut_minutes,
			ut_seconds,
			warning_flag
		);

        assert_eq!(ut_hours, 14, "UT Hours");
        assert_eq!(ut_minutes, 36, "UT Minutes");
        assert_eq!(ut_seconds, 51.67, "UT Seconds");
        assert_eq!(warning_flag, "OK", "Warning Flag");
    }
}

/// Universal Time <-> Sidereal Time tests.
pub struct TestGreenwichSiderealLocalSiderealScaffold {
    pub gst_hours: f64,
    pub gst_minutes: f64,
    pub gst_seconds: f64,
    pub geographical_longitude: f64,
}

impl TestGreenwichSiderealLocalSiderealScaffold {
    pub fn test_greenwich_sidereal_time_to_local_sidereal_time(&mut self) {
        let (lst_hours, lst_minutes, lst_seconds) =
            DT::greenwich_sidereal_time_to_local_sidereal_time(
                self.gst_hours,
                self.gst_minutes,
                self.gst_seconds,
                self.geographical_longitude,
            );

        println!(
			"Greenwich sidereal time to local sidereal time: [GST] {}:{}:{} [Longitude] {} = [LST] {}:{}:{}",
			self.gst_hours,
			self.gst_minutes,
			self.gst_seconds,
			self.geographical_longitude,
			lst_hours,
			lst_minutes,
			lst_seconds
		);

        assert_eq!(lst_hours, 0, "LST Hours");
        assert_eq!(lst_minutes, 24, "LST Minutes");
        assert_eq!(lst_seconds, 5.23, "LST Seconds");
    }

    pub fn test_local_sidereal_time_to_greenwich_sidereal_time(&mut self) {
        let (lst_hours, lst_minutes, lst_seconds) =
            DT::greenwich_sidereal_time_to_local_sidereal_time(
                self.gst_hours,
                self.gst_minutes,
                self.gst_seconds,
                self.geographical_longitude,
            );

        let (gst_hours, gst_minutes, gst_seconds) =
            DT::local_sidereal_time_to_greenwich_sidereal_time(
                lst_hours as f64,
                lst_minutes as f64,
                lst_seconds,
                self.geographical_longitude,
            );

        println!(
			"Local sidereal time to greenwich sidereal time: [LST] {}:{}:{} [Longitude] {} = [GST] {}:{}:{}",
			lst_hours,
			lst_minutes,
			lst_seconds,
			self.geographical_longitude,
			gst_hours,
			gst_minutes,
			gst_seconds
		);

        assert_eq!(gst_hours, 4, "GST Hours");
        assert_eq!(gst_minutes, 40, "GST Minutes");
        assert_eq!(gst_seconds, 5.23, "GST Seconds");
    }
}

/// Test Julian Date to Day of Week
pub fn test_julian_date_to_day_of_week() {
    let julian_date = 2455001.5;

    let day_of_week = MA::f_dow(julian_date);

    println!(
        "Julian date to day of week: [JD] {} = [Day of Week] {}",
        julian_date, day_of_week
    );

    assert_eq!(day_of_week, "Friday", "Day of Week");
}
