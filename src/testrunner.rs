use crate::lib::types as pa_types;
use crate::tests::binary as BINS;
use crate::tests::comet as COMT;
use crate::tests::coordinates as CST;
use crate::tests::datetime as DTT;
use crate::tests::eclipses as ECL;
use crate::tests::moon as MOONT;
use crate::tests::planet as PLANETT;
use crate::tests::sun as SUNT;

/// Run all functional tests.
pub fn run_tests() {
    run_datetime_tests();

    run_coordinate_tests();

    run_sun_tests();

    run_planet_tests();

    run_comet_tests();

    run_binary_tests();

    run_moon_tests();

    run_eclipse_tests();
}

pub fn run_datetime_tests() {
    DTT::test_easter(4, 20, 2003);
    DTT::test_day_numbers();

    let mut test_civil_time = DTT::TestCivilTimeScaffold {
        civil_hours: 18.0,
        civil_minutes: 31.0,
        civil_seconds: 27.0,
    };
    test_civil_time.test_civil_time_to_decimal_hours();
    test_civil_time.test_decimal_hours_to_civil_time();
    test_civil_time.test_decimal_time_parts();

    let mut test_local_civil_time = DTT::TestLocalCivilTimeScaffold {
        lct_hours: 3.0,
        lct_minutes: 37.0,
        lct_seconds: 0.0,
        is_daylight_savings: true,
        zone_correction: 4,
        local_day: 1.0,
        local_month: 7,
        local_year: 2013,
    };
    test_local_civil_time.test_local_civil_time_to_universal_time();
    test_local_civil_time.test_universal_time_to_local_civil_time();

    let mut test_universal_time_sidereal_time = DTT::TestUniversalTimeSiderealTimeScaffold {
        ut_hours: 14.0,
        ut_minutes: 36.0,
        ut_seconds: 51.67,
        gw_day: 22.0,
        gw_month: 4,
        gw_year: 1980,
    };
    test_universal_time_sidereal_time.test_universal_time_to_greenwich_sidereal_time();
    test_universal_time_sidereal_time.test_greenwich_sidereal_time_to_universal_time();

    let mut test_greenwich_sidereal_local_sidereal =
        DTT::TestGreenwichSiderealLocalSiderealScaffold {
            gst_hours: 4.0,
            gst_minutes: 40.0,
            gst_seconds: 5.23,
            geographical_longitude: -64.0,
        };
    test_greenwich_sidereal_local_sidereal.test_greenwich_sidereal_time_to_local_sidereal_time();
    test_greenwich_sidereal_local_sidereal.test_local_sidereal_time_to_greenwich_sidereal_time();

    DTT::test_julian_date_to_day_of_week();
}

pub fn run_coordinate_tests() {
    let mut test_angle_decimal_degrees = CST::TestAngleDecimalDegreesScaffold {
        degrees: 182.0,
        minutes: 31.0,
        seconds: 27.0,
    };
    test_angle_decimal_degrees.test_angle_to_decimal_degrees();
    test_angle_decimal_degrees.test_decimal_degrees_to_angle();

    let mut test_right_ascension_hour_angle = CST::TestRightAscensionHourAngleScaffold {
        ra_hours: 18.0,
        ra_minutes: 32.0,
        ra_seconds: 21.0,
        lct_hours: 14.0,
        lct_minutes: 36.0,
        lct_seconds: 51.67,
        is_daylight_saving: false,
        zone_correction: -4,
        local_day: 22.0,
        local_month: 4,
        local_year: 1980,
        geographical_longitude: -64.0,
    };
    test_right_ascension_hour_angle.test_right_ascension_to_hour_angle();
    test_right_ascension_hour_angle.test_hour_angle_to_right_ascension();

    let mut test_equatorial_horizon = CST::TestEquatorialHorizonScaffold {
        hour_angle_hours: 5.0,
        hour_angle_minutes: 51.0,
        hour_angle_seconds: 44.0,
        declination_degrees: 23.0,
        declination_minutes: 13.0,
        declination_seconds: 10.0,
        geographical_latitude: 52.0,
    };
    test_equatorial_horizon.test_equatorial_coordinates_to_horizon_coordinates();
    test_equatorial_horizon.test_horizon_coordinates_to_equatorial_coordinates();

    let mut test_ecliptic = CST::TestEclipticScaffold {
        ecliptic_longitude_degrees: 139.0,
        ecliptic_longitude_minutes: 41.0,
        ecliptic_longitude_seconds: 10.0,
        ecliptic_latitude_degrees: 4.0,
        ecliptic_latitude_minutes: 52.0,
        ecliptic_latitude_seconds: 31.0,
        greenwich_day: 6.0,
        greenwich_month: 7,
        greenwich_year: 2009,
    };
    test_ecliptic.test_mean_obliquity_of_the_ecliptic();
    test_ecliptic.test_ecliptic_coordinate_to_equatorial_coordinate();
    test_ecliptic.test_equatorial_coordinate_to_ecliptic_coordinate();

    let mut test_galactic = CST::TestGalacticScaffold {
        ra_hours: 10.0,
        ra_minutes: 21.0,
        ra_seconds: 0.0,
        dec_degrees: 10.0,
        dec_minutes: 3.0,
        dec_seconds: 11.0,
    };
    test_galactic.test_equatorial_coordinate_to_galactic_coordinate();
    test_galactic.test_galactic_coordinate_to_equatorial_coordinate();

    CST::test_angle_between_two_objects(
        5.0,
        13.0,
        31.7,
        -8.0,
        13.0,
        30.0,
        6.0,
        44.0,
        13.4,
        -16.0,
        41.0,
        11.0,
        "H".to_string(),
    );

    CST::test_rising_and_setting(
        23.0, 39.0, 20.0, 21.0, 42.0, 0.0, 24.0, 8, 2010, 64.0, 30.0, 0.5667,
    );

    CST::test_correct_for_precession(
        9.0, 10.0, 43.0, 14.0, 23.0, 25.0, 0.923, 1, 1950, 1.0, 6, 1979,
    );

    CST::test_nutation_in_ecliptic_longitude_and_obliquity(1.0, 9, 1988);

    CST::test_correct_for_aberration(
        0.0, 0.0, 0.0, 8.0, 9, 1988, 352.0, 37.0, 10.1, -1.0, 32.0, 56.4,
    );

    CST::test_atmospheric_refraction(
        23.0,
        14.0,
        0.0,
        40.0,
        10.0,
        0.0,
        "TRUE".to_string(),
        0.17,
        51.2036110,
        0,
        0,
        23.0,
        3,
        1987,
        1.0,
        1.0,
        24.0,
        1012.0,
        21.7,
    );

    CST::test_corrections_for_geocentric_parallax(
        22.0,
        35.0,
        19.0,
        -7.0,
        41.0,
        13.0,
        "TRUE".to_string(),
        1.019167,
        -100.0,
        50.0,
        60.0,
        0,
        -6,
        26.0,
        2,
        1979,
        10.0,
        45.0,
        0.0,
    );

    CST::test_heliographic_coordinates(220.0, 10.5, 1.0, 5, 1988);

    CST::test_carrington_rotation_number(27.0, 1, 1975);

    let mut test_selenographic = CST::TestSelenographicScaffold {
        gwdate_day: 1.0,
        gwdate_month: 5,
        gwdate_year: 1988,
    };
    test_selenographic.test_selenographic_coordinates_1();
    test_selenographic.test_selenographic_coordinates_2();
}

