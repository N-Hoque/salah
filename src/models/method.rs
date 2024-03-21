// Salah
//
// See README.md and LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use super::{
    adjustments::Adjustment,
    parameters::{Configuration, Parameters},
    rounding::Rounding,
};
use serde::{Deserialize, Serialize};

/// Provides preset configuration for a few authorities
/// for calculating prayer times.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Method {
    /// Muslim World League. Standard Fajr time with an angle of 18°.
    /// Earlier Isha time with an angle of 17°.
    MuslimWorldLeague,

    /// Egyptian General Authority of Survey. Early Fajr time using an angle 19.5°
    /// and a slightly earlier Isha time using an angle of 17.5°.
    Egyptian,

    /// University of Islamic Sciences, Karachi. A generally applicable method that
    /// uses standard Fajr and Isha angles of 18°.
    Karachi,

    /// Umm al-Qura University, Makkah. Uses a fixed interval of 90 minutes
    /// from maghrib to calculate Isha. And a slightly earlier Fajr time with
    /// an angle of 18.5°. Note: you should add a +30 minute custom adjustment
    /// for Isha during Ramadan.
    UmmAlQura,

    /// Used in the UAE. Slightly earlier Fajr time and slightly later Isha
    /// time with angles of 18.2° for Fajr and Isha in addition to 3 minute
    /// offsets for sunrise, Dhuhr, Asr, and Maghrib.
    Dubai,

    /// Method developed by Khalid Shaukat, founder of Moonsighting Committee Worldwide.
    /// Uses standard 18° angles for Fajr and Isha in addition to seasonal adjustment values.
    /// This method automatically applies the 1/7 approximation rule for locations above 55°
    /// latitude. Recommended for North America and the UK.
    MoonsightingCommittee,

    /// Also known as the ISNA method. Can be used for North America,
    /// but the moonsightingCommittee method is preferable. Gives later Fajr times and early.
    /// Isha times with angles of 15°.
    NorthAmerica,

    /// Standard Fajr time with an angle of 18°. Slightly earlier Isha time with an angle of 17.5°.
    Kuwait,

    /// Same Isha interval as `ummAlQura` but with the standard Fajr time using an angle of 18°.
    Qatar,

    /// Used in Singapore, Malaysia, and Indonesia. Early Fajr time with an angle of 20°
    /// and standard Isha time with an angle of 18°.
    Singapore,

    /// Institute of Geophysics, University of Tehran. Early Isha time with an angle of 14°.
    /// Slightly later Fajr time with an angle of 17.7°. Calculates Maghrib based on the sun
    /// reaching an angle of 4.5° below the horizon.
    Tehran,

    /// An approximation of the Diyanet method used in Turkey.
    /// This approximation is less accurate outside the region of Turkey.
    Turkey,

    /// Defaults to angles of 0°, should generally be used for making a custom method
    /// and setting your own values.
    #[default]
    Other,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MuslimWorldLeague => write!(f, "Muslim World League"),
            Self::Egyptian => write!(f, "Egyptian"),
            Self::Karachi => write!(f, "Karachi"),
            Self::UmmAlQura => write!(f, "Umm Al Qura"),
            Self::Dubai => write!(f, "Dubai"),
            Self::MoonsightingCommittee => write!(f, "Moonsighting Committee"),
            Self::NorthAmerica => write!(f, "North America"),
            Self::Kuwait => write!(f, "Kuwait"),
            Self::Qatar => write!(f, "Qatar"),
            Self::Singapore => write!(f, "Singapore"),
            Self::Tehran => write!(f, "Tehran"),
            Self::Turkey => write!(f, "Turkey"),
            Self::Other => write!(f, "Other"),
        }
    }
}

