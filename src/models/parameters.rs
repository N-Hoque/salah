// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use serde::{Deserialize, Serialize};

use super::{
    adjustments::TimeAdjustment, event::Prayer, high_altitude_rule::HighLatitudeRule, madhab::Madhab, method::Method,
    rounding::Rounding, shafaq::Shafaq,
};
use crate::Event;

const ONE_HALF: f64 = 0.5;
const ONE_SEVENTH: f64 = 1.0 / 7.0;

/// Settings that are used for determining the
/// the correct prayer time.
///
/// It is recommended to use [Configuration](struct.Configuration.html) to build
/// the parameters that are need.
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize, derive_builder::Builder)]
#[builder(default, name = "Configuration")]
pub struct Parameters {
    pub method: Method,
    pub fajr_angle: f64,
    pub maghrib_angle: f64,
    pub isha_angle: f64,
    #[builder(setter(custom))]
    pub isha_interval: i32,
    pub madhab: Madhab,
    pub high_latitude_rule: HighLatitudeRule,
    pub adjustments: TimeAdjustment,
    pub method_adjustments: TimeAdjustment,
    pub rounding: Rounding,
    pub shafaq: Shafaq,
}

impl Parameters {
    #[must_use]
    pub fn from_angles(fajr_angle: f64, isha_angle: f64) -> Self {
        Self {
            fajr_angle,
            maghrib_angle: 0.0,
            isha_angle,
            method: Method::Other,
            isha_interval: 0,
            madhab: Madhab::Shafi,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
            rounding: Rounding::Nearest,
            shafaq: Shafaq::General,
        }
    }

    #[must_use]
    pub fn from_method(method: Method) -> Self {
        method.parameters()
    }

    #[must_use]
    pub const fn with_madhab(mut self, madhab: Madhab) -> Self {
        self.madhab = madhab;
        self
    }

    #[must_use]
    pub fn night_portions(&self) -> (f64, f64) {
        match self.high_latitude_rule {
            HighLatitudeRule::MiddleOfTheNight => (ONE_HALF, ONE_HALF),
            HighLatitudeRule::SeventhOfTheNight => (ONE_SEVENTH, ONE_SEVENTH),
            HighLatitudeRule::TwilightAngle => (self.fajr_angle / 60.0, self.isha_angle / 60.0),
        }
    }

    #[must_use]
    pub const fn time_adjustments(&self, prayer: Event) -> i64 {
        match prayer {
            Event::Sunrise => self.adjustments.sunrise + self.method_adjustments.sunrise,
            Event::Prayer(Prayer::Fajr) => self.adjustments.fajr + self.method_adjustments.fajr,
            Event::Prayer(Prayer::Dhuhr) => self.adjustments.dhuhr + self.method_adjustments.dhuhr,
            Event::Prayer(Prayer::Asr) => self.adjustments.asr + self.method_adjustments.asr,
            Event::Prayer(Prayer::Maghrib) => self.adjustments.maghrib + self.method_adjustments.maghrib,
            Event::Prayer(Prayer::Isha) => self.adjustments.isha + self.method_adjustments.isha,
            _ => 0,
        }
    }
}

impl Configuration {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn isha_interval(&mut self, isha_interval: i32) -> &mut Self {
        self.isha_angle = Some(0.0);
        self.isha_interval = Some(isha_interval);
        self
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn calculate_parameters_with_fajr_and_isha_angles() {
        const FAJR_ANGLE: f64 = 18.0;
        const ISHA_ANGLE: f64 = FAJR_ANGLE;

        let params = Configuration::new()
            .fajr_angle(FAJR_ANGLE)
            .isha_angle(ISHA_ANGLE)
            .build()
            .unwrap();

        assert_approx_eq!(f64, params.fajr_angle, FAJR_ANGLE, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.isha_angle, ISHA_ANGLE, epsilon = 0.000_000_1);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn calculated_night_portions_middle_of_the_night() {
        const FAJR_ANGLE: f64 = 18.0;
        const ISHA_ANGLE: f64 = FAJR_ANGLE;

        let params = Configuration::new()
            .fajr_angle(FAJR_ANGLE)
            .isha_angle(ISHA_ANGLE)
            .build()
            .unwrap();

        assert_approx_eq!(f64, params.night_portions().0, ONE_HALF, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.night_portions().1, ONE_HALF, epsilon = 0.000_000_1);
    }

    #[test]
    fn calculated_night_portions_seventh_of_the_night() {
        const FAJR_ANGLE: f64 = 18.0;
        const ISHA_ANGLE: f64 = FAJR_ANGLE;

        let params = Configuration::new()
            .fajr_angle(FAJR_ANGLE)
            .isha_angle(ISHA_ANGLE)
            .high_latitude_rule(HighLatitudeRule::SeventhOfTheNight)
            .build()
            .unwrap();

        assert_approx_eq!(f64, params.night_portions().0, ONE_SEVENTH, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.night_portions().1, ONE_SEVENTH, epsilon = 0.000_000_1);
    }

    #[test]
    fn calculated_night_portions_twilight_angle() {
        const FAJR_ANGLE: f64 = 10.0;
        const ISHA_ANGLE: f64 = 10.0;

        let params = Configuration::new()
            .fajr_angle(FAJR_ANGLE)
            .isha_angle(ISHA_ANGLE)
            .high_latitude_rule(HighLatitudeRule::TwilightAngle)
            .build()
            .unwrap();

        assert_approx_eq!(f64, params.night_portions().0, FAJR_ANGLE / 60.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.night_portions().1, ISHA_ANGLE / 60.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn parameters_using_method_and_madhab() {
        const FAJR_ANGLE: f64 = 15.0;
        const ISHA_ANGLE: f64 = FAJR_ANGLE;

        let params = Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi);

        assert_eq!(params.method, Method::NorthAmerica);
        assert_approx_eq!(f64, params.fajr_angle, FAJR_ANGLE, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.isha_angle, ISHA_ANGLE, epsilon = 0.000_000_1);
        assert_eq!(params.isha_interval, 0);
        assert_eq!(params.madhab, Madhab::Hanafi);
    }
}
