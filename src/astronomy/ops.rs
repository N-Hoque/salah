// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use chrono::{DateTime, Duration, TimeZone};

use crate::{
    astronomy::unit::{Angle, Coordinates, Normalize, Stride},
    models::{rounding::Rounding, shafaq::Shafaq},
};

// The geometric mean longitude of the sun.
pub fn mean_solar_longitude(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 163
    let term1 = 280.466_456_7;
    let term2 = 36000.76983 * julian_century;
    let term3 = 0.000_303_2 * julian_century.powi(2);
    let degrees = term1 + term2 + term3;

    Angle::new(degrees).unwound()
}

// The geometric mean longitude of the moon.
pub fn mean_lunar_longitude(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 144
    let term1 = 218.3165;
    let term2 = 481_267.881_3 * julian_century;
    let degrees = term1 + term2;

    Angle::new(degrees).unwound()
}

pub fn ascending_lunar_node_longitude(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 144
    let term1 = 125.04452;
    let term2 = 1_934.136_261 * julian_century;
    let term3 = 0.002_070_8 * julian_century.powi(2);
    let term4 = julian_century.powi(3) / 450_000.0;
    let degrees = term1 - term2 + term3 + term4;

    Angle::new(degrees).unwound()
}

// The mean anomaly of the sun.
pub fn mean_solar_anomaly(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 163
    let term1 = 357.52911;
    let term2 = 35999.05029 * julian_century;
    let term3 = 0.000_153_7 * julian_century.powi(2);
    let degrees = term1 + term2 - term3;

    Angle::new(degrees).unwound()
}

// The Sun's equation of the center.
pub fn solar_equation_of_the_center(julian_century: f64, mean_anomaly: Angle) -> Angle {
    // Equation from Astronomical Algorithms page 164
    let mean_radians = mean_anomaly.radians();
    let term1 = 0.000_014f64.mul_add(
        -julian_century.powi(2),
        0.004_817f64.mul_add(-julian_century, 1.914_602),
    ) * mean_radians.sin();
    let term2 = 0.000_101f64.mul_add(-julian_century, 0.019_993) * (2.0 * mean_radians).sin();
    let term3 = 0.000_289 * (3.0 * mean_radians).sin();

    Angle::new(term1 + term2 + term3)
}

// The apparent longitude of the Sun, referred to the
// true equinox of the date.
pub fn apparent_solar_longitude(julian_century: f64, mean_longitude: Angle) -> Angle {
    // Equation from Astronomical Algorithms page 164
    let longitude = mean_longitude + solar_equation_of_the_center(julian_century, mean_solar_anomaly(julian_century));
    let omega = Angle::new(1934.136f64.mul_add(-julian_century, 125.04));
    let lambda = Angle::new(0.00478f64.mul_add(-omega.radians().sin(), longitude.degrees - 0.00569));

    lambda.unwound()
}

// The mean obliquity of the ecliptic, formula
// adopted by the International Astronomical Union.
pub fn mean_obliquity_of_the_ecliptic(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 147
    let term1 = 23.439_291;
    let term2 = 0.013_004_167 * julian_century;
    let term3 = 0.000_000_163_9 * julian_century.powi(2);
    let term4 = 0.000_000_503_6 * julian_century.powi(3);

    Angle::new(term1 - term2 - term3 + term4)
}

// The mean obliquity of the ecliptic, corrected for
// calculating the apparent position of the sun.
pub fn apparent_obliquity_of_the_ecliptic(julian_century: f64, mean_obliquity_of_the_ecliptic: Angle) -> Angle {
    // Equation from Astronomical Algorithms page 165
    let degrees: f64 = 1934.136f64.mul_add(-julian_century, 125.04);

    Angle::new(0.00256f64.mul_add(
        Angle::new(degrees).radians().cos(),
        mean_obliquity_of_the_ecliptic.degrees,
    ))
}

// Mean sidereal time, the hour angle of the vernal equinox.
pub fn mean_sidereal_time(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 165
    let julian_day = julian_century.mul_add(36525.0, 2_451_545.0);
    let term1 = 280.460_618_37;
    let term2 = 360.985_647_366_29 * (julian_day - 2_451_545.0);
    let term3 = 0.000_387_933 * julian_century.powi(2);
    let term4 = julian_century.powi(3) / 38_710_000.0;
    let degrees = term1 + term2 + term3 - term4;

    Angle::new(degrees).unwound()
}

