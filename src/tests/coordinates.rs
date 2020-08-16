use crate::lib::coordinates as CS;
use crate::lib::util;

pub struct TestAngleDecimalDegreesScaffold {
    pub degrees: f64,
    pub minutes: f64,
    pub seconds: f64,
}

impl TestAngleDecimalDegreesScaffold {
    pub fn test_angle_to_decimal_degrees(&mut self) {
        let decimal_degrees = util::round_f64(
            CS::angle_to_decimal_degrees(self.degrees, self.minutes, self.seconds),
            6,
        );

        println!(
            "Angle to decimal degrees: [DMS] {}d {}m {}s = [Decimal Degrees] {}",
            self.degrees, self.minutes, self.seconds, decimal_degrees
        );

        assert_eq!(decimal_degrees, 182.524167, "Decimal Degrees");
    }

    pub fn test_decimal_degrees_to_angle(&mut self) {
        let decimal_degrees = util::round_f64(
            CS::angle_to_decimal_degrees(self.degrees, self.minutes, self.seconds),
            6,
        );

        let (degrees, minutes, seconds) = CS::decimal_degrees_to_angle(decimal_degrees);

        println!(
            "Decimal degrees to angle: [Decimal Degrees] {} = [DMS] {}d {}m {}s",
            decimal_degrees, degrees, minutes, seconds
        );

        assert_eq!(degrees, 182.0, "Degrees");
        assert_eq!(minutes, 31.0, "Minutes");
        assert_eq!(seconds, 27.0, "Seconds");
    }
}

pub struct TestRightAscensionHourAngleScaffold {
    pub ra_hours: f64,
    pub ra_minutes: f64,
    pub ra_seconds: f64,
    pub lct_hours: f64,
    pub lct_minutes: f64,
    pub lct_seconds: f64,
    pub is_daylight_saving: bool,
    pub zone_correction: i32,
    pub local_day: f64,
    pub local_month: u32,
    pub local_year: u32,
    pub geographical_longitude: f64,
}

impl TestRightAscensionHourAngleScaffold {
    pub fn test_right_ascension_to_hour_angle(&mut self) {
        let (hour_angle_hours, hour_angle_minutes, hour_angle_seconds) =
            CS::right_ascension_to_hour_angle(
                self.ra_hours,
                self.ra_minutes,
                self.ra_seconds,
                self.lct_hours,
                self.lct_minutes,
                self.lct_seconds,
                self.is_daylight_saving,
                self.zone_correction,
                self.local_day,
                self.local_month,
                self.local_year,
                self.geographical_longitude,
            );

        println!(
			"Right ascension to hour angle: [RA] {}:{}:{} [LCT] {}:{}:{} [DST?] {} [ZC] {} [Local Date] {}/{}/{} [Geog Long] {} = [HA] {}:{}:{}",
			self.ra_hours,
			self.ra_minutes,
			self.ra_seconds,
			self.lct_hours,
			self.lct_minutes,
			self.lct_seconds,
			self.is_daylight_saving,
			self.zone_correction,
			self.local_month,
			self.local_day,
			self.local_year,
			self.geographical_longitude,
			hour_angle_hours,
			hour_angle_minutes,
			hour_angle_seconds
		);

        assert_eq!(hour_angle_hours, 9.0, "Hour Angle Hours");
        assert_eq!(hour_angle_minutes, 52.0, "Hour Angle Minutes");
        assert_eq!(hour_angle_seconds, 23.66, "Hour Angle Seconds");
    }
    pub fn test_hour_angle_to_right_ascension(&mut self) {
        let (hour_angle_hours, hour_angle_minutes, hour_angle_seconds) =
            CS::right_ascension_to_hour_angle(
                self.ra_hours,
                self.ra_minutes,
                self.ra_seconds,
                self.lct_hours,
                self.lct_minutes,
                self.lct_seconds,
                self.is_daylight_saving,
                self.zone_correction,
                self.local_day,
                self.local_month,
                self.local_year,
                self.geographical_longitude,
            );

        let (right_ascension_hours, right_ascension_minutes, right_ascension_seconds) =
            CS::hour_angle_to_right_ascension(
                hour_angle_hours,
                hour_angle_minutes,
                hour_angle_seconds,
                self.lct_hours,
                self.lct_minutes,
                self.lct_seconds,
                self.is_daylight_saving,
                self.zone_correction,
                self.local_day,
                self.local_month,
                self.local_year,
                self.geographical_longitude,
            );

        println!(
			"Hour angle to right ascension: [HA] {}:{}:{} [LCT] {}:{}:{} [DST?] {} [ZC] {} [Local Date] {}/{}/{} [Geog Long] {} = [RA] {}:{}:{}",
			hour_angle_hours,
			hour_angle_minutes,
			hour_angle_seconds,
			self.lct_hours,
			self.lct_minutes,
			self.lct_seconds,
			self.is_daylight_saving,
			self.zone_correction,
			self.local_month,
			self.local_day,
			self.local_year,
			self.geographical_longitude,
			right_ascension_hours,
			right_ascension_minutes,
			right_ascension_seconds,
		);

        assert_eq!(right_ascension_hours, 18.0, "Right Ascension Hours");
        assert_eq!(right_ascension_minutes, 32.0, "Right Ascension Minutes");
        assert_eq!(right_ascension_seconds, 21.0, "Right Ascension Seconds");
    }
}

