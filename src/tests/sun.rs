use crate::lib::sun as CS;
use crate::lib::types as pa_types;

pub fn test_approximate_position_of_sun(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
) {
    let (sun_ra_hour, sun_ra_min, sun_ra_sec, sun_dec_deg, sun_dec_min, sun_dec_sec) =
        CS::approximate_position_of_sun(
            lct_hours,
            lct_minutes,
            lct_seconds,
            local_day,
            local_month,
            local_year,
            is_daylight_saving,
            zone_correction,
        );

    println!(
		"Approximate position of sun: [Local Civil Time] {}:{}:{} [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} = [Sun] [RA] {}h {}m {}s [Dec] {}d {}m {}s",
		lct_hours,
		lct_minutes,
		lct_seconds,
		local_month,
		local_day,
		local_year,
		is_daylight_saving,
		zone_correction,
		sun_ra_hour,
		sun_ra_min,
		sun_ra_sec,
		sun_dec_deg,
		sun_dec_min,
		sun_dec_sec
	);

    assert_eq!(sun_ra_hour, 8.0, "Sun RA Hour");
    assert_eq!(sun_ra_min, 23.0, "Sun RA Minutes");
    assert_eq!(sun_ra_sec, 33.73, "Sun RA Seconds");
    assert_eq!(sun_dec_deg, 19.0, "Sun Dec Degrees");
    assert_eq!(sun_dec_min, 21.0, "Sun Dec Minutes");
    assert_eq!(sun_dec_sec, 14.33, "Sun Dec Seconds");
}

pub fn test_precise_position_of_sun(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
) {
    let (sun_ra_hour, sun_ra_min, sun_ra_sec, sun_dec_deg, sun_dec_min, sun_dec_sec) =
        CS::precise_position_of_sun(
            lct_hours,
            lct_minutes,
            lct_seconds,
            local_day,
            local_month,
            local_year,
            is_daylight_saving,
            zone_correction,
        );

    println!(
		"Precise position of sun: [Local Civil Time] {}:{}:{} [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} = [Sun] [RA] {}h {}m {}s [Dec] {}d {}m {}s",
		lct_hours,
		lct_minutes,
		lct_seconds,
		local_month,
		local_day,
		local_year,
		is_daylight_saving,
		zone_correction,
		sun_ra_hour,
		sun_ra_min,
		sun_ra_sec,
		sun_dec_deg,
		sun_dec_min,
		sun_dec_sec
	);

    assert_eq!(sun_ra_hour, 8.0, "Sun RA Hour");
    assert_eq!(sun_ra_min, 26.0, "Sun RA Minutes");
    assert_eq!(sun_ra_sec, 3.83, "Sun RA Seconds");
    assert_eq!(sun_dec_deg, 19.0, "Sun Dec Degrees");
    assert_eq!(sun_dec_min, 12.0, "Sun Dec Minutes");
    assert_eq!(sun_dec_sec, 49.72, "Sun Dec Seconds");
}

pub fn test_sun_distance_and_angular_size(
    lct_hours: f64,
    lct_minutes: f64,
    lct_seconds: f64,
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
) {
    let (sun_dist_km, sun_ang_size_deg, sun_ang_size_min, sun_ang_size_sec) =
        CS::sun_distance_and_angular_size(
            lct_hours,
            lct_minutes,
            lct_seconds,
            local_day,
            local_month,
            local_year,
            is_daylight_saving,
            zone_correction,
        );

    println!(
		"Sun distance and angular size: [Local Civil Time] {}:{}:{} [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} = [Sun] [Distance] {} km [Angular Size] {}d {}m {}s",
		lct_hours,
		lct_minutes,
		lct_seconds,
		local_month,
		local_day,
		local_year,
		is_daylight_saving,
		zone_correction,
		sun_dist_km,
		sun_ang_size_deg,
		sun_ang_size_min,
		sun_ang_size_sec
	);

    assert_eq!(sun_dist_km, 151920130.0, "Sun Distance in km");
    assert_eq!(sun_ang_size_deg, 0.0, "Sun Angular Size Degrees");
    assert_eq!(sun_ang_size_min, 31.0, "Sun Angular Size Minutes");
    assert_eq!(sun_ang_size_sec, 29.93, "Sun Angular Size Seconds");
}

