/// Info about a comet (elliptical):
/// * `name` -- Name of comet.
/// * `epoch` -- Epoch of the perihelion.
/// * `peri` -- Longitude of the perihelion.
/// * `node` -- Longitude of the ascending node.
/// * `period` -- Period of the orbit.
/// * `axis` -- Semi-major axis of the orbit.
/// * `ecc` -- Eccentricity of the orbit.
/// * `incl` -- Inclination of the orbit.
pub struct CometInfoElliptical {
    pub name: String,
    pub epoch: f64,
    pub peri: f64,
    pub node: f64,
    pub period: f64,
    pub axis: f64,
    pub ecc: f64,
    pub incl: f64,
}

/// Info about a comet (parabolic):
/// * epoch_peri_day -- Epoch of the perihelion (day)
/// * epoch_peri_month -- Epoch of the perihelion (month)
/// * epoch_peri_year -- Epoch of the perihelion (year)
/// * arg_peri -- Longitude of the perihelion (degrees)
/// * node -- Longitude of the ascending node (degrees)
/// * peri_dist -- Distance at perihelion (AU)
/// * incl -- Orbital inclination (degrees)
pub struct CometInfoParabolic {
    pub name: String,
    pub epoch_peri_day: f64,
    pub epoch_peri_month: u32,
    pub epoch_peri_year: u32,
    pub arg_peri: f64,
    pub node: f64,
    pub peri_dist: f64,
    pub incl: f64,
}

/// Retrieve info about a comet (elliptical).
///
/// ## Returns
/// * CometInfoElliptical structure.
/// * status
pub fn get_comet_info_elliptical_vector(comet_name: String) -> (CometInfoElliptical, String) {
    let mut comet_elliptical_vector: Vec<CometInfoElliptical> = Vec::new();

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Encke".to_string(),
        epoch: 1974.32,
        peri: 160.1,
        node: 334.2,
        period: 3.3,
        axis: 2.21,
        ecc: 0.85,
        incl: 12.0,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Temple 2".to_string(),
        epoch: 1972.87,
        peri: 310.2,
        node: 119.3,
        period: 5.26,
        axis: 3.02,
        ecc: 0.55,
        incl: 12.5,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Haneda-Campos".to_string(),
        epoch: 1978.77,
        peri: 12.02,
        node: 131.7,
        period: 5.37,
        axis: 3.07,
        ecc: 0.64,
        incl: 5.81,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Schwassmann-Wachmann 2".to_string(),
        epoch: 1974.7,
        peri: 123.3,
        node: 126.0,
        period: 6.51,
        axis: 3.49,
        ecc: 0.39,
        incl: 3.7,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Borrelly".to_string(),
        epoch: 1974.36,
        peri: 67.8,
        node: 75.1,
        period: 6.76,
        axis: 3.58,
        ecc: 0.63,
        incl: 30.2,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Whipple".to_string(),
        epoch: 1970.77,
        peri: 18.2,
        node: 188.4,
        period: 7.47,
        axis: 3.82,
        ecc: 0.35,
        incl: 10.2,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Oterma".to_string(),
        epoch: 1958.44,
        peri: 150.0,
        node: 155.1,
        period: 7.88,
        axis: 3.96,
        ecc: 0.14,
        incl: 4.0,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Schaumasse".to_string(),
        epoch: 1960.29,
        peri: 138.1,
        node: 86.2,
        period: 8.18,
        axis: 4.05,
        ecc: 0.71,
        incl: 12.0,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Comas Sola".to_string(),
        epoch: 1969.83,
        peri: 102.9,
        node: 62.8,
        period: 8.55,
        axis: 4.18,
        ecc: 0.58,
        incl: 13.4,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Schwassmann-Wachmann 1".to_string(),
        epoch: 1974.12,
        peri: 334.1,
        node: 319.6,
        period: 15.03,
        axis: 6.09,
        ecc: 0.11,
        incl: 9.7,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Neujmin 1".to_string(),
        epoch: 1966.94,
        peri: 334.0,
        node: 347.2,
        period: 17.93,
        axis: 6.86,
        ecc: 0.78,
        incl: 15.0,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Crommelin".to_string(),
        epoch: 1956.82,
        peri: 86.4,
        node: 250.4,
        period: 27.89,
        axis: 9.17,
        ecc: 0.92,
        incl: 28.9,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Olbers".to_string(),
        epoch: 1956.46,
        peri: 150.0,
        node: 85.4,
        period: 69.47,
        axis: 16.84,
        ecc: 0.93,
        incl: 44.6,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Pons-Brooks".to_string(),
        epoch: 1954.39,
        peri: 94.2,
        node: 255.2,
        period: 70.98,
        axis: 17.2,
        ecc: 0.96,
        incl: 74.2,
    });

    comet_elliptical_vector.push(CometInfoElliptical {
        name: "Halley".to_string(),
        epoch: 1986.112,
        peri: 170.011,
        node: 58.154,
        period: 76.0081,
        axis: 17.9435,
        ecc: 0.9673,
        incl: 162.2384,
    });

    for i in comet_elliptical_vector {
        if i.name == comet_name {
            return (i, "OK".to_string());
        }
    }

    return (
        CometInfoElliptical {
            name: comet_name,
            epoch: -99.0,
            peri: -99.0,
            node: -99.0,
            period: -99.0,
            axis: -99.0,
            ecc: -99.0,
            incl: -99.0,
        },
        "NotFound".to_string(),
    );
}

/// Retrieve info about a comet (parabolic).
///
/// ## Returns
/// * CometInfoParabolic structure.
/// * status
pub fn get_comet_info_parabolic_vector(comet_name: String) -> (CometInfoParabolic, String) {
    let mut comet_parabolic_vector: Vec<CometInfoParabolic> = Vec::new();

    comet_parabolic_vector.push(CometInfoParabolic {
        name: "Kohler".to_string(),
        epoch_peri_day: 10.5659,
        epoch_peri_month: 11,
        epoch_peri_year: 1977,
        arg_peri: 163.4799,
        node: 181.8175,
        peri_dist: 0.990662,
        incl: 48.7196,
    });

    for i in comet_parabolic_vector {
        if i.name == comet_name {
            return (i, "OK".to_string());
        }
    }

    return (
        CometInfoParabolic {
            name: comet_name,
            epoch_peri_day: 0.0,
            epoch_peri_month: 0,
            epoch_peri_year: 0,
            arg_peri: 0.0,
            node: 0.0,
            peri_dist: 0.0,
            incl: 0.0,
        },
        "NotFound".to_string(),
    );
}