pub fn nutation_in_longitude(solar_longitude: Angle, lunar_longitude: Angle, ascending_node: Angle) -> f64 {
    // Equation from Astronomical Algorithms page 144
    let term1 = (-17.2 / 3600.0) * ascending_node.radians().sin();
    let term2 = (1.32 / 3600.0) * (2.0 * solar_longitude.radians()).sin();
    let term3 = (0.23 / 3600.0) * (2.0 * lunar_longitude.radians()).sin();
    let term4 = (0.21 / 3600.0) * (2.0 * ascending_node.radians()).sin();

    term1 - term2 - term3 + term4
}

pub fn nutation_in_obliquity(solar_longitude: Angle, lunar_longitude: Angle, ascending_node: Angle) -> f64 {
    // Equation from Astronomical Algorithms page 144
    let term1 = (9.2 / 3600.0) * ascending_node.radians().cos();
    let term2 = (0.57 / 3600.0) * (2.0 * solar_longitude.radians()).cos();
    let term3 = (0.10 / 3600.0) * (2.0 * lunar_longitude.radians()).cos();
    let term4 = (0.09 / 3600.0) * (2.0 * ascending_node.radians()).cos();

    term1 + term2 + term3 - term4
}

pub fn altitude_of_celestial_body(observer_latitude: Angle, declination: Angle, local_hour_angle: Angle) -> Angle {
    // Equation from Astronomical Algorithms page 93
    let term1 = observer_latitude.radians().sin() * declination.radians().sin();
    let term2 = observer_latitude.radians().cos() * declination.radians().cos() * local_hour_angle.radians().cos();

    Angle::from_radians((term1 + term2).asin())
}

pub fn approximate_transit(longitude: Angle, sidereal_time: Angle, right_ascension: Angle) -> f64 {
    // Equation from page Astronomical Algorithms 102
    let longitude_angle = longitude * Angle::new(-1.0);

    ((right_ascension + longitude_angle - sidereal_time) / Angle::new(360.0))
        .degrees
        .normalized_to_scale(1.0)
}

// The time at which the sun is at its highest point in the sky (in universal time)
pub fn corrected_transit(
    approximate_transit: f64,
    longitude: Angle,
    sidereal_time: Angle,
    right_ascension: Angle,
    previous_right_ascension: Angle,
    next_right_ascension: Angle,
) -> f64 {
    // Equation from page Astronomical Algorithms 102
    let longitude_angle = longitude * Angle::new(-1.0);
    let plane_angle = Angle::new(360.985_647f64.mul_add(approximate_transit, sidereal_time.degrees)).unwound();
    let interpolated_angles = interpolate_angles(
        right_ascension,
        previous_right_ascension,
        next_right_ascension,
        approximate_transit,
    )
    .unwound();
    let angles = (plane_angle - longitude_angle - interpolated_angles).quadrant_shifted();
    let angle_delta = angles / Angle::new(-360.0);

    (approximate_transit + angle_delta.degrees) * 24.0
}

pub fn corrected_hour_angle(
    approximate_transit: f64,
    angle: Angle,
    coordinates: Coordinates,
    after_transit: bool,
    sidereal_time: Angle,
    right_ascension: Angle,
    previous_right_ascension: Angle,
    next_right_ascension: Angle,
    declination: Angle,
    previous_declination: Angle,
    next_declination: Angle,
) -> f64 {
    // Equation from page Astronomical Algorithms 102
    let longitude_angle = coordinates.longitude_angle() * Angle::new(-1.0);
    let term1 = coordinates
        .latitude_angle()
        .radians()
        .sin()
        .mul_add(-declination.radians().sin(), angle.radians().sin());
    let term2 = coordinates.latitude_angle().radians().cos() * declination.radians().cos();
    let term_angle = Angle::from_radians((term1 / term2).acos());

    let adjusted_approx_transit = if after_transit {
        approximate_transit + (term_angle.degrees / 360.0)
    } else {
        approximate_transit - (term_angle.degrees / 360.0)
    };

    let plane_angle = Angle::new(360.985_647f64.mul_add(adjusted_approx_transit, sidereal_time.degrees)).unwound();
    let interpolated_angles = interpolate_angles(
        right_ascension,
        previous_right_ascension,
        next_right_ascension,
        adjusted_approx_transit,
    )
    .unwound();
    let declination_angle = Angle::new(interpolate(
        declination.degrees,
        previous_declination.degrees,
        next_declination.degrees,
        adjusted_approx_transit,
    ));
    let adjusted_angles = plane_angle - longitude_angle - interpolated_angles;
    let celestial_body_altitude =
        altitude_of_celestial_body(coordinates.latitude_angle(), declination_angle, adjusted_angles);
    let term3 = (celestial_body_altitude - angle).degrees;
    let term4 = 360.0
        * declination_angle.radians().cos()
        * coordinates.latitude_angle().radians().cos()
        * adjusted_angles.radians().sin();
    let angle_delta = term3 / term4;

    (adjusted_approx_transit + angle_delta) * 24.0
}

