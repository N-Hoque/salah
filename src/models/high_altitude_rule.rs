// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use serde::{Deserialize, Serialize};

use crate::astronomy::unit::Coordinates;

/// Rule for approximating Fajr and Isha at high latitudes

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum HighLatitudeRule {
    /// Fajr won't be earlier than the midpoint of the night and isha
    /// won't be later than the midpoint of the night. This is the default
    /// value to prevent fajr and isha crossing boundaries.
    #[default]
    MiddleOfTheNight,

    /// Fajr will never be earlier than the beginning of the last seventh of
    /// the night and Isha will never be later than the end of the first seventh of the night.
    ///
    /// This is recommended to use for locations above 48° latitude to prevent prayer
    /// times that would be difficult to perform.
    SeventhOfTheNight,

    /// The night is divided into portions of roughly 1/3. The exact value is derived
    /// by dividing the fajr/isha angles by 60.
    ///
    /// This can be used to prevent difficult fajr and isha times at certain locations.
    TwilightAngle,
}

impl HighLatitudeRule {
    pub fn recommended(coordinates: &Coordinates) -> Self {
        if coordinates.latitude > 48.0 {
            Self::SeventhOfTheNight
        } else {
            Self::MiddleOfTheNight
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::normal_rule((45.983_226, -3.216_649), HighLatitudeRule::MiddleOfTheNight)]
    #[case::high_lat_rule((48.983_226, -3.216_649), HighLatitudeRule::SeventhOfTheNight)]
    fn test_recommended_rule_for_position(#[case] coords: (f64, f64), #[case] expected_rule: HighLatitudeRule) {
        let location = Coordinates::from(coords);

        assert_eq!(HighLatitudeRule::recommended(&location), expected_rule);
    }
}
