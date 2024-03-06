// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

//! # Prayer Schedule
//!
//! This module provides the main objects that are used for calculating
//! the prayer times.

use chrono::{DateTime, Datelike, Days, Duration, Local, TimeZone, Utc};

use crate::{
    astronomy::{
        ops,
        solar::SolarTime,
        unit::{Angle, Coordinates, Stride},
    },
    models::{method::Method, parameters::Parameters, prayer::Prayer, rounding::Rounding},
};

/// A data struct to hold the timing for all
/// prayers.
#[derive(Clone)]
pub struct PrayerTimes<Tz: TimeZone> {
    fajr: DateTime<Tz>,
    sunrise: DateTime<Tz>,
    dhuhr: DateTime<Tz>,
    asr: DateTime<Tz>,
    maghrib: DateTime<Tz>,
    isha: DateTime<Tz>,
    midnight: DateTime<Tz>,
    qiyam: DateTime<Tz>,
    fajr_tomorrow: DateTime<Tz>,
}

impl std::fmt::Display for PrayerTimes<Utc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current_time = Utc::now();
        let (hours, minutes) = self.time_remaining(&current_time);

        let prayer_table = tabled::col![
            current_time.format("%A, %-d %B, %C%y %H:%M:%S"),
            tabled::row![
                tabled::col!["Fajr", "Dhuhr", "Asr", "Maghrib", "Isha"],
                tabled::col![
                    self.fajr.time().format("%H:%M"),
                    self.dhuhr.time().format("%H:%M"),
                    self.asr.time().format("%H:%M"),
                    self.maghrib.time().format("%H:%M"),
                    self.isha.time().format("%H:%M"),
                ],
                tabled::col!["Current Prayer", "Next Prayer", "Time Left", "Midnight", "Qiyam"],
                tabled::col![
                    self.current(&current_time),
                    self.next(&current_time),
                    format!("{hours}h {minutes}m"),
                    self.midnight.time().format("%H:%M"),
                    self.qiyam.time().format("%H:%M")
                ]
            ]
        ];

        write!(f, "{prayer_table}")
    }
}

impl std::fmt::Display for PrayerTimes<Local> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current_time = Local::now();
        let (hours, minutes) = self.time_remaining(&current_time);

        let prayer_table = tabled::col![
            current_time.format("%A, %-d %B, %C%y %H:%M:%S"),
            tabled::row![
                tabled::col!["Fajr", "Dhuhr", "Asr", "Maghrib", "Isha"],
                tabled::col![
                    self.fajr.time().format("%H:%M"),
                    self.dhuhr.time().format("%H:%M"),
                    self.asr.time().format("%H:%M"),
                    self.maghrib.time().format("%H:%M"),
                    self.isha.time().format("%H:%M"),
                ],
                tabled::col!["Current Prayer", "Next Prayer", "Time Left", "Midnight", "Qiyam"],
                tabled::col![
                    self.current(&current_time),
                    self.next(&current_time),
                    format!("{hours}h {minutes}m"),
                    self.midnight.time().format("%H:%M"),
                    self.qiyam.time().format("%H:%M")
                ]
            ]
        ];

        write!(f, "{prayer_table}")
    }
}