pub struct TestEquatorialHorizonScaffold {
    pub hour_angle_hours: f64,
    pub hour_angle_minutes: f64,
    pub hour_angle_seconds: f64,
    pub declination_degrees: f64,
    pub declination_minutes: f64,
    pub declination_seconds: f64,
    pub geographical_latitude: f64,
}

impl TestEquatorialHorizonScaffold {
    pub fn test_equatorial_coordinates_to_horizon_coordinates(&mut self) {
        let (
            azimuth_degrees,
            azimuth_minutes,
            azimuth_seconds,
            altitude_degrees,
            altitude_minutes,
            altitude_seconds,
        ) = CS::equatorial_coordinates_to_horizon_coordinates(
            self.hour_angle_hours,
            self.hour_angle_minutes,
            self.hour_angle_seconds,
            self.declination_degrees,
            self.declination_minutes,
            self.declination_seconds,
            self.geographical_latitude,
        );

        println!(
			"Equatorial coordinates to horizon coordinates: [HA] {}:{}:{} [Declination] {}d {}m {}s [Geographical Latitude] {} = [Azimuth] {}d {}m {}s [Altitude] {}d {}m {}s",
			self.hour_angle_hours,
			self.hour_angle_minutes,
			self.hour_angle_seconds,
			self.declination_degrees,
			self.declination_minutes,
			self.declination_seconds,
			self.geographical_latitude,
			azimuth_degrees,
			azimuth_minutes,
			azimuth_seconds,
			altitude_degrees,
			altitude_minutes,
			altitude_seconds
		);

        assert_eq!(azimuth_degrees, 283.0, "Azimuth Degrees");
        assert_eq!(azimuth_minutes, 16.0, "Azimuth Minutes");
        assert_eq!(azimuth_seconds, 15.7, "Azimuth Seconds");
        assert_eq!(altitude_degrees, 19.0, "Altitude Degrees");
        assert_eq!(altitude_minutes, 20.0, "Altitude Minutes");
        assert_eq!(altitude_seconds, 3.64, "Altitude Seconds");
    }

    pub fn test_horizon_coordinates_to_equatorial_coordinates(&mut self) {
        let (
            azimuth_degrees,
            azimuth_minutes,
            azimuth_seconds,
            altitude_degrees,
            altitude_minutes,
            altitude_seconds,
        ) = CS::equatorial_coordinates_to_horizon_coordinates(
            self.hour_angle_hours,
            self.hour_angle_minutes,
            self.hour_angle_seconds,
            self.declination_degrees,
            self.declination_minutes,
            self.declination_seconds,
            self.geographical_latitude,
        );

        let (
            hour_angle_hours,
            hour_angle_minutes,
            hour_angle_seconds,
            declination_degrees,
            declination_minutes,
            declination_seconds,
        ) = CS::horizon_coordinates_to_equatorial_coordinates(
            azimuth_degrees,
            azimuth_minutes,
            azimuth_seconds,
            altitude_degrees,
            altitude_minutes,
            altitude_seconds,
            self.geographical_latitude,
        );

        println!(
			"Horizon coordinates to equatorial coordinates: [Azimuth] {}d {}m {}s [Altitude] {}d {}m {}s [Geographical Latitude] {} = [HA] {}:{}:{} [Declination] {}d {}m {}s",
			azimuth_degrees,
			azimuth_minutes,
			azimuth_seconds,
			altitude_degrees,
			altitude_minutes,
			altitude_seconds,
			self.geographical_latitude,
			hour_angle_hours,
			hour_angle_minutes,
			hour_angle_seconds,
			declination_degrees,
			declination_minutes,
			declination_seconds,
		);

        assert_eq!(hour_angle_hours, 5.0, "Hour Angle Hours");
        assert_eq!(hour_angle_minutes, 51.0, "Hour Angle Minutes");
        assert_eq!(hour_angle_seconds, 44.0, "Hour Angle Seconds");
        assert_eq!(declination_degrees, 23.0, "Declination Degrees");
        assert_eq!(declination_minutes, 13.0, "Declination Minutes");
        assert_eq!(declination_seconds, 10.0, "Declination Seconds");
    }
}