pub fn test_sunrise_and_sunset(
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
    geographical_long_deg: f64,
    geographical_lat_deg: f64,
) {
    let (
        local_sunrise_hour,
        local_sunrise_minute,
        local_sunset_hour,
        local_sunset_minute,
        azimuth_of_sunrise_deg,
        azimuth_of_sunset_deg,
        status,
    ) = CS::sunrise_and_sunset(
        local_day,
        local_month,
        local_year,
        is_daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
    );

    println!(
		"Sunrise and sunset: [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} [Geographical Longitude/Latitude] {}d/{}d = [Sunrise] {}:{} [Sunset] {}:{} [Azimuth of Sunrise/Sunset] {}d/{}d [Status] {}",
		local_month,
		local_day,
		local_year,
		is_daylight_saving,
		zone_correction,
		geographical_long_deg,
		geographical_lat_deg,
		local_sunrise_hour,
		local_sunrise_minute,
		local_sunset_hour,
		local_sunset_minute,
		azimuth_of_sunrise_deg,
		azimuth_of_sunset_deg,
		status
	);

    assert_eq!(local_sunrise_hour, 6.0, "Local Sunrise Hour");
    assert_eq!(local_sunrise_minute, 5.0, "Local Sunrise Minute");
    assert_eq!(local_sunset_hour, 17.0, "Local Sunset Hour");
    assert_eq!(local_sunset_minute, 45.0, "Local Sunset Minute");
    assert_eq!(
        azimuth_of_sunrise_deg, 94.83,
        "Azimuth of Sunrise (degrees)"
    );
    assert_eq!(azimuth_of_sunset_deg, 265.43, "Azimuth of Sunset (degrees)");
    assert_eq!(status, "OK", "Status of Calculation");
}

pub fn test_morning_and_evening_twilight(
    local_day: f64,
    local_month: u32,
    local_year: u32,
    is_daylight_saving: bool,
    zone_correction: i32,
    geographical_long_deg: f64,
    geographical_lat_deg: f64,
    twilight_type: pa_types::TwilightType,
) {
    let display_twilight_type = match twilight_type {
        pa_types::TwilightType::Astronomical => "Astronomical",
        pa_types::TwilightType::Civil => "Civil",
        pa_types::TwilightType::Nautical => "Nautical",
    };

    let (
        am_twilight_begins_hour,
        am_twilight_begins_min,
        pm_twilight_ends_hour,
        pm_twilight_ends_min,
        status,
    ) = CS::morning_and_evening_twilight(
        local_day,
        local_month,
        local_year,
        is_daylight_saving,
        zone_correction,
        geographical_long_deg,
        geographical_lat_deg,
        twilight_type,
    );

    println!(
		"Morning and evening twilight: [Local Date] {}/{}/{} [DST?] {} [Zone Correction] {} [Geographical Longitude/Latitude] {}d/{}d [Twilight Type] {:?} = [Twilight] [Begins] {}:{} [Ends] {}:{} [Status] {}",
		local_month,
		local_day,
		local_year,
		is_daylight_saving,
		zone_correction,
		geographical_long_deg,
		geographical_lat_deg,
		display_twilight_type,
		am_twilight_begins_hour,
		am_twilight_begins_min,
		pm_twilight_ends_hour,
		pm_twilight_ends_min,
		status
	);

    assert_eq!(am_twilight_begins_hour, 3.0, "AM Twilight Begins (hour)");
    assert_eq!(am_twilight_begins_min, 17.0, "AM Twilight Begins (minute)");
    assert_eq!(pm_twilight_ends_hour, 20.0, "PM Twilight Ends (hour)");
    assert_eq!(pm_twilight_ends_min, 37.0, "PM Twilight Ends (minute)");
    assert_eq!(status, "OK", "Status of Calculation");
}

pub fn test_equation_of_time(gwdate_day: f64, gwdate_month: u32, gwdate_year: u32) {
    let (equation_of_time_min, equation_of_time_sec) =
        CS::equation_of_time(gwdate_day, gwdate_month, gwdate_year);

    println!(
        "Equation of time: [Greenwich Date] {}/{}/{} = [Time] {}:{}",
        gwdate_month, gwdate_day, gwdate_year, equation_of_time_min, equation_of_time_sec
    );

    assert_eq!(equation_of_time_min, 6.0, "Equation of Time (min)");
    assert_eq!(equation_of_time_sec, 31.52, "Equation of Time (sec)");
}

pub fn test_solar_elongation(
    ra_hour: f64,
    ra_min: f64,
    ra_sec: f64,
    dec_deg: f64,
    dec_min: f64,
    dec_sec: f64,
    gwdate_day: f64,
    gwdate_month: u32,
    gwdate_year: u32,
) {
    let solar_elongation_deg = CS::solar_elongation(
        ra_hour,
        ra_min,
        ra_sec,
        dec_deg,
        dec_min,
        dec_sec,
        gwdate_day,
        gwdate_month,
        gwdate_year,
    );

    println!(
		"Solar elongation: [RA] {}h {}m {}s [Declination] {}d {}m {}s [Greenwich Date] {}/{}/{} = [Solar Elongation] {}d",
		ra_hour,
		ra_min,
		ra_sec,
		dec_deg,
		dec_min,
		dec_sec,
		gwdate_month,
		gwdate_day,
		gwdate_year,
		solar_elongation_deg);

    assert_eq!(solar_elongation_deg, 24.78, "Solar Elongation (degrees)");
}