impl<Tz: TimeZone> PrayerTimes<Tz> {
    #[must_use]
    pub fn new(date: &DateTime<Tz>, coordinates: Coordinates, parameters: &Parameters) -> Self {
        let tomorrow = date.tomorrow();
        let solar_time = SolarTime::new(date, coordinates);
        let solar_time_tomorrow = SolarTime::new(&tomorrow, coordinates);

        let asr = solar_time.afternoon(parameters.madhab.shadow().into());
        let night = solar_time_tomorrow
            .clone()
            .sunrise
            .signed_duration_since(&solar_time.sunset);

        let fajr =
            Self::calculate_fajr(parameters, &solar_time, night, coordinates, date).rounded_minute(parameters.rounding);
        let sunrise = solar_time
            .sunrise
            .adjust_time(parameters.time_adjustments(Prayer::Sunrise))
            .rounded_minute(parameters.rounding);
        let dhuhr = solar_time
            .transit
            .adjust_time(parameters.time_adjustments(Prayer::Dhuhr))
            .rounded_minute(parameters.rounding);
        let asr = asr
            .adjust_time(parameters.time_adjustments(Prayer::Asr))
            .rounded_minute(parameters.rounding);
        let maghrib = ops::adjust_time(&solar_time.sunset, parameters.time_adjustments(Prayer::Maghrib))
            .rounded_minute(parameters.rounding);
        let isha =
            Self::calculate_isha(parameters, solar_time, night, coordinates, date).rounded_minute(parameters.rounding);

        // Calculate the middle of the night and qiyam times
        let (midnight, qiyam, fajr_tomorrow) =
            Self::calculate_qiyam(&maghrib, parameters, &solar_time_tomorrow, coordinates, &tomorrow);

        Self {
            fajr,
            sunrise,
            dhuhr,
            asr,
            maghrib,
            isha,
            midnight,
            qiyam,
            fajr_tomorrow,
        }
    }

    #[must_use]
    pub fn time(&self, prayer: Prayer) -> DateTime<Tz> {
        match prayer {
            Prayer::Fajr => self.fajr.clone(),
            Prayer::Sunrise => self.sunrise.clone(),
            Prayer::Dhuhr => self.dhuhr.clone(),
            Prayer::Asr => self.asr.clone(),
            Prayer::Maghrib => self.maghrib.clone(),
            Prayer::Isha => self.isha.clone(),
            Prayer::Qiyam => self.qiyam.clone(),
            Prayer::FajrTomorrow => self.fajr_tomorrow.clone(),
        }
    }

    #[must_use]
    pub fn current(&self, time: &DateTime<Tz>) -> Prayer {
        self.current_time(time)
    }

    #[must_use]
    pub fn next(&self, time: &DateTime<Tz>) -> Prayer {
        match self.current(time) {
            Prayer::Fajr => Prayer::Sunrise,
            Prayer::Sunrise => Prayer::Dhuhr,
            Prayer::Dhuhr => Prayer::Asr,
            Prayer::Asr => Prayer::Maghrib,
            Prayer::Maghrib => Prayer::Isha,
            Prayer::Isha => Prayer::Qiyam,
            _ => Prayer::FajrTomorrow,
        }
    }

    #[must_use]
    pub fn time_remaining(&self, time: &DateTime<Tz>) -> (u32, u32) {
        let mut now = Utc::now();
        if self.next(time) == Prayer::FajrTomorrow {
            // If we're waiting for FajrTomorrow, we need to push the day
            // forward by 1 so that the time keeping is corrected
            now = now.checked_add_days(Days::new(1)).unwrap();
        }
        let next_time = self.time(self.next(time));
        let now_to_next = next_time.signed_duration_since(now).num_seconds() as f64;
        let whole: f64 = now_to_next / 3600.0;
        let fract = whole.fract();
        let hours = whole.trunc() as u32;
        let minutes = (fract * 60.0).round() as u32;

        (hours, minutes)
    }

    fn current_time(&self, time: &DateTime<Tz>) -> Prayer {
        if self.fajr_tomorrow.clone().signed_duration_since(time).num_seconds() <= 0 {
            Prayer::FajrTomorrow
        } else if self.qiyam.clone().signed_duration_since(time).num_seconds() <= 0 {
            Prayer::Qiyam
        } else if self.isha.clone().signed_duration_since(time).num_seconds() <= 0 {
            Prayer::Isha
        } else if self.maghrib.clone().signed_duration_since(time).num_seconds() <= 0 {
            Prayer::Maghrib
        } else if self.asr.clone().signed_duration_since(time).num_seconds() <= 0 {
            Prayer::Asr
        } else if self.dhuhr.clone().signed_duration_since(time).num_seconds() <= 0 {
            Prayer::Dhuhr
        } else if self.sunrise.clone().signed_duration_since(time).num_seconds() <= 0 {
            Prayer::Sunrise
        } else if self.fajr.clone().signed_duration_since(time).num_seconds() <= 0 {
            Prayer::Fajr
        } else {
            // In case no other time matches, it must be treated as Qiyam to prevent
            // the library from failing when checking times *before* Fajr
            Prayer::Qiyam
        }
    }