pub fn run_sun_tests() {
    SUNT::test_approximate_position_of_sun(0.0, 0.0, 0.0, 27 as f64, 7, 2003, false, 0);

    SUNT::test_precise_position_of_sun(0.0, 0.0, 0.0, 27 as f64, 7, 1988, false, 0);

    SUNT::test_sun_distance_and_angular_size(0.0, 0.0, 0.0, 27 as f64, 7, 1988, false, 0);

    SUNT::test_sunrise_and_sunset(10.0, 3, 1986, false, -5, -71.05, 42.37);

    SUNT::test_morning_and_evening_twilight(
        7.0,
        9,
        1979,
        false,
        0,
        0.0,
        52.0,
        pa_types::TwilightType::Astronomical,
    );

    SUNT::test_equation_of_time(27.0, 7, 2010);

    SUNT::test_solar_elongation(10.0, 6.0, 45.0, 11.0, 57.0, 27.0, 27.8333333, 7, 2010);
}

pub fn run_planet_tests() {
    let mut test_planet_position = PLANETT::TestPositionOfPlanetScaffold {
        lct_hour: 0.0,
        lct_minute: 0.0,
        lct_second: 0.0,
        is_daylight_saving: false,
        zone_correction_hours: 0,
        local_date_day: 22.0,
        local_date_month: 11,
        local_date_year: 2003,
        planet_name: "Jupiter".to_string(),
    };
    test_planet_position.test_approximate_position_of_planet();
    test_planet_position.test_precise_position_of_planet();
    test_planet_position.test_visual_aspects_of_a_planet();
}

pub fn run_comet_tests() {
    COMT::test_position_of_elliptical_comet(
        0.0,
        0.0,
        0.0,
        false,
        0,
        1.0,
        1,
        1984,
        "Halley".to_string(),
    );

    COMT::test_position_of_parabolic_comet(
        0.0,
        0.0,
        0.0,
        false,
        0,
        25.0,
        12,
        1977,
        "Kohler".to_string(),
    );
}

pub fn run_binary_tests() {
    BINS::test_binary_star_orbit(1.0, 1, 1980, "eta-Cor".to_string());
}

pub fn run_moon_tests() {
    let mut test_moon_position_and_info = MOONT::TestMoonPositionInfoScaffold {
        lct_hour: 0.0,
        lct_min: 0.0,
        lct_sec: 0.0,
        is_daylight_saving: false,
        zone_correction_hours: 0,
        local_date_day: 1.0,
        local_date_month: 9,
        local_date_year: 2003,
    };
    test_moon_position_and_info.test_approximate_position_of_moon();
    test_moon_position_and_info.test_precise_position_of_moon();
    test_moon_position_and_info.test_moon_phase();
    test_moon_position_and_info.test_moon_dist_ang_diam_hor_parallax();

    MOONT::test_times_of_new_moon_and_full_moon(false, 0, 1.0, 9, 2003);
    MOONT::test_moonrise_and_moonset(6.0, 3, 1986, false, -5, -71.05, 42.3667);
}

pub fn run_eclipse_tests() {
    let mut test_lunar_eclipse = ECL::TestLunarEclipseScaffold {
        local_date_day: 1.0,
        local_date_month: 4,
        local_date_year: 2015,
        is_daylight_saving: false,
        zone_correction_hours: 10,
    };
    test_lunar_eclipse.test_lunar_eclipse_occurrence();
    test_lunar_eclipse.test_lunar_eclipse_circumstances();

    let mut test_solar_eclipse = ECL::TestSolarEclipseScaffold {
        local_date_day: 1.0,
        local_date_month: 4,
        local_date_year: 2015,
        is_daylight_saving: false,
        zone_correction_hours: 0,
    };
    test_solar_eclipse.test_solar_eclipse_occurrence();

    ECL::test_solar_eclipse_circumstances(20.0, 3, 2015, false, 0, 0.0, 68.65);
}