pub struct TestEclipticScaffold {
    pub ecliptic_longitude_degrees: f64,
    pub ecliptic_longitude_minutes: f64,
    pub ecliptic_longitude_seconds: f64,
    pub ecliptic_latitude_degrees: f64,
    pub ecliptic_latitude_minutes: f64,
    pub ecliptic_latitude_seconds: f64,
    pub greenwich_day: f64,
    pub greenwich_month: u32,
    pub greenwich_year: u32,
}
impl TestEclipticScaffold {
    pub fn test_mean_obliquity_of_the_ecliptic(&mut self) {
        let obliquity = util::round_f64(
            CS::mean_obliquity_of_the_ecliptic(
                self.greenwich_day,
                self.greenwich_month,
                self.greenwich_year,
            ),
            8,
        );

        println!(
            "Mean obliquity of the ecliptic: [Greenwich Date] {}/{}/{} = [Obliquity] {}",
            self.greenwich_month, self.greenwich_day, self.greenwich_year, obliquity
        );

        assert_eq!(obliquity, 23.43805531, "Obliquity");
    }

    pub fn test_ecliptic_coordinate_to_equatorial_coordinate(&mut self) {
        let (ra_hours, ra_minutes, ra_seconds, dec_degrees, dec_minutes, dec_seconds) =
            CS::ecliptic_coordinate_to_equatorial_coordinate(
                self.ecliptic_longitude_degrees,
                self.ecliptic_longitude_minutes,
                self.ecliptic_longitude_seconds,
                self.ecliptic_latitude_degrees,
                self.ecliptic_latitude_minutes,
                self.ecliptic_latitude_seconds,
                self.greenwich_day,
                self.greenwich_month,
                self.greenwich_year,
            );

        println!(
			"Ecliptic coordinates to equatorial coordinates: [Ecliptic] [Longitude] {}d {}m {}s [Latitude] {}d {}m {}s [Greenwich Date] {}/{}/{} = [Right Ascension] {}:{}:{} [Declination] {}d {}m {}s",
			self.ecliptic_longitude_degrees,
			self.ecliptic_longitude_minutes,
			self.ecliptic_longitude_seconds,
			self.ecliptic_latitude_degrees,
			self.ecliptic_latitude_minutes,
			self.ecliptic_latitude_seconds,
			self.greenwich_month,
			self.greenwich_day,
			self.greenwich_year,
			ra_hours,
			ra_minutes,
			ra_seconds,
			dec_degrees,
			dec_minutes,
			dec_seconds
		);

        assert_eq!(ra_hours, 9.0, "RA Hours");
        assert_eq!(ra_minutes, 34.0, "RA Minutes");
        assert_eq!(ra_seconds, 53.4, "RA Seconds");
        assert_eq!(dec_degrees, 19.0, "Dec Degrees");
        assert_eq!(dec_minutes, 32.0, "Dec Minutes");
        assert_eq!(dec_seconds, 8.52, "Dec Seconds");
    }

    pub fn test_equatorial_coordinate_to_ecliptic_coordinate(&mut self) {
        let (ra_hours, ra_minutes, ra_seconds, dec_degrees, dec_minutes, dec_seconds) =
            CS::ecliptic_coordinate_to_equatorial_coordinate(
                self.ecliptic_longitude_degrees,
                self.ecliptic_longitude_minutes,
                self.ecliptic_longitude_seconds,
                self.ecliptic_latitude_degrees,
                self.ecliptic_latitude_minutes,
                self.ecliptic_latitude_seconds,
                self.greenwich_day,
                self.greenwich_month,
                self.greenwich_year,
            );

        let (ecl_long_deg, ecl_long_min, ecl_long_sec, ecl_lat_deg, ecl_lat_min, ecl_lat_sec) =
            CS::equatorial_coordinate_to_ecliptic_coordinate(
                ra_hours,
                ra_minutes,
                ra_seconds,
                dec_degrees,
                dec_minutes,
                dec_seconds,
                self.greenwich_day,
                self.greenwich_month,
                self.greenwich_year,
            );

        println!(
			"Equatorial coordinates to ecliptic coordinates: [Right Ascension] {}:{}:{} [Declination] {}d {}m {}s [Greenwich Date] {}/{}/{} = [Ecliptic] [Longitude] {}d {}m {}s [Latitude] {}d {}m {}s",
			ra_hours,
			ra_minutes,
			ra_seconds,
			dec_degrees,
			dec_minutes,
			dec_seconds,
			self.greenwich_month,
			self.greenwich_day,
			self.greenwich_year,
			ecl_long_deg,
			ecl_long_min,
			ecl_long_sec,
			ecl_lat_deg,
			ecl_lat_min,
			ecl_lat_sec
		);

        assert_eq!(ecl_long_deg, 139.0, "Ecliptic Longitude Degrees");
        assert_eq!(ecl_long_min, 41.0, "Ecliptic Longitude Minutes");
        assert_eq!(ecl_long_sec, 9.97, "Ecliptic Longitude Seconds");
        assert_eq!(ecl_lat_deg, 4.0, "Ecliptic Latitude Degrees");
        assert_eq!(ecl_lat_min, 52.0, "Ecliptic Latitude Minutes");
        assert_eq!(ecl_lat_sec, 30.99, "Ecliptic Latitude Seconds");
    }
}