// Interpolation of a value given equidistant previous and
// next values and a factor equal to the fraction of the interpolated
// point's time over the time between values.
pub fn interpolate(value: f64, previous_value: f64, next_value: f64, factor: f64) -> f64 {
    // Equation from Astronomical Algorithms page 24
    let a = value - previous_value;
    let b = next_value - value;
    let c = b - a;

    (factor / 2.0).mul_add(factor.mul_add(c, a + b), value)
}

// Interpolation of three angles, accounting for angle unwinding.
pub fn interpolate_angles(value: Angle, previous_value: Angle, next_value: Angle, factor: f64) -> Angle {
    // Equation from Astronomical Algorithms page 24
    let a = (value - previous_value).unwound();
    let b = (next_value - value).unwound();
    let c = b - a;

    Angle::new((factor / 2.0).mul_add(factor.mul_add(c.degrees, a.degrees + b.degrees), value.degrees))
}

// The Julian Day for the given Gregorian date.
pub fn julian_day(year: i32, month: i32, day: i32, hours: f64) -> f64 {
    // Equation from Astronomical Algorithms page 60

    // NOTE: Casting to i32 is done intentionally for the purpose of decimal truncation

    let adjusted_year: i32 = if month > 2 { year } else { year - 1 };
    let adjusted_month: i32 = if month > 2 { month } else { month + 12 };
    let adjusted_day: f64 = f64::from(day) + (hours / 24.0);

    let a: i32 = adjusted_year / 100;
    let b: i32 = 2 - a + (a / 4);

    let i0: i32 = (365.25 * (f64::from(adjusted_year) + 4716.0)) as i32;
    let i1: i32 = (30.6001 * (f64::from(adjusted_month) + 1.0)) as i32;

    f64::from(i0) + f64::from(i1) + adjusted_day + f64::from(b) - 1524.5
}

// Julian century from the epoch.
pub fn julian_century(julian_day: f64) -> f64 {
    // Equation from Astronomical Algorithms page 163
    (julian_day - 2_451_545.0) / 36525.0
}

