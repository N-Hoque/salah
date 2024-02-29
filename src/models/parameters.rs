// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use crate::Prayer;

use super::{
    adjustments::TimeAdjustment, high_altitude_rule::HighLatitudeRule, madhab::Madhab, method::Method,
    rounding::Rounding, shafaq::Shafaq,
};

/// Settings that are used for determining the
/// the correct prayer time.
///
/// It is recommended to use [Configuration](struct.Configuration.html) to build
/// the parameters that are need.
/// A builder for the the [Parameters](struct.Parameters.html).
///
/// It is recommended that this is used for setting
/// all parameters that are needed.
#[derive(Clone, derive_builder::Builder)]
#[builder(name = "Configuration")]
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
    pub fn new(fajr_angle: f64, isha_angle: f64) -> Self {
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
    pub fn night_portions(&self) -> (f64, f64) {
        match self.high_latitude_rule {
            HighLatitudeRule::MiddleOfTheNight => (1.0 / 2.0, 1.0 / 2.0),
            HighLatitudeRule::SeventhOfTheNight => (1.0 / 7.0, 1.0 / 7.0),
            HighLatitudeRule::TwilightAngle => (self.fajr_angle / 60.0, self.isha_angle / 60.0),
        }
    }

    #[must_use]
    pub const fn time_adjustments(&self, prayer: Prayer) -> i64 {
        match prayer {
            Prayer::Fajr => self.adjustments.fajr + self.method_adjustments.fajr,
            Prayer::Sunrise => self.adjustments.sunrise + self.method_adjustments.sunrise,
            Prayer::Dhuhr => self.adjustments.dhuhr + self.method_adjustments.dhuhr,
            Prayer::Asr => self.adjustments.asr + self.method_adjustments.asr,
            Prayer::Maghrib => self.adjustments.maghrib + self.method_adjustments.maghrib,
            Prayer::Isha => self.adjustments.isha + self.method_adjustments.isha,
            _ => 0,
        }
    }
}

impl Configuration {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fajr_angle: Some(0.0),
            maghrib_angle: Some(0.0),
            isha_angle: Some(0.0),
            method: Some(Method::Other),
            isha_interval: Some(0),
            madhab: Some(Madhab::Shafi),
            high_latitude_rule: Some(HighLatitudeRule::MiddleOfTheNight),
            adjustments: Some(TimeAdjustment::default()),
            method_adjustments: Some(TimeAdjustment::default()),
            rounding: Some(Rounding::Nearest),
            shafaq: Some(Shafaq::General),
        }
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
        let params = Configuration::new().fajr_angle(18.0).isha_angle(18.0).build().unwrap();

        assert_approx_eq!(f64, params.fajr_angle, 18.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.isha_angle, 18.0, epsilon = 0.000_000_1);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn calculated_night_portions_middle_of_the_night() {
        let params = Configuration::new().fajr_angle(18.0).isha_angle(18.0).build().unwrap();

        assert_approx_eq!(f64, params.night_portions().0, 1.0 / 2.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.night_portions().1, 1.0 / 2.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn calculated_night_portions_seventh_of_the_night() {
        let params = Configuration::new()
            .fajr_angle(18.0)
            .isha_angle(18.0)
            .high_latitude_rule(HighLatitudeRule::SeventhOfTheNight)
            .build()
            .unwrap();

        assert_approx_eq!(f64, params.night_portions().0, 1.0 / 7.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.night_portions().1, 1.0 / 7.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn calculated_night_portions_twilight_angle() {
        let params = Configuration::new()
            .fajr_angle(10.0)
            .isha_angle(15.0)
            .high_latitude_rule(HighLatitudeRule::TwilightAngle)
            .build()
            .unwrap();

        assert_approx_eq!(f64, params.night_portions().0, 10.0 / 60.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.night_portions().1, 15.0 / 60.0, epsilon = 0.000_000_1);
    }

    #[test]
    fn parameters_using_method_and_madhab() {
        let params = Configuration::new()
            .method(Method::NorthAmerica)
            .madhab(Madhab::Hanafi)
            .build()
            .unwrap();

        assert_eq!(params.method, Method::NorthAmerica);
        assert_approx_eq!(f64, params.fajr_angle, 15.0, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, params.isha_angle, 15.0, epsilon = 0.000_000_1);
        assert_eq!(params.isha_interval, 0);
        assert_eq!(params.madhab, Madhab::Hanafi);
    }
}