pub struct TestGalacticScaffold {
    pub ra_hours: f64,
    pub ra_minutes: f64,
    pub ra_seconds: f64,
    pub dec_degrees: f64,
    pub dec_minutes: f64,
    pub dec_seconds: f64,
}
impl TestGalacticScaffold {
    pub fn test_equatorial_coordinate_to_galactic_coordinate(&mut self) {
        let (gal_long_deg, gal_long_min, gal_long_sec, gal_lat_deg, gal_lat_min, gal_lat_sec) =
            CS::equatorial_coordinate_to_galactic_coordinate(
                self.ra_hours,
                self.ra_minutes,
                self.ra_seconds,
                self.dec_degrees,
                self.dec_minutes,
                self.dec_seconds,
            );

        println!(
			"Equatorial coordinate to galactic coordinate: [RA] {}:{}:{} [Dec] {}d {}m {}s = [Galactic] [Long] {}d {}m {}s [Lat] {}d {}m {}s",
			self.ra_hours,
			self.ra_minutes,
			self.ra_seconds,
			self.dec_degrees,
			self.dec_minutes,
			self.dec_seconds,
			gal_long_deg,
			gal_long_min,
			gal_long_sec,
			gal_lat_deg,
			gal_lat_min,
			gal_lat_sec
		);

        assert_eq!(gal_long_deg, 232.0, "Galactic Longitude Degrees");
        assert_eq!(gal_long_min, 14.0, "Galactic Longitude Minutes");
        assert_eq!(gal_long_sec, 52.38, "Galactic Longitude Seconds");
        assert_eq!(gal_lat_deg, 51.0, "Galactic Latitude Degrees");
        assert_eq!(gal_lat_min, 7.0, "Galactic Latitude Minutes");
        assert_eq!(gal_lat_sec, 20.16, "Galactic Latitude Seconds");
    }

    pub fn test_galactic_coordinate_to_equatorial_coordinate(&mut self) {
        let (gal_long_deg, gal_long_min, gal_long_sec, gal_lat_deg, gal_lat_min, gal_lat_sec) =
            CS::equatorial_coordinate_to_galactic_coordinate(
                self.ra_hours,
                self.ra_minutes,
                self.ra_seconds,
                self.dec_degrees,
                self.dec_minutes,
                self.dec_seconds,
            );

        let (ra_hours, ra_minutes, ra_seconds, dec_degrees, dec_minutes, dec_seconds) =
            CS::galactic_coordinate_to_equatorial_coordinate(
                gal_long_deg,
                gal_long_min,
                gal_long_sec,
                gal_lat_deg,
                gal_lat_min,
                gal_lat_sec,
            );

        println!(
			"Galactic coordinate to equatorial coordinate: [Galactic] [Long] {}d {}m {}s [Lat] {}d {}m {}s = [RA] {}:{}:{} [Dec] {}d {}m {}s", 
			gal_long_deg,
			gal_long_min,
			gal_long_sec,
			gal_lat_deg,
			gal_lat_min,
			gal_lat_sec,
			ra_hours,
			ra_minutes,
			ra_seconds,
			dec_degrees,
			dec_minutes,
			dec_seconds,
		);

        assert_eq!(ra_hours, 10.0, "Right Ascension Hours");
        assert_eq!(ra_minutes, 21.0, "Right Ascension Minutes");
        assert_eq!(ra_seconds, 0.0, "Right Ascension Seconds");
        assert_eq!(dec_degrees, 10.0, "Declination Degrees");
        assert_eq!(dec_minutes, 3.0, "Declination Degrees");
        assert_eq!(dec_seconds, 11.0, "Declination Seconds");
    }
}