// Checks if the given year is a leap year.
pub const fn is_leap_year(year: u32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

// Twilight adjustment based on observational data for use
// in the Moonsighting Committee calculation method.
pub fn season_adjusted_morning_twilight<Tz: TimeZone>(
    latitude: f64,
    day: u32,
    year: u32,
    sunrise: &DateTime<Tz>,
) -> DateTime<Tz> {
    let dyy = f64::from(days_since_solstice(day, year, latitude));
    let adjustment = twilight_adjustments(AdjustmentDaytime::Morning, latitude, dyy, Shafaq::General);

    let rounded_adjustment = (adjustment * -60.0).round() as i64;
    sunrise
        .clone()
        .checked_add_signed(Duration::try_seconds(rounded_adjustment).unwrap())
        .unwrap()
}

fn twilight_adjustments(daytime: AdjustmentDaytime, latitude: f64, dyy: f64, shafaq: Shafaq) -> f64 {
    let adjustment_values = twilight_adjustment_values(daytime, latitude, shafaq);

    if (0.00..=90.0).contains(&dyy) {
        ((adjustment_values.b - adjustment_values.a) / 91.0).mul_add(dyy, adjustment_values.a)
    } else if (91.0..=136.0).contains(&dyy) {
        ((adjustment_values.c - adjustment_values.b) / 46.0).mul_add(dyy - 91.0, adjustment_values.b)
    } else if (137.0..=182.0).contains(&dyy) {
        ((adjustment_values.d - adjustment_values.c) / 46.0).mul_add(dyy - 137.0, adjustment_values.c)
    } else if (183.0..=228.0).contains(&dyy) {
        ((adjustment_values.c - adjustment_values.d) / 46.0).mul_add(dyy - 183.0, adjustment_values.d)
    } else if (229.0..=274.0).contains(&dyy) {
        ((adjustment_values.b - adjustment_values.c) / 46.0).mul_add(dyy - 229.0, adjustment_values.c)
    } else {
        ((adjustment_values.a - adjustment_values.b) / 91.0).mul_add(dyy - 275.0, adjustment_values.b)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum AdjustmentDaytime {
    Morning,
    Evening,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct TwilightAdjustmentValues {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

fn twilight_adjustment_values(daytime: AdjustmentDaytime, latitude: f64, shafaq: Shafaq) -> TwilightAdjustmentValues {
    if daytime == AdjustmentDaytime::Morning {
        TwilightAdjustmentValues {
            a: (28.65f64 / 55.0).mul_add(latitude.abs(), 75.0),
            b: (19.44f64 / 55.0).mul_add(latitude.abs(), 75.0),
            c: (32.74f64 / 55.0).mul_add(latitude.abs(), 75.0),
            d: (48.10f64 / 55.0).mul_add(latitude.abs(), 75.0),
        }
    } else {
        match shafaq {
            Shafaq::General => TwilightAdjustmentValues {
                a: (25.60f64 / 55.0).mul_add(latitude.abs(), 75.0),
                b: (2.050f64 / 55.0).mul_add(latitude.abs(), 75.0),
                c: (9.210f64 / 55.0).mul_add(-latitude.abs(), 75.0),
                d: (6.140f64 / 55.0).mul_add(latitude.abs(), 75.0),
            },
            Shafaq::Ahmer => TwilightAdjustmentValues {
                a: (17.40f64 / 55.0).mul_add(latitude.abs(), 62.0),
                b: (7.160f64 / 55.0).mul_add(-latitude.abs(), 62.0),
                c: (5.120f64 / 55.0).mul_add(latitude.abs(), 62.0),
                d: (19.44f64 / 55.0).mul_add(latitude.abs(), 62.0),
            },
            Shafaq::Abyad => TwilightAdjustmentValues {
                a: (25.60f64 / 55.0).mul_add(latitude.abs(), 75.0),
                b: (7.160f64 / 55.0).mul_add(latitude.abs(), 75.0),
                c: (36.84f64 / 55.0).mul_add(latitude.abs(), 75.0),
                d: (81.84f64 / 55.0).mul_add(latitude.abs(), 75.0),
            },
        }
    }
}

// Twilight adjustment based on observational data for use
// in the Moonsighting Committee calculation method.
pub fn season_adjusted_evening_twilight<Tz: TimeZone>(
    latitude: f64,
    day: u32,
    year: u32,
    sunset: &DateTime<Tz>,
    shafaq: Shafaq,
) -> DateTime<Tz> {
    let dyy = f64::from(days_since_solstice(day, year, latitude));
    let adjustment = twilight_adjustments(AdjustmentDaytime::Evening, latitude, dyy, shafaq);

    let rounded_adjustment = (adjustment * 60.0).round() as i64;
    let adjusted_date = sunset
        .clone()
        .checked_add_signed(Duration::try_seconds(rounded_adjustment).unwrap())
        .unwrap();

    adjusted_date.rounded_minute(Rounding::Nearest)
}

// Solstice calculation to determine a date's seasonal progression.
// Used in the Moonsighting Committee calculation method.
pub fn days_since_solstice(day_of_year: u32, year: u32, latitude: f64) -> u32 {
    let days_in_year = if is_leap_year(year) { 366 } else { 365 };

    if latitude < 0.0 {
        let southern_offset = if is_leap_year(year) { 173 } else { 172 };
        (day_of_year - southern_offset) + days_in_year
    } else {
        let northern_offset = 10;
        let lapsed_days = day_of_year + northern_offset;

        if lapsed_days >= days_in_year {
            lapsed_days - days_in_year
        } else {
            lapsed_days
        }
    }
}

pub fn adjust_time<Tz: TimeZone>(date: &DateTime<Tz>, minutes: i64) -> DateTime<Tz> {
    date.clone()
        .checked_add_signed(Duration::try_seconds(minutes * 60).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn calculate_julian_day() {
        let julian_day = julian_day(1992, 10, 13, 0.0);

        assert_approx_eq!(f64, julian_day, 2_448_908.5, epsilon = 0.000_000_1);
    }

    #[test]
    fn calculate_julian_century() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);

        assert_approx_eq!(f64, julian_century, -0.072_183_436_002_737_86, epsilon = 0.000_000_1);
    }

    #[test]
    fn calculate_mean_solar_longitude() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_longitude = mean_solar_longitude(julian_century);

        assert_approx_eq!(
            f64,
            mean_solar_longitude.degrees,
            201.807_193_206_707_32,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_apparent_solar_longitude() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_longitude = mean_solar_longitude(julian_century);
        let apparent_solar_longitude = apparent_solar_longitude(julian_century, mean_solar_longitude).radians();

        assert_approx_eq!(
            f64,
            apparent_solar_longitude,
            3.489_069_182_045_206,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_mean_obliquity_of_the_ecliptic() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_obliq_of_ecliptic = mean_obliquity_of_the_ecliptic(julian_century);

        assert_approx_eq!(
            f64,
            mean_obliq_of_ecliptic.degrees,
            23.440_229_684_413_012,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_apparent_obliquity_of_the_ecliptic() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_obliq_of_ecliptic = mean_obliquity_of_the_ecliptic(julian_century);
        let apparent_obliq_of_ecliptic = apparent_obliquity_of_the_ecliptic(julian_century, mean_obliq_of_ecliptic);

        assert_approx_eq!(
            f64,
            apparent_obliq_of_ecliptic.degrees,
            23.439_991_106_199_55,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_mean_solar_anomaly() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_anomaly = mean_solar_anomaly(julian_century);

        assert_approx_eq!(
            f64,
            mean_solar_anomaly.degrees,
            278.993_966_431_597_5,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_solar_equation_of_the_center() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_anomaly = mean_solar_anomaly(julian_century);
        let solar_equation_of_center = solar_equation_of_the_center(julian_century, mean_solar_anomaly);

        assert_approx_eq!(
            f64,
            solar_equation_of_center.degrees,
            -1.897_323_843_371_985,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_mean_lunar_longitude() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_lunar_longitude = mean_lunar_longitude(julian_century);

        assert_approx_eq!(
            f64,
            mean_lunar_longitude.degrees,
            38.747_190_008_209_145,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_acending_lunar_node_longitude() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let ascending_lunar_node = ascending_lunar_node_longitude(julian_century);

        assert_approx_eq!(
            f64,
            ascending_lunar_node.degrees,
            264.657_131_805_429,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_mean_sidereal_time() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_sidereal_time = mean_sidereal_time(julian_century);

        assert_approx_eq!(
            f64,
            mean_sidereal_time.degrees,
            21.801_339_167_752_303,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_nutation_longitude() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_longitude = mean_solar_longitude(julian_century);
        let mean_lunar_longitude = mean_lunar_longitude(julian_century);
        let ascending_lunar_node = ascending_lunar_node_longitude(julian_century);
        let nutation_longitude =
            nutation_in_longitude(mean_solar_longitude, mean_lunar_longitude, ascending_lunar_node);

        assert_approx_eq!(
            f64,
            nutation_longitude,
            0.004_452_535_816_968_656_4,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_nutation_in_obliquity() {
        let julian_day = julian_day(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_longitude = mean_solar_longitude(julian_century);
        let mean_lunar_longitude = mean_lunar_longitude(julian_century);
        let ascending_lunar_node = ascending_lunar_node_longitude(julian_century);
        let nutation_obliq = nutation_in_obliquity(mean_solar_longitude, mean_lunar_longitude, ascending_lunar_node);

        assert_approx_eq!(
            f64,
            nutation_obliq,
            -0.000_092_747_500_292_341_56,
            epsilon = 0.000_000_1
        );
    }

    #[test]
    fn calculate_altitude_of_celestial_body() {
        let coordinates = Coordinates::new(35.783_333_333_333_33, -78.65);
        let declination_angle = Angle::new(21.894_701_414_701_338);
        let local_hour_angle = Angle::new(108.092_753_578_383_22);
        let celestial_body =
            altitude_of_celestial_body(coordinates.latitude_angle(), declination_angle, local_hour_angle);

        assert_approx_eq!(
            f64,
            celestial_body.degrees,
            -0.900_615_621_559_432_1,
            epsilon = 0.000_000_1
        );
    }
}