    fn calculate_fajr(
        parameters: &Parameters,
        solar_time: &SolarTime<Tz>,
        night: Duration,
        coordinates: Coordinates,
        prayer_date: &DateTime<Tz>,
    ) -> DateTime<Tz> {
        let mut fajr = if parameters.method == Method::MoonsightingCommittee && coordinates.latitude >= 55.0 {
            // special case for moonsighting committee above latitude 55
            let night_fraction = night.num_seconds() / 7;
            solar_time
                .clone()
                .sunrise
                .checked_add_signed(Duration::try_seconds(-night_fraction).unwrap())
                .unwrap()
        } else {
            // Nothing to do.
            solar_time.time_for_solar_angle(Angle::new(-parameters.fajr_angle), false)
        };

        let safe_fajr = if parameters.method == Method::MoonsightingCommittee {
            let day_of_year = prayer_date.ordinal();
            ops::season_adjusted_morning_twilight(
                coordinates.latitude,
                day_of_year,
                prayer_date.year() as u32,
                &solar_time.sunrise,
            )
        } else {
            let portion = parameters.night_portions().0;
            let night_fraction = portion * (night.num_seconds() as f64);

            solar_time
                .clone()
                .sunrise
                .checked_add_signed(Duration::try_seconds(-night_fraction as i64).unwrap())
                .unwrap()
        };

        if fajr < safe_fajr {
            fajr = safe_fajr;
        }

        fajr.adjust_time(parameters.time_adjustments(Prayer::Fajr))
    }

    fn calculate_isha(
        parameters: &Parameters,
        solar_time: SolarTime<Tz>,
        night: Duration,
        coordinates: Coordinates,
        prayer_date: &DateTime<Tz>,
    ) -> DateTime<Tz> {
        if parameters.isha_interval > 0 {
            solar_time
                .sunset
                .checked_add_signed(Duration::try_seconds(i64::from(parameters.isha_interval * 60)).unwrap())
                .unwrap()
        } else {
            let safe_isha = if parameters.method == Method::MoonsightingCommittee {
                let day_of_year = prayer_date.ordinal();

                ops::season_adjusted_evening_twilight(
                    coordinates.latitude,
                    day_of_year,
                    prayer_date.year() as u32,
                    &solar_time.sunset,
                    parameters.shafaq,
                )
            } else {
                let portion = parameters.night_portions().1;
                let night_fraction = portion * (night.num_seconds() as f64);

                solar_time
                    .clone()
                    .sunset
                    .checked_add_signed(Duration::try_seconds(night_fraction as i64).unwrap())
                    .unwrap()
            };

            let isha = if parameters.method == Method::MoonsightingCommittee && coordinates.latitude >= 55.0 {
                // special case for moonsighting committee above latitude 55
                let night_fraction = night.num_seconds() / 7;
                solar_time
                    .sunset
                    .checked_add_signed(Duration::try_seconds(night_fraction).unwrap())
                    .unwrap()
            } else {
                solar_time.time_for_solar_angle(Angle::new(-parameters.isha_angle), true)
            };

            if isha > safe_isha {
                safe_isha
            } else {
                isha
            }
        }
        .adjust_time(parameters.time_adjustments(Prayer::Isha))
    }

    fn calculate_qiyam(
        current_maghrib: &DateTime<Tz>,
        parameters: &Parameters,
        solar_time: &SolarTime<Tz>,
        coordinates: Coordinates,
        prayer_date: &DateTime<Tz>,
    ) -> (DateTime<Tz>, DateTime<Tz>, DateTime<Tz>) {
        let tomorrow = prayer_date.tomorrow();
        let solar_time_tomorrow = SolarTime::new(&tomorrow, coordinates);
        let night = solar_time_tomorrow.sunrise.signed_duration_since(&solar_time.sunset);

        let tomorrow_fajr = Self::calculate_fajr(parameters, solar_time, night, coordinates, prayer_date);
        let night_duration = tomorrow_fajr
            .clone()
            .signed_duration_since(current_maghrib.clone())
            .num_seconds() as f64;
        let middle_night_portion = (night_duration / 2.0) as i64;
        let last_third_portion = (night_duration * (2.0 / 3.0)) as i64;
        let middle_of_night = current_maghrib
            .clone()
            .checked_add_signed(Duration::try_seconds(middle_night_portion).unwrap())
            .unwrap()
            .rounded_minute(Rounding::Nearest);
        let last_third_of_night = current_maghrib
            .clone()
            .checked_add_signed(Duration::try_seconds(last_third_portion).unwrap())
            .unwrap()
            .rounded_minute(Rounding::Nearest);

        (middle_of_night, last_third_of_night, tomorrow_fajr)
    }
}

