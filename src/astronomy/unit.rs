// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use std::ops::{Add, Div, Mul, Sub};

use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike};
use serde::{Deserialize, Serialize};

use crate::{astronomy::ops, models::rounding::Rounding};

pub trait Normalize {
    fn normalized_to_scale(&self, max: f64) -> f64;
}

impl Normalize for f64 {
    fn normalized_to_scale(&self, max: f64) -> f64 {
        max.mul_add(-(self / max).floor(), *self)
    }
}

/// Convenience methods for the `DateTime` type.
pub trait Stride {
    #[must_use]
    fn tomorrow(&self) -> Self;
    #[must_use]
    fn yesterday(&self) -> Self;
    #[must_use]
    fn julian_day(&self) -> f64;
    #[must_use]
    fn adjust_time(&self, minutes: i64) -> Self;
    #[must_use]
    fn rounded_minute(&self, rounding: Rounding) -> Self;
}

impl<Tz: TimeZone> Stride for DateTime<Tz> {
    /// Returns the date/time for tomorrow.
    fn tomorrow(&self) -> Self {
        let ordinal = self.ordinal() + 1;

        self.with_ordinal(ordinal)
            .or_else(|| self.with_year(self.year() + 1).and_then(|x| x.with_ordinal(1)))
            .unwrap()
    }

    /// Returns the date/time for yesterday.
    fn yesterday(&self) -> Self {
        let ordinal = self.ordinal() - 1;

        self.with_ordinal(ordinal)
            .or_else(|| {
                self.with_year(self.year() - 1)
                    .and_then(|x| x.with_month(12))
                    .and_then(|x| x.with_day(31))
            })
            .unwrap()
    }

    /// Returns the Julian day.
    fn julian_day(&self) -> f64 {
        ops::julian_day(self.year(), self.month() as i32, self.day() as i32, 0.0)
    }

    fn rounded_minute(&self, rounding: Rounding) -> Self {
        let adjusted = self.clone();
        let seconds = adjusted.second();

        match rounding {
            Rounding::Nearest => {
                let rounded = (f64::from(seconds) / 60.0).round() as i64;
                let adjusted_seconds = i64::from(seconds);

                if rounded == 1 {
                    adjusted + Duration::try_seconds(60 - adjusted_seconds).unwrap()
                } else {
                    adjusted + Duration::try_seconds(-adjusted_seconds).unwrap()
                }
            }
            Rounding::Up => {
                let adjusted_seconds = i64::from(seconds);

                adjusted + Duration::try_seconds(60 - adjusted_seconds).unwrap()
            }
            Rounding::None => adjusted,
        }
    }

    fn adjust_time(&self, minutes: i64) -> Self {
        let some_date = self.clone();
        some_date
            .checked_add_signed(Duration::try_seconds(minutes * 60).unwrap())
            .unwrap()
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Angle {
    pub degrees: f64,
}

impl Angle {
    pub const fn new(degrees: f64) -> Self {
        Self { degrees }
    }

    pub fn from_radians(radians: f64) -> Self {
        Self {
            degrees: radians.to_degrees(),
        }
    }

    pub fn radians(self) -> f64 {
        self.degrees.to_radians()
    }

    pub fn unwound(self) -> Self {
        Self {
            degrees: self.degrees.normalized_to_scale(360.0),
        }
    }

    pub fn quadrant_shifted(self) -> Self {
        if self.degrees >= -180.0 && self.degrees <= 180.0 {
            // Nothing to do. Already initialized
            // to the default value.
            self
        } else {
            let value = 360.0f64.mul_add(-(self.degrees / 360.0).round(), self.degrees);
            Self { degrees: value }
        }
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            degrees: self.degrees + rhs.degrees,
        }
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            degrees: self.degrees - rhs.degrees,
        }
    }
}

impl Mul for Angle {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            degrees: self.degrees * rhs.degrees,
        }
    }
}

impl Div for Angle {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        assert!(rhs.degrees != 0.0, "Cannot divide by zero.");

        Self {
            degrees: self.degrees / rhs.degrees,
        }
    }
}

/// The latitude and longitude associated with a location.
/// Both latiude and longitude values are specified in degrees.
#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    #[must_use]
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}

impl From<(f64, f64)> for Coordinates {
    fn from((latitude, longitude): (f64, f64)) -> Self {
        Self::new(latitude, longitude)
    }
}

impl Coordinates {
    #[must_use]
    pub const fn latitude_angle(&self) -> Angle {
        Angle::new(self.latitude)
    }

