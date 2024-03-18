// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use std::fmt;

use crate::astronomy::unit::{Angle, Coordinates};

#[repr(transparent)]
pub struct Qiblah(f64);

impl Qiblah {
    #[must_use]
    pub fn new(location_coordinates: &Coordinates) -> Self {
        // Equation from "Spherical Trigonometry For the use
        // of colleges and schools" page 50
        let makkah_coordinates = Coordinates::new(21.422_524_1, 39.826_181_8);
        let term1 =
            (makkah_coordinates.longitude_angle().radians() - location_coordinates.longitude_angle().radians()).sin();
        let term2 =
            makkah_coordinates.latitude_angle().radians().tan() * location_coordinates.latitude_angle().radians().cos();
        let term3 = (makkah_coordinates.longitude_angle().radians() - location_coordinates.longitude_angle().radians())
            .cos()
            * location_coordinates.latitude_angle().radians().sin();
        let term4 = term1.atan2(term2 - term3);

        Self(Angle::from_radians(term4).unwound().degrees)
    }

    #[must_use]
    pub const fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Qiblah {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::from_new_york_city_north_america((40.7128, -74.0059), 58.481_763_5)]
    #[case::from_san_francisco_north_america((37.7749, -122.4194), 18.843_822_245_692_426)]
    #[case::from_washington_dc_north_america((38.9072, -77.0369), 56.560_468_214_635_99)]
    #[case::from_anchorage_north_america((61.2181, -149.9003), 350.883_076_115_985_3)]
    #[case::from_sydney_australia((-33.8688, 151.2093), 277.499_604_448_739_9)]
    #[case::from_auckland_new_zealand((-36.8485, 174.7633), 261.197_326_403_658_45)]
    #[case::from_london_united_kingdom((51.5074, -0.1278),  118.987_218_9)]
    #[case::from_paris_france((48.8566, 2.3522), 119.163_135_421_833_47)]
    #[case::from_oslo_norway((59.9139, 10.7522), 139.027_856_055_375_14)]
    #[case::from_islamabad_pakistan((33.7294, 73.0931), 255.881_615_678_543_6)]
    #[case::from_tokyo_japan((35.6895, 139.6917), 293.020_724_414_411_63)]
    #[case::from_jakarta_indonesia((-6.182_339_95, 106.842_871_54), 295.144_298_382_526_5)]
    fn test_qiblah_direction(#[case] coords: (f64, f64), #[case] expected_angle: f64) {
        let location = Coordinates::from(coords);
        let qiblah = Qiblah::new(&location);

        assert_approx_eq!(f64, qiblah.value(), expected_angle, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_direction_display() {
        let nyc = Coordinates::new(40.7128, -74.0059);
        let qiblah = Qiblah::new(&nyc);
        let actual_value = qiblah.to_string();

        assert!(actual_value.contains("58.4817635"));
    }
}
