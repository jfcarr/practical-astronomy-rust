/// Info about a planet:
/// * `name` -- Name of planet.
/// * `tp` -- Period of orbit.
/// * `long` -- Longitude at the epoch.
/// * `peri` -- Longitude of the perihelion.
/// * `ecc` -- Eccentricity of the orbit.
/// * `axis` -- Semi-major axis of the orbit.
/// * `incl` -- Orbital inclination.
/// * `node` -- Longitude of the ascending node.
/// * `theta0` -- Angular diameter at 1 AU.
/// * `v0` -- Visual magnitude at 1 AU.
pub struct PlanetInfo {
    pub name: String,
    pub tp: f64,
    pub long: f64,
    pub peri: f64,
    pub ecc: f64,
    pub axis: f64,
    pub incl: f64,
    pub node: f64,
    pub theta0: f64,
    pub v0: f64,
}

/// Retrieve info about a planet.
///
/// ## Returns
/// PlanetInfo structure.
pub fn get_planet_info_vector(planet_name: String) -> (PlanetInfo, String) {
    let mut planet_vector: Vec<PlanetInfo> = Vec::new();

    planet_vector.push(PlanetInfo {
        name: "Mercury".to_string(),
        tp: 0.24085,
        long: 75.5671,
        peri: 77.612,
        ecc: 0.205627,
        axis: 0.387098,
        incl: 7.0051,
        node: 48.449,
        theta0: 6.74,
        v0: -0.42,
    });
    planet_vector.push(PlanetInfo {
        name: "Venus".to_string(),
        tp: 0.615207,
        long: 272.30044,
        peri: 131.54,
        ecc: 0.006812,
        axis: 0.723329,
        incl: 3.3947,
        node: 76.769,
        theta0: 16.92,
        v0: -4.4,
    });
    planet_vector.push(PlanetInfo {
        name: "Earth".to_string(),
        tp: 0.999996,
        long: 99.556772,
        peri: 103.2055,
        ecc: 0.016671,
        axis: 0.999985,
        incl: -99.0,
        node: -99.0,
        theta0: -99.0,
        v0: -99.0,
    });
    planet_vector.push(PlanetInfo {
        name: "Mars".to_string(),
        tp: 1.880765,
        long: 109.09646,
        peri: 336.217,
        ecc: 0.093348,
        axis: 1.523689,
        incl: 1.8497,
        node: 49.632,
        theta0: 9.36,
        v0: -1.52,
    });
    planet_vector.push(PlanetInfo {
        name: "Jupiter".to_string(),
        tp: 11.857911,
        long: 337.917132,
        peri: 14.6633,
        ecc: 0.048907,
        axis: 5.20278,
        incl: 1.3035,
        node: 100.595,
        theta0: 196.74,
        v0: -9.4,
    });
    planet_vector.push(PlanetInfo {
        name: "Saturn".to_string(),
        tp: 29.310579,
        long: 172.398316,
        peri: 89.567,
        ecc: 0.053853,
        axis: 9.51134,
        incl: 2.4873,
        node: 113.752,
        theta0: 165.6,
        v0: -8.88,
    });
    planet_vector.push(PlanetInfo {
        name: "Uranus".to_string(),
        tp: 84.039492,
        long: 356.135400,
        peri: 172.884833,
        ecc: 0.046321,
        axis: 19.21814,
        incl: 0.773059,
        node: 73.926961,
        theta0: 65.8,
        v0: -7.19,
    });
    planet_vector.push(PlanetInfo {
        name: "Neptune".to_string(),
        tp: 165.845392,
        long: 326.895127,
        peri: 23.07,
        ecc: 0.010483,
        axis: 30.1985,
        incl: 1.7673,
        node: 131.879,
        theta0: 62.2,
        v0: -6.87,
    });

    for i in planet_vector {
        if i.name == planet_name {
            return (i, "OK".to_string());
        }
    }

    return (
        PlanetInfo {
            name: planet_name,
            tp: -99.0,
            long: -99.0,
            peri: -99.0,
            ecc: -99.0,
            axis: -99.0,
            incl: -99.0,
            node: -99.0,
            theta0: -99.0,
            v0: -99.0,
        },
        "NotFound".to_string(),
    );
}
