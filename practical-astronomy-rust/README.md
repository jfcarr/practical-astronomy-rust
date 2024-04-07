# Practical Astronomy in Rust

Algorithms from "[Practical Astronomy with your Calculator or Spreadsheet](https://www.amazon.com/Practical-Astronomy-your-Calculator-Spreadsheet/dp/1108436072)" by Peter Duffett-Smith, implemented in Rust.  API documentation is published [here](https://jfcarr.github.io/practical-astronomy-rust/).

If you're interested in this topic, please buy the book!  It provides far more detail and context.

## Quick Start

We'll calculate the circumstances of the April 8 solar eclipse for West Alexandria, Ohio, using the published crate.  (You can tweak the inputs, if you like)

First, open a terminal and create a binary application:

```bash
cargo new pa_solar_test
```

Switch to the new project directory, and add a reference to the Practical Astronomy crate:

```bash
cargo add practical-astronomy-rust
```

Then, edit `main.rs` and update it to look like this:

```rust
use practical_astronomy_rust::eclipses as ECL;

fn main() {
    // Input values
    let local_date_day: f64 = 8.0;
    let local_date_month: u32 = 4;
    let local_date_year: u32 = 2024;
    let is_daylight_saving: bool = true;
    let zone_correction_hours: i32 = 5;
    let geog_longitude_deg: f64 = -84.53639;
    let geog_latitude_deg: f64 = 39.74722;

    // Calculate the circumstances of the eclipse
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

    // Results are in Universal Time, so lets adjust them for local
    let adjustment_value: f64 = if is_daylight_saving {
        (zone_correction_hours as f64) - 1.0
    } else {
        zone_correction_hours as f64
    };

    let ut_first_contact_hour_adj: f64 = ut_first_contact_hour - adjustment_value;
    let ut_mid_eclipse_hour_adj: f64 = ut_mid_eclipse_hour - adjustment_value;
    let ut_last_contact_hour_adj: f64 = ut_last_contact_hour - adjustment_value;

    // Display the results
    println!("Solar eclipse circumstances:\n\t[Local Date] {}/{}/{}\n\t[DST?] {}\n\t[Zone Correction] {} hours\n\t[Geographical Longitude/Latitude] {} degrees / {} degrees\n\t=\n\t[Certain Date] {}/{}/{}\n\t[First Contact] {}:{}\n\t[Mid Eclipse] {}:{}\n\t[Last Contact] {}:{}\n\t[Magnitude] {}", local_date_month, local_date_day, local_date_year, is_daylight_saving, zone_correction_hours, geog_longitude_deg, geog_latitude_deg, solar_eclipse_certain_date_month, solar_eclipse_certain_date_day, solar_eclipse_certain_date_year, ut_first_contact_hour_adj, ut_first_contact_minutes, ut_mid_eclipse_hour_adj, ut_mid_eclipse_minutes, ut_last_contact_hour_adj, ut_last_contact_minutes, eclipse_magnitude);
}
```

Save the file, and run it:

```bash
cargo run
```

You should see this:

```
Solar eclipse circumstances:
	[Local Date] 4/8/2024
	[DST?] true
	[Zone Correction] 5 hours
	[Geographical Longitude/Latitude] -84.53639 degrees / 39.74722 degrees
	=
	[Certain Date] 4/8/2024
	[First Contact] 13:55
	[Mid Eclipse] 15:11
	[Last Contact] 16:27
	[Magnitude] 1.006
```

## Library Functions - Status

### Date/Time

- [x] Calculate -> Date of Easter
- [x] Convert -> Civil Date to Day Number
- [x] Convert -> Civil Time <-> Decimal Hours
- [x] Extract -> Hour, Minutes, and Seconds parts of Decimal Hours
- [x] Convert -> Local Civil Time <-> Universal Time
- [x] Convert -> Universal Time <-> Greenwich Sidereal Time
- [x] Convert -> Greenwich Sidereal Time <-> Local Sidereal Time
- [x] Calculate -> Day of Week for Julian Date

### Coordinates

- [x] Convert -> Angle <-> Decimal Degrees
- [x] Convert -> Right Ascension <-> Hour Angle
- [x] Convert -> Equatorial Coordinates <-> Horizon Coordinates
- [x] Calculate -> Obliquity of the Ecliptic
- [x] Convert -> Ecliptic Coordinates <-> Equatorial Coordinates
- [x] Convert -> Equatorial Coordinates <-> Galactic Coordinates
- [x] Calculate -> Angle between two objects
- [x] Calculate -> Rising and Setting times for an object
- [x] Calculate -> Precession (corrected coordinates between two epochs)
- [x] Calculate -> Nutation (in ecliptic longitude and obliquity) for a Greenwich date
- [x] Calculate -> Effects of aberration for ecliptic coordinates
- [x] Calculate -> RA and Declination values, corrected for atmospheric refraction
- [x] Calculate -> RA and Declination values, corrected for geocentric parallax
- [x] Calculate -> Heliographic coordinates
- [x] Calculate -> Carrington rotation number
- [x] Calculate -> Selenographic (lunar) coordinates (sub-Earth and sub-Solar)

### The Sun

- [x] Calculate -> Approximate and precise positions of the Sun
- [x] Calculate -> Sun's distance and angular size
- [x] Calculate -> Local sunrise and sunset
- [x] Calculate -> Morning and evening twilight
- [x] Calculate -> Equation of time
- [x] Calculate -> Solar elongation

### Planets

- [x] Calculate -> Approximate position of planet
- [x] Calculate -> Precise position of planet
- [x] Calculate -> Visual aspects of planet (distance, angular diameter, phase, light time, position angle of bright limb, and apparent magnitude)
- [x] Calculate -> Position of comet (elliptical and parabolic)
- [x] Calculate -> Binary star orbit data

### The Moon

- [x] Calculate -> Approximate and precise position of Moon
- [x] Calculate -> Moon phase and position angle of bright limb
- [x] Calculate -> Times of new Moon and full Moon
- [x] Calculate -> Moon's distance, angular diameter, and horizontal parallax
- [x] Calculate -> Local moonrise and moonset

### Eclipses

- [x] Calculate -> Lunar eclipse occurrence and circumstances
- [x] Calculate -> Solar eclipse occurrence and circumstances