pub fn test_angle_between_two_objects(
    ra_long_1_hour_deg: f64,
    ra_long_1_min: f64,
    ra_long_1_sec: f64,
    dec_lat_1_deg: f64,
    dec_lat_1_min: f64,
    dec_lat_1_sec: f64,
    ra_long_2_hour_deg: f64,
    ra_long_2_min: f64,
    ra_long_2_sec: f64,
    dec_lat_2_deg: f64,
    dec_lat_2_min: f64,
    dec_lat_2_sec: f64,
    hour_or_degree: String,
) {
    let (angle_deg, angle_min, angle_sec) = CS::angle_between_two_objects(
        ra_long_1_hour_deg,
        ra_long_1_min,
        ra_long_1_sec,
        dec_lat_1_deg,
        dec_lat_1_min,
        dec_lat_1_sec,
        ra_long_2_hour_deg,
        ra_long_2_min,
        ra_long_2_sec,
        dec_lat_2_deg,
        dec_lat_2_min,
        dec_lat_2_sec,
        hour_or_degree.to_string(),
    );

    println!(
		"Angle between two objects: [Object 1] [RA Long] {}{} {}m {}s [Dec Lat] {}d {}m {}s [Object 2] [RA Long] {}{} {}m {}s [Dec Lat] {}d {}m {}s [Hour or Degree?] {} = [Angle] {}d {}m {}s",
		ra_long_1_hour_deg,
		if hour_or_degree == "H" {"h"} else {"d"},
		ra_long_1_min,
		ra_long_1_sec,
		dec_lat_1_deg,
		dec_lat_1_min,
		dec_lat_1_sec,
		ra_long_2_hour_deg,
		if hour_or_degree == "H" {"h"} else {"d"},
		ra_long_2_min,
		ra_long_2_sec,
		dec_lat_2_deg,
		dec_lat_2_min,
		dec_lat_2_sec,
		hour_or_degree,
		angle_deg,
		angle_min,
		angle_sec
	);

    assert_eq!(angle_deg, 23.0, "Angle Degrees");
    assert_eq!(angle_min, 40.0, "Angle Minutes");
    assert_eq!(angle_sec, 25.86, "Angle Seconds");
}

pub fn test_rising_and_setting(
    ra_hours: f64,
    ra_minutes: f64,
    ra_seconds: f64,
    dec_deg: f64,
    dec_min: f64,
    dec_sec: f64,
    gw_date_day: f64,
    gw_date_month: u32,
    gw_date_year: u32,
    geog_long_deg: f64,
    geog_lat_deg: f64,
    vert_shift_deg: f64,
) {
    let (rise_set_status, ut_rise_hour, ut_rise_min, ut_set_hour, ut_set_min, az_rise, az_set) =
        CS::rising_and_setting(
            ra_hours,
            ra_minutes,
            ra_seconds,
            dec_deg,
            dec_min,
            dec_sec,
            gw_date_day,
            gw_date_month,
            gw_date_year,
            geog_long_deg,
            geog_lat_deg,
            vert_shift_deg,
        );

    println!(
		"Rising and setting: [RA] {}h {}m {}s [Dec] {}d {}m {}s, [Greenwich Date] {}/{}/{} [Geog Long/Lat] {}/{} [Vertical Shift] {}d = [Status] {} [Rise] {}:{} ut [Set] {}:{} ut [AZ Rise/Set] {}/{}",
		ra_hours,
		ra_minutes,
		ra_seconds,
		dec_deg,
		dec_min,
		dec_sec,
		gw_date_month,
		gw_date_day,
		gw_date_year,
		geog_long_deg,
		geog_lat_deg,
		vert_shift_deg,
		rise_set_status.to_string(),
		ut_rise_hour,
		ut_rise_min,
		ut_set_hour,
		ut_set_min,
		az_rise,
		az_set
	);

    assert_eq!(rise_set_status, "OK", "Rise/Set Status");
    assert_eq!(ut_rise_hour, 14.0, "UT Rise Hour");
    assert_eq!(ut_rise_min, 16.0, "UT Rise Minute");
    assert_eq!(ut_set_hour, 4.0, "UT Set Hour");
    assert_eq!(ut_set_min, 10.0, "UT Set Minute");
    assert_eq!(az_rise, 64.36, "AZ Rise");
    assert_eq!(az_set, 295.64, "AZ Set");
}

pub fn test_correct_for_precession(
    ra_hour: f64,
    ra_minutes: f64,
    ra_seconds: f64,
    dec_deg: f64,
    dec_minutes: f64,
    dec_seconds: f64,
    epoch1_day: f64,
    epoch1_month: u32,
    epoch1_year: u32,
    epoch2_day: f64,
    epoch2_month: u32,
    epoch2_year: u32,
) {
    let (
        corrected_ra_hour,
        corrected_ra_minutes,
        corrected_ra_seconds,
        corrected_dec_deg,
        corrected_dec_minutes,
        corrected_dec_seconds,
    ) = CS::correct_for_precession(
        ra_hour,
        ra_minutes,
        ra_seconds,
        dec_deg,
        dec_minutes,
        dec_seconds,
        epoch1_day,
        epoch1_month,
        epoch1_year,
        epoch2_day,
        epoch2_month,
        epoch2_year,
    );

    println!(
		"Correct for precession: [RA] {}h {}m {}s [Dec] {}d {}m {}s [Epoch 1] {}/{}/{} [Epoch 2] {}/{}/{} = [Corrected Values] [RA] {}h {}m {}s [Dec] {}d {}m {}s",
		ra_hour,
		ra_minutes,
		ra_seconds,
		dec_deg,
		dec_minutes,
		dec_seconds,
		epoch1_month,
		epoch1_day,
		epoch1_year,
		epoch2_month,
		epoch2_day,
		epoch2_year,
		corrected_ra_hour,
		corrected_ra_minutes,
		corrected_ra_seconds,
		corrected_dec_deg,
		corrected_dec_minutes,
		corrected_dec_seconds
	);

    assert_eq!(corrected_ra_hour, 9.0, "Corrected Right Ascension Hour");
    assert_eq!(
        corrected_ra_minutes, 12.0,
        "Corrected Right Ascension Minutes"
    );
    assert_eq!(
        corrected_ra_seconds, 20.18,
        "Corrected Right Ascension Seconds"
    );
    assert_eq!(corrected_dec_deg, 14.0, "Corrected Declination Hour");
    assert_eq!(corrected_dec_minutes, 16.0, "Corrected Declination Minutes");
    assert_eq!(corrected_dec_seconds, 9.12, "Corrected Declination Seconds");
}