    #[must_use]
    pub const fn longitude_angle(&self) -> Angle {
        Angle::new(self.longitude)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use chrono::Utc;
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn angle_conversion_from_radians() {
        assert_approx_eq!(f64, Angle::from_radians(PI).degrees, 180.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, Angle::from_radians(PI / 2.0).degrees, 90.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn angle_conversion_degrees_to_radians() {
        assert_approx_eq!(f64, Angle::new(180.0).radians(), PI, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, Angle::new(90.0).radians(), PI / 2.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn normalize_value() {
        assert_approx_eq!(f64, 2.0_f64.normalized_to_scale(-5.0), -3.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, (-4.0_f64).normalized_to_scale(-5.0), -4.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, (-6.0_f64).normalized_to_scale(-5.0), -1.0, epsilon = 0.000_000_1);

        assert_approx_eq!(f64, (-1.0_f64).normalized_to_scale(24.0), 23.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, 1.0_f64.normalized_to_scale(24.0), 1.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, 49.0_f64.normalized_to_scale(24.0), 1.0, epsilon = 0.000_000_1);

        assert_approx_eq!(f64, 361.0_f64.normalized_to_scale(360.0), 1.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, 360.0_f64.normalized_to_scale(360.0), 0.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, 259.0_f64.normalized_to_scale(360.0), 259.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, 2592.0_f64.normalized_to_scale(360.0), 72.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn angle_unwound() {
        assert_approx_eq!(f64, Angle::new(-45.0).unwound().degrees, 315.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, Angle::new(361.0).unwound().degrees, 1.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, Angle::new(360.0).unwound().degrees, 0.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, Angle::new(259.0).unwound().degrees, 259.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, Angle::new(2592.0).unwound().degrees, 72.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn closest_angle() {
        assert_approx_eq!(
            f64,
            Angle::new(360.0).quadrant_shifted().degrees,
            0.0,
            epsilon = 0.000_000_1
        );
        assert_approx_eq!(
            f64,
            Angle::new(361.0).quadrant_shifted().degrees,
            1.0,
            epsilon = 0.000_000_1
        );
        assert_approx_eq!(
            f64,
            Angle::new(1.0).quadrant_shifted().degrees,
            1.0,
            epsilon = 0.000_000_1
        );
        assert_approx_eq!(
            f64,
            Angle::new(-1.0).quadrant_shifted().degrees,
            -1.0,
            epsilon = 0.000_000_1
        );
        assert_approx_eq!(
            f64,
            Angle::new(-181.0).quadrant_shifted().degrees,
            179.0,
            epsilon = 0.000_000_1
        );
        assert_approx_eq!(
            f64,
            Angle::new(180.0).quadrant_shifted().degrees,
            180.0,
            epsilon = 0.000_000_1
        );
        assert_approx_eq!(
            f64,
            Angle::new(359.0).quadrant_shifted().degrees,
            -1.0,
            epsilon = 0.000_000_1
        );
        assert_approx_eq!(
            f64,
            Angle::new(-359.0).quadrant_shifted().degrees,
            1.0,
            epsilon = 0.000_000_1
        );
        assert_approx_eq!(
            f64,
            Angle::new(1261.0).quadrant_shifted().degrees,
            -179.0,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn adding_angles() {
        let angle_a = Angle::new(45.0);
        let angle_b = Angle::new(45.0);

        assert_approx_eq!(f64, (angle_a + angle_b).degrees, 90.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn calculate_rounding_nearest() {
        let time_1 = Utc.with_ymd_and_hms(2015, 7, 13, 4, 37, 30).unwrap();

        assert_eq!(
            time_1.rounded_minute(Rounding::Nearest),
            Utc.with_ymd_and_hms(2015, 7, 13, 4, 38, 00).unwrap()
        );
    }

    #[test]
    fn calculate_rounding_up() {
        let time_1 = Utc.with_ymd_and_hms(2015, 7, 13, 5, 59, 20).unwrap();

        assert_eq!(
            time_1.rounded_minute(Rounding::Up),
            Utc.with_ymd_and_hms(2015, 7, 13, 6, 00, 00).unwrap()
        );
    }

    #[test]
    fn calculate_rounding_none() {
        let time_1 = Utc.with_ymd_and_hms(2015, 7, 13, 5, 59, 20).unwrap();

        assert_eq!(
            time_1.rounded_minute(Rounding::None),
            Utc.with_ymd_and_hms(2015, 7, 13, 5, 59, 20).unwrap()
        );
    }
}