/// A builder for the [`PrayerTimes`](struct.PrayerTimes.html) struct.
pub struct PrayerSchedule<Tz: TimeZone> {
    date: Option<DateTime<Tz>>,
    coordinates: Option<Coordinates>,
    params: Option<Parameters>,
}

impl<Tz: TimeZone> Default for PrayerSchedule<Tz> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Tz: TimeZone> PrayerSchedule<Tz> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            date: None,
            coordinates: None,
            params: None,
        }
    }

    pub fn with_date(&mut self, date: &DateTime<Tz>) -> &mut Self {
        self.date = Some(date.clone());
        self
    }

    pub fn with_coordinates(&mut self, location: Coordinates) -> &mut Self {
        self.coordinates = Some(location);
        self
    }

    pub fn with_parameters(&mut self, params: Parameters) -> &mut Self {
        self.params = Some(params);
        self
    }

    pub fn build(&self) -> Result<PrayerTimes<Tz>, String> {
        match (&self.date, self.coordinates, &self.params) {
            (Some(date), Some(coordinates), Some(params)) => Ok(PrayerTimes::new(date, coordinates, params)),
            (x, y, z) => Err(format!(
                "Required information is needed in order to calculate the prayer times.\n{x:?}\n{y:?}\n{z:?}",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;
    use crate::models::madhab::Madhab;

    #[test]
    fn current_prayer_should_be_fajr() {
        // Given the above DateTime, the Fajr prayer is at 2015-07-12T08:42:00Z
        let local_date = Utc.with_ymd_and_hms(2015, 7, 12, 9, 0, 0).unwrap();
        let params = Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(&local_date, coordinates, &params);
        let current_prayer_time = local_date.with_timezone(&Utc);

        assert_eq!(times.current_time(&current_prayer_time), Prayer::Fajr);
    }

    #[test]
    fn current_prayer_should_be_sunrise() {
        // Given the below DateTime, sunrise is at 2015-07-12T10:08:00Z
        let local_date = Utc.with_ymd_and_hms(2015, 7, 12, 11, 0, 0).unwrap();
        let params = Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(&local_date, coordinates, &params);
        let current_prayer_time = local_date.with_timezone(&Utc);

        assert_eq!(times.current_time(&current_prayer_time), Prayer::Sunrise);
    }

    #[test]
    fn current_prayer_should_be_dhuhr() {
        // Given the above DateTime, dhuhr prayer is at 2015-07-12T17:21:00Z
        let local_date = Utc.with_ymd_and_hms(2015, 7, 12, 19, 0, 0).unwrap();
        let params = Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(&local_date, coordinates, &params);
        let current_prayer_time = local_date.with_timezone(&Utc);

        assert_eq!(times.current_time(&current_prayer_time), Prayer::Dhuhr);
    }

    #[test]
    fn current_prayer_should_be_asr() {
        // Given the below DateTime, asr is at 2015-07-12T22:22:00Z
        let local_date = Utc.with_ymd_and_hms(2015, 7, 12, 22, 26, 0).unwrap();
        let params = Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(&local_date, coordinates, &params);
        let current_prayer_time = local_date.with_timezone(&Utc);

        assert_eq!(times.current_time(&current_prayer_time), Prayer::Asr);
    }

    #[test]
    fn current_prayer_should_be_maghrib() {
        // Given the below DateTime, maghrib is at 2015-07-13T00:32:00Z
        let local_date = Utc.with_ymd_and_hms(2015, 7, 12, 0, 0, 0).unwrap();
        let params = Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(&local_date, coordinates, &params);
        let current_prayer_time = Utc.with_ymd_and_hms(2015, 7, 13, 1, 0, 0).unwrap();

        assert_eq!(times.current_time(&current_prayer_time), Prayer::Maghrib);
    }

    #[test]
    fn current_prayer_should_be_isha() {
        // Given the below DateTime, isha is at 2015-07-13T01:57:00Z
        let local_date = Utc.with_ymd_and_hms(2015, 7, 12, 0, 0, 0).unwrap();
        let params = Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(&local_date, coordinates, &params);
        let current_prayer_time = Utc.with_ymd_and_hms(2015, 7, 13, 2, 0, 0).unwrap();

        assert_eq!(times.current_time(&current_prayer_time), Prayer::Isha);
    }

    #[test]
    fn current_prayer_should_be_none() {
        let local_date = Utc.with_ymd_and_hms(2015, 7, 12, 8, 0, 0).unwrap();
        let params = Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(&local_date, coordinates, &params);
        let current_prayer_time = local_date.with_timezone(&Utc);

        assert_eq!(times.current_time(&current_prayer_time), Prayer::Qiyam);
    }

    #[test]
    fn calculate_times_for_moonsighting_method() {
        let date = Utc.with_ymd_and_hms(2016, 1, 31, 0, 0, 0).unwrap();
        let params = Parameters::from_method(Method::MoonsightingCommittee).with_madhab(Madhab::Shafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let result = PrayerSchedule::new()
            .with_date(&date)
            .with_coordinates(coordinates)
            .with_parameters(params)
            .build();

        match result {
            Ok(schedule) => {
                // fajr    = 2016-01-31 10:48:00 UTC
                // sunrise = 2016-01-31 12:16:00 UTC
                // dhuhr   = 2016-01-31 17:33:00 UTC
                // asr     = 2016-01-31 20:20:00 UTC
                // maghrib = 2016-01-31 22:43:00 UTC
                // isha    = 2016-02-01 00:05:00 UTC
                assert_eq!(schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(), "10:48 AM");
                assert_eq!(
                    schedule.time(Prayer::Sunrise).format("%-l:%M %p").to_string(),
                    "12:16 PM"
                );
                assert_eq!(schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(), "5:33 PM");
                assert_eq!(schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(), "8:20 PM");
                assert_eq!(
                    schedule.time(Prayer::Maghrib).format("%-l:%M %p").to_string(),
                    "10:43 PM"
                );
                assert_eq!(schedule.time(Prayer::Isha).format("%-l:%M %p").to_string(), "12:05 AM");
            }

            Err(_err) => unreachable!(),
        }
    }

    #[test]
    fn calculate_times_for_moonsighting_method_with_high_latitude() {
        let date = Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap();
        let params = Parameters::from_method(Method::MoonsightingCommittee).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(59.9094, 10.7349);
        let result = PrayerSchedule::new()
            .with_date(&date)
            .with_coordinates(coordinates)
            .with_parameters(params)
            .build();

        match result {
            Ok(schedule) => {
                // fajr    = 2016-01-01 06:34:00 UTC
                // sunrise = 2016-01-01 08:19:00 UTC
                // dhuhr   = 2016-01-01 11:25:00 UTC
                // asr     = 2016-01-01 12:36:00 UTC
                // maghrib = 2016-01-01 14:25:00 UTC
                // isha    = 2016-01-01 16:02:00 UTC
                assert_eq!(schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(), "6:34 AM");
                assert_eq!(
                    schedule.time(Prayer::Sunrise).format("%-l:%M %p").to_string(),
                    "8:19 AM"
                );
                assert_eq!(schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(), "11:25 AM");
                assert_eq!(schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(), "12:36 PM");
                assert_eq!(
                    schedule.time(Prayer::Maghrib).format("%-l:%M %p").to_string(),
                    "2:25 PM"
                );
                assert_eq!(schedule.time(Prayer::Isha).format("%-l:%M %p").to_string(), "4:02 PM");
            }

            Err(_err) => unreachable!(),
        }
    }
}