pub fn test_nutation_in_ecliptic_longitude_and_obliquity(
    greenwich_day: f64,
    greenwich_month: u32,
    greenwich_year: u32,
) {
    let (nut_in_long_deg, nut_in_obl_deg) = CS::nutation_in_ecliptic_longitude_and_obliquity(
        greenwich_day,
        greenwich_month,
        greenwich_year,
    );

    let nut_in_long_deg = util::round_f64(nut_in_long_deg, 9);
    let nut_in_obl_deg = util::round_f64(nut_in_obl_deg, 7);

    println!(
		"Nutation in ecliptic longitude and obliquity: [Greenwich Date] {}/{}/{} = [Nutation] [Longitude] {}d [Obliquity] {}d",
		greenwich_month,
		greenwich_day,
		greenwich_year,
		nut_in_long_deg,
		nut_in_obl_deg
	);

    assert_eq!(
        nut_in_long_deg, 0.001525808,
        "Nutation in Longitude (degrees)"
    );
    assert_eq!(nut_in_obl_deg, 0.0025671, "Nutation in Obliquity (degrees)");
}

pub fn test_correct_for_aberration(
    ut_hour: f64,
    ut_minutes: f64,
    ut_seconds: f64,
    gw_day: f64,
    gw_month: u32,
    gw_year: u32,
    true_ecl_long_deg: f64,
    true_ecl_long_min: f64,
    true_ecl_long_sec: f64,
    true_ecl_lat_deg: f64,
    true_ecl_lat_min: f64,
    true_ecl_lat_sec: f64,
) {
    let (
        apparent_ecl_long_deg,
        apparent_ecl_long_min,
        apparent_ecl_long_sec,
        apparent_ecl_lat_deg,
        apparent_ecl_lat_min,
        apparent_ecl_lat_sec,
    ) = CS::correct_for_aberration(
        ut_hour,
        ut_minutes,
        ut_seconds,
        gw_day,
        gw_month,
        gw_year,
        true_ecl_long_deg,
        true_ecl_long_min,
        true_ecl_long_sec,
        true_ecl_lat_deg,
        true_ecl_lat_min,
        true_ecl_lat_sec,
    );

    println!(
		"Correct for aberration: [UT] {}:{}:{} [Greenwich Date] {}/{}/{} [True Ecliptic] [Longitude] {}d {}m {}s [Latitude] {}d {}m {}s = [Apparent Ecliptic] [Longitude] {}d {}m {}s [Latitude] {}d {}m {}s",
		ut_hour,
		ut_minutes,
		ut_seconds,
		gw_month,
		gw_day,
		gw_year,
		true_ecl_long_deg,
		true_ecl_long_min,
		true_ecl_long_sec,
		true_ecl_lat_deg,
		true_ecl_lat_min,
		true_ecl_lat_sec,
		apparent_ecl_long_deg,
		apparent_ecl_long_min,
		apparent_ecl_long_sec,
		apparent_ecl_lat_deg,
		apparent_ecl_lat_min,
		apparent_ecl_lat_sec
	);

    assert_eq!(
        apparent_ecl_long_deg, 352.0,
        "Apparent Ecliptic Longitude Degrees"
    );
    assert_eq!(
        apparent_ecl_long_min, 37.0,
        "Apparent Ecliptic Longitude Minutes"
    );
    assert_eq!(
        apparent_ecl_long_sec, 30.45,
        "Apparent Ecliptic Longitude Seconds"
    );
    assert_eq!(
        apparent_ecl_lat_deg, -1.0,
        "Apparent Ecliptic Latitude Degrees"
    );
    assert_eq!(
        apparent_ecl_lat_min, 32.0,
        "Apparent Ecliptic Latitude Minutes"
    );
    assert_eq!(
        apparent_ecl_lat_sec, 56.33,
        "Apparent Ecliptic Latitude Seconds"
    );
}