impl Method {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn parameters(&self) -> Parameters {
        match self {
            Self::MuslimWorldLeague => Configuration::new()
                .fajr_angle(18.0)
                .isha_angle(17.0)
                .method(*self)
                .method_adjustments(Adjustment::default().dhuhr(1).build().unwrap())
                .build()
                .unwrap(),

            Self::Egyptian => Configuration::new()
                .fajr_angle(19.5)
                .isha_angle(17.5)
                .method(*self)
                .method_adjustments(Adjustment::default().dhuhr(1).build().unwrap())
                .build()
                .unwrap(),

            Self::Karachi => Configuration::new()
                .fajr_angle(18.0)
                .isha_angle(18.0)
                .method(*self)
                .method_adjustments(Adjustment::default().dhuhr(1).build().unwrap())
                .build()
                .unwrap(),

            Self::UmmAlQura => Configuration::new()
                .fajr_angle(18.5)
                .isha_angle(0.0)
                .method(*self)
                .isha_interval(90)
                .build()
                .unwrap(),
            Self::Dubai => Configuration::new()
                .fajr_angle(18.2)
                .isha_angle(18.2)
                .method(*self)
                .method_adjustments(
                    Adjustment::default()
                        .sunrise(-3)
                        .dhuhr(3)
                        .asr(3)
                        .maghrib(3)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),

            Self::MoonsightingCommittee => Configuration::new()
                .fajr_angle(18.0)
                .isha_angle(18.0)
                .method(*self)
                .method_adjustments(Adjustment::default().dhuhr(5).maghrib(3).build().unwrap())
                .build()
                .unwrap(),

            Self::NorthAmerica => Configuration::new()
                .fajr_angle(15.0)
                .isha_angle(15.0)
                .method(*self)
                .method_adjustments(Adjustment::default().dhuhr(1).build().unwrap())
                .build()
                .unwrap(),

            Self::Kuwait => Configuration::new()
                .fajr_angle(18.0)
                .isha_angle(17.5)
                .method(*self)
                .build()
                .unwrap(),

            Self::Qatar => Configuration::new()
                .fajr_angle(18.0)
                .isha_angle(0.0)
                .method(*self)
                .isha_interval(90)
                .build()
                .unwrap(),

            Self::Singapore => Configuration::new()
                .fajr_angle(20.0)
                .isha_angle(18.0)
                .method(*self)
                .method_adjustments(Adjustment::default().dhuhr(1).build().unwrap())
                .rounding(Rounding::Up)
                .build()
                .unwrap(),

            Self::Tehran => Configuration::new()
                .fajr_angle(17.7)
                .isha_angle(14.0)
                .method(*self)
                .maghrib_angle(4.5)
                .build()
                .unwrap(),

            Self::Turkey => Configuration::new()
                .fajr_angle(18.0)
                .isha_angle(17.0)
                .method(*self)
                .method_adjustments(
                    Adjustment::default()
                        .sunrise(-7)
                        .dhuhr(5)
                        .asr(4)
                        .maghrib(7)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),

            Self::Other => Configuration::new()
                .fajr_angle(0.0)
                .isha_angle(0.0)
                .method(*self)
                .build()
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::using_muslim_world_league(Method::MuslimWorldLeague, (18.0, 17.0), 0)]
    #[case::using_egyptian(Method::Egyptian, (19.5, 17.5), 0)]
    #[case::using_karachi(Method::Karachi, (18.0, 18.0), 0)]
    #[case::using_umm_al_qura(Method::UmmAlQura, (18.5, 0.0), 90)]
    #[case::using_dubai(Method::Dubai, (18.2, 18.2), 0)]
    #[case::using_moonsighting_committee(Method::MoonsightingCommittee, (18.0, 18.0), 0)]
    #[case::using_north_america(Method::NorthAmerica, (15.0, 15.0), 0)]
    #[case::using_kuwait(Method::Kuwait, (18.0, 17.5), 0)]
    #[case::using_qatar(Method::Qatar, (18.0, 0.0), 90)]
    #[case::using_singapore(Method::Singapore, (20.0, 18.0), 0)]
    #[case::using_other(Method::Other, (0.0, 0.0), 0)]
    fn test_parameters_from_method(#[case] method: Method, #[case] angles: (f64, f64), #[case] interval: i32) {
        const EPSILON: f64 = 0.000_000_1;

        let params = method.parameters();

        let (fajr, isha) = angles;

        assert_eq!(params.method, method);
        assert_approx_eq!(f64, params.fajr_angle, fajr, epsilon = EPSILON);
        assert_approx_eq!(f64, params.isha_angle, isha, epsilon = EPSILON);
        assert_eq!(params.isha_interval, interval);
    }
}
