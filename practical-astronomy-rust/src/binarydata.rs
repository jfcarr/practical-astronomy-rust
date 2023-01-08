/// Info about a binary system:
/// * `name` -- Name of binary system.
/// * `period` -- Period of the orbit.
/// * `epoch_peri` -- Epoch of the perihelion.
/// * `long_peri` -- Longitude of the perihelion.
/// * `ecc` -- Eccentricity of the orbit.
/// * `axis` -- Semi-major axis of the orbit.
/// * `incl` -- Orbital inclination.
/// * `pa_node` -- Position angle of the ascending node.
pub struct BinaryInfo {
    pub name: String,
    pub period: f64,
    pub epoch_peri: f64,
    pub long_peri: f64,
    pub ecc: f64,
    pub axis: f64,
    pub incl: f64,
    pub pa_node: f64,
}

/// Retrieve info about a binary system.
///
/// ## Returns
/// * BinaryInfo structure.
/// * status
pub fn get_binary_info_vector(binary_name: String) -> (BinaryInfo, String) {
    let mut binary_info_vector: Vec<BinaryInfo> = Vec::new();

    binary_info_vector.push(BinaryInfo {
        name: "eta-Cor".to_string(),
        period: 41.623,
        epoch_peri: 1934.008,
        long_peri: 219.907,
        ecc: 0.2763,
        axis: 0.907,
        incl: 59.025,
        pa_node: 23.717,
    });

    binary_info_vector.push(BinaryInfo {
        name: "gamma-Vir".to_string(),
        period: 171.37,
        epoch_peri: 1836.433,
        long_peri: 252.88,
        ecc: 0.8808,
        axis: 3.746,
        incl: 146.05,
        pa_node: 31.78,
    });

    binary_info_vector.push(BinaryInfo {
        name: "eta-Cas".to_string(),
        period: 480.0,
        epoch_peri: 1889.6,
        long_peri: 268.59,
        ecc: 0.497,
        axis: 11.9939,
        incl: 34.76,
        pa_node: 278.42,
    });

    binary_info_vector.push(BinaryInfo {
        name: "zeta-Ori".to_string(),
        period: 1508.6,
        epoch_peri: 2070.6,
        long_peri: 47.3,
        ecc: 0.07,
        axis: 2.728,
        incl: 72.0,
        pa_node: 155.5,
    });

    binary_info_vector.push(BinaryInfo {
        name: "alpha-CMa".to_string(),
        period: 50.09,
        epoch_peri: 1894.13,
        long_peri: 147.27,
        ecc: 0.5923,
        axis: 7.5,
        incl: 136.53,
        pa_node: 44.57,
    });

    binary_info_vector.push(BinaryInfo {
        name: "delta-Gem".to_string(),
        period: 1200.0,
        epoch_peri: 1437.0,
        long_peri: 57.19,
        ecc: 0.11,
        axis: 6.9753,
        incl: 63.28,
        pa_node: 18.38,
    });

    binary_info_vector.push(BinaryInfo {
        name: "alpha-Gem".to_string(),
        period: 420.07,
        epoch_peri: 1965.3,
        long_peri: 261.43,
        ecc: 0.33,
        axis: 6.295,
        incl: 115.94,
        pa_node: 40.47,
    });

    binary_info_vector.push(BinaryInfo {
        name: "aplah-CMi".to_string(),
        period: 40.65,
        epoch_peri: 1927.6,
        long_peri: 269.8,
        ecc: 0.4,
        axis: 4.548,
        incl: 35.7,
        pa_node: 284.3,
    });

    binary_info_vector.push(BinaryInfo {
        name: "alpha-Cen".to_string(),
        period: 79.92,
        epoch_peri: 1955.56,
        long_peri: 231.56,
        ecc: 0.516,
        axis: 17.583,
        incl: 79.24,
        pa_node: 204.868,
    });

    binary_info_vector.push(BinaryInfo {
        name: "alpha Sco".to_string(),
        period: 900.0,
        epoch_peri: 1889.0,
        long_peri: 0.0,
        ecc: 0.0,
        axis: 3.21,
        incl: 86.3,
        pa_node: 273.0,
    });

    for i in binary_info_vector {
        if i.name == binary_name {
            return (i, "OK".to_string());
        }
    }

    return (
        BinaryInfo {
            name: binary_name,
            period: 0.0,
            epoch_peri: 0.0,
            long_peri: 0.0,
            ecc: 0.0,
            axis: 0.0,
            incl: 0.0,
            pa_node: 0.0,
        },
        "NotFound".to_string(),
    );
}