pub fn test_atmospheric_refraction(
    true_ra_hour: f64,
    true_ra_min: f64,
    true_ra_sec: f64,
    true_dec_deg: f64,
    true_dec_min: f64,
    true_dec_sec: f64,
    coordinate_type: String,
    geog_long_deg: f64,
    geog_lat_deg: f64,
    daylight_saving_hours: i32,
    timezone_hours: i32,
    lcd_day: f64,
    lcd_month: u32,
    lcd_year: u32,
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
    atmospheric_pressure_mbar: f64,
    atmospheric_temperature_celsius: f64,
) {
    let (
        corrected_ra_hour,
        corrected_ra_min,
        corrected_ra_sec,
        corrected_dec_deg,
        corrected_dec_min,
        corrected_dec_sec,
    ) = CS::atmospheric_refraction(
        true_ra_hour,
        true_ra_min,
        true_ra_sec,
        true_dec_deg,
        true_dec_min,
        true_dec_sec,
        coordinate_type.to_string(),
        geog_long_deg,
        geog_lat_deg,
        daylight_saving_hours,
        timezone_hours,
        lcd_day,
        lcd_month,
        lcd_year,
        lct_hour,
        lct_min,
        lct_sec,
        atmospheric_pressure_mbar,
        atmospheric_temperature_celsius,
    );

    println!(
		"Atmospheric refraction:  [RA] {}:{}:{} [DEC] {}d {}m {}s [COORD TYPE] {} [GEOG LON/LAT] {}d/{}d [DS HOURS] {} [TZ HOURS] {} [LCD] {}/{}/{} [LCT] {}:{}:{} [ATM] [PRESS MBR] {} [TEMP C] {} = [CORRECTED] [RA] {}:{}:{} [DEC] {}d {}m {}s",
		true_ra_hour,
		true_ra_min,
		true_ra_sec,
		true_dec_deg,
		true_dec_min,
		true_dec_sec,
		coordinate_type.to_string(),
		geog_long_deg,
		geog_lat_deg,
		daylight_saving_hours,
		timezone_hours,
		lcd_month,
		lcd_day,
		lcd_year,
		lct_hour,
		lct_min,
		lct_sec,
		atmospheric_pressure_mbar,
		atmospheric_temperature_celsius,
		corrected_ra_hour,
		corrected_ra_min,
		corrected_ra_sec,
		corrected_dec_deg,
		corrected_dec_min,
		corrected_dec_sec,
	);

    assert_eq!(corrected_ra_hour, 23.0, "Corrected RA Hours");
    assert_eq!(corrected_ra_min, 13.0, "Corrected RA Minutes");
    assert_eq!(corrected_ra_sec, 44.74, "Corrected RA Seconds");
    assert_eq!(corrected_dec_deg, 40.0, "Corrected Declination Degrees");
    assert_eq!(corrected_dec_min, 19.0, "Corrected Declination Minutes");
    assert_eq!(corrected_dec_sec, 45.76, "Corrected Declination Seconds");
}

pub fn test_corrections_for_geocentric_parallax(
    ra_hour: f64,
    ra_min: f64,
    ra_sec: f64,
    dec_deg: f64,
    dec_min: f64,
    dec_sec: f64,
    coordinate_type: String,
    equatorial_hor_parallax_deg: f64,
    geog_long_deg: f64,
    geog_lat_deg: f64,
    height_m: f64,
    daylight_saving: i32,
    timezone_hours: i32,
    lcd_day: f64,
    lcd_month: u32,
    lcd_year: u32,
    lct_hour: f64,
    lct_min: f64,
    lct_sec: f64,
) {
    let (
        corrected_ra_hour,
        corrected_ra_min,
        corrected_ra_sec,
        corrected_dec_deg,
        corrected_dec_min,
        corrected_dec_sec,
    ) = CS::corrections_for_geocentric_parallax(
        ra_hour,
        ra_min,
        ra_sec,
        dec_deg,
        dec_min,
        dec_sec,
        coordinate_type.to_string(),
        equatorial_hor_parallax_deg,
        geog_long_deg,
        geog_lat_deg,
        height_m,
        daylight_saving,
        timezone_hours,
        lcd_day,
        lcd_month,
        lcd_year,
        lct_hour,
        lct_min,
        lct_sec,
    );

    println!(
		"Corrections for geocentric parallax: [RA] {}h {}m {}s [Dec] {}d {}m {}s [Coordinate Type] {} [Eq Hor Parallax] {}d [Geog Long/Lat] {}/{} [Height] {}m [DST] {}h [TZ] {}h [Local Civil Date] {}/{}/{} [Local Civil Time] {}:{}:{} = [Corrected] [RA] {}h {}m {}s [Dec] {}h {}m {}s",
		ra_hour,
		ra_min,
		ra_sec,
		dec_deg,
		dec_min,
		dec_sec,
		coordinate_type.to_string(),
		equatorial_hor_parallax_deg,
		geog_long_deg,
		geog_lat_deg,
		height_m,
		daylight_saving,
		timezone_hours,
		lcd_month,
		lcd_day,
		lcd_year,
		lct_hour,
		lct_min,
		lct_sec,
		corrected_ra_hour,
		corrected_ra_min,
		corrected_ra_sec,
		corrected_dec_deg,
		corrected_dec_min,
		corrected_dec_sec
	);

    assert_eq!(corrected_ra_hour, 22.0, "Corrected RA Hours");
    assert_eq!(corrected_ra_min, 36.0, "Corrected RA Minutes");
    assert_eq!(corrected_ra_sec, 43.22, "Corrected RA Seconds");
    assert_eq!(corrected_dec_deg, -8.0, "Corrected Declination Degrees");
    assert_eq!(corrected_dec_min, 32.0, "Corrected Declination Minutes");
    assert_eq!(corrected_dec_sec, 17.4, "Corrected Declination Seconds");
}

