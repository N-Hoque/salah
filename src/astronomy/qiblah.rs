// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use std::fmt;

use crate::astronomy::unit::{Angle, Coordinates};

#[derive(Debug)]
pub struct Qiblah(f64);

impl Qiblah {
    #[must_use]
    pub fn new(location_coordinates: Coordinates) -> Self {
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
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn qiblah_direction_from_nyc_in_north_america() {
        let nyc = Coordinates::new(40.7128, -74.0059);
        let qiblah = Qiblah::new(nyc);

        assert_that!(qiblah.value()).is_close_to(58.481_763_5, 0.000_000_1_f64);
    }

    #[test]
    fn qiblah_direction_from_sf_in_north_america() {
        let sf = Coordinates::new(37.7749, -122.4194);
        let qiblah = Qiblah::new(sf);

        assert_approx_eq!(f64, qiblah.value(), 18.843_822_245_692_426, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_direction_from_dc_in_north_america() {
        let dc = Coordinates::new(38.9072, -77.0369);
        let qiblah = Qiblah::new(dc);

        assert_approx_eq!(f64, qiblah.value(), 56.560_468_214_635_99, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_direction_from_anchorage_in_north_america() {
        let dc = Coordinates::new(61.2181, -149.9003);
        let qiblah = Qiblah::new(dc);

        assert_approx_eq!(f64, qiblah.value(), 350.883_076_115_985_3, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_directioon_from_sydney_australia() {
        let sydney = Coordinates::new(-33.8688, 151.2093);
        let qiblah = Qiblah::new(sydney);

        assert_approx_eq!(f64, qiblah.value(), 277.499_604_448_739_9, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_directioon_from_auckland_new_zealand() {
        let auckland = Coordinates::new(-36.8485, 174.7633);
        let qiblah = Qiblah::new(auckland);

        assert_approx_eq!(f64, qiblah.value(), 261.197_326_403_658_45, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_direction_from_london_united_kingdom() {
        let london = Coordinates::new(51.5074, -0.1278);
        let qiblah = Qiblah::new(london);

        assert_that!(qiblah.value()).is_close_to(118.987_218_9, 0.000_000_1_f64);
    }

    #[test]
    fn qiblah_direction_from_paris_france() {
        let paris = Coordinates::new(48.8566, 2.3522);
        let qiblah = Qiblah::new(paris);

        assert_approx_eq!(f64, qiblah.value(), 119.163_135_421_833_47, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_direction_from_oslo_norway() {
        let oslo = Coordinates::new(59.9139, 10.7522);
        let qiblah = Qiblah::new(oslo);

        assert_approx_eq!(f64, qiblah.value(), 139.027_856_055_375_14, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_direction_from_islamabad_pakistan() {
        let islamabad = Coordinates::new(33.7294, 73.0931);
        let qiblah = Qiblah::new(islamabad);

        assert_approx_eq!(f64, qiblah.value(), 255.881_615_678_543_6, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_direction_from_tokyo_japan() {
        let tokyo = Coordinates::new(35.6895, 139.6917);
        let qiblah = Qiblah::new(tokyo);

        assert_approx_eq!(f64, qiblah.value(), 293.020_724_414_411_63, epsilon = 0.000_000_1);
    }

    #[test]
    fn qiblah_direction_from_jakarta_indonesia() {
        let jakarta = Coordinates::new(-6.182_339_95, 106.842_871_54);
        let qiblah = Qiblah::new(jakarta);

        assert_that!(qiblah.value()).is_close_to(295.144_298_382_526_5, 0.000_000_1_f64);
    }

    #[test]
    fn qiblah_direction_display() {
        let nyc = Coordinates::new(40.7128, -74.0059);
        let qiblah = Qiblah::new(nyc);
        let actual_value = qiblah.to_string();

        assert!(actual_value.contains("58.4817635"));
    }
}