pub fn test_heliographic_coordinates(
    helio_position_angle_deg: f64,
    helio_displacement_arcmin: f64,
    gwdate_day: f64,
    gwdate_month: u32,
    gwdate_year: u32,
) {
    let (helio_long_deg, helio_lat_deg) = CS::heliographic_coordinates(
        helio_position_angle_deg,
        helio_displacement_arcmin,
        gwdate_day,
        gwdate_month,
        gwdate_year,
    );

    println!(
		"Heliographic coordinates: [Helio] [Position Angle] {}d [Displacement] {} arcmin [Greenwich Date] {}/{}/{} = [Helio] [Longitude] {}d [Latitude] {}d",
		helio_position_angle_deg,
		helio_displacement_arcmin,
		gwdate_month,
		gwdate_day,
		gwdate_year,
		helio_long_deg,
		helio_lat_deg
	);

    assert_eq!(helio_long_deg, 142.59, "Heliographic Longitude - degrees");
    assert_eq!(helio_lat_deg, -19.94, "Heliographic Latitude - degrees");
}

pub fn test_carrington_rotation_number(gwdate_day: f64, gwdate_month: u32, gwdate_year: u32) {
    let crn = CS::carrington_rotation_number(gwdate_day, gwdate_month, gwdate_year);

    println!(
        "Carrington rotation number: [Greenwich Date] {}/{}/{} = [CRN] {}",
        gwdate_month, gwdate_day, gwdate_year, crn
    );

    assert_eq!(crn, 1624, "Carrington Rotation Number");
}

pub struct TestSelenographicScaffold {
    pub gwdate_day: f64,
    pub gwdate_month: u32,
    pub gwdate_year: u32,
}
impl TestSelenographicScaffold {
    pub fn test_selenographic_coordinates_1(&mut self) {
        let (sub_earth_longitude, sub_earth_latitude, position_angle_of_pole) =
            CS::selenographic_coordinates_1(self.gwdate_day, self.gwdate_month, self.gwdate_year);

        println!(
			"Selenographic coordinates 1: [Greenwich Date] {}/{}/{} = [Sub-Earth] [Longitude] {} [Latitude] {} [Position Angle of Pole] {}",
			self.gwdate_month,
			self.gwdate_day,
			self.gwdate_year,
			sub_earth_longitude,
			sub_earth_latitude,
			position_angle_of_pole
		);

        assert_eq!(sub_earth_longitude, -4.88, "Sub-Earth Longitude");
        assert_eq!(sub_earth_latitude, 4.04, "Sub-Earth Latitude");
        assert_eq!(position_angle_of_pole, 19.78, "Position Angle of Pole");
    }

    pub fn test_selenographic_coordinates_2(&mut self) {
        let (sub_solar_longitude, sub_solar_colongitude, sub_solar_latitude) =
            CS::selenographic_coordinates_2(self.gwdate_day, self.gwdate_month, self.gwdate_year);

        println!(
			"Selenographic coordinates 2: [Greenwich Date] {}/{}/{} = [Sub-Solar] [Longitude] {} [Co-Longitude] {} [Latitude] {}",
			self.gwdate_month,
			self.gwdate_day,
			self.gwdate_year,
			sub_solar_longitude,
			sub_solar_colongitude,
			sub_solar_latitude
		);

        assert_eq!(sub_solar_longitude, 6.81, "Sub-Solar Longitude");
        assert_eq!(sub_solar_colongitude, 83.19, "Sub-Solar Colongitude");
        assert_eq!(sub_solar_latitude, 1.19, "Sub-Solar Latitude");
    }
}
