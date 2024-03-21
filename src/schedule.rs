// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

//! # Prayer Schedule
//!
//! This module provides the main objects that are used for calculating
//! the prayer times.

use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Utc};

use crate::{
    astronomy::{
        ops,
        solar::SolarTime,
        unit::{Angle, Coordinates, Stride},
    },
    models::{
        event::{Event, Prayer, Reason},
        method::Method,
        parameters::Parameters,
    },
};

/// A data struct to hold the timing for all
/// prayers.
#[derive(Debug, Clone)]
pub struct Times<Tz: TimeZone> {
    midnight_yesterday: DateTime<Tz>,
    qiyam_yesterday: DateTime<Tz>,
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

struct EventInfo<'d, Tz: TimeZone> {
    name: Event,
    time: &'d DateTime<Tz>,
}

impl<'d, Tz: TimeZone> EventInfo<'d, Tz> {
    const fn prayer(&'d self) -> Event {
        self.name
    }

    const fn time(&'d self) -> &DateTime<Tz> {
        self.time
    }
}

pub struct ExpectedPrayers<'d, Tz: TimeZone> {
    current: EventInfo<'d, Tz>,
    next: EventInfo<'d, Tz>,
}

impl<'d, Tz: TimeZone> ExpectedPrayers<'d, Tz> {
    pub const fn current_event(&'d self) -> Event {
        self.current.prayer()
    }

    pub const fn current_time(&'d self) -> &DateTime<Tz> {
        self.current.time()
    }

    pub const fn next_event(&'d self) -> Event {
        self.next.prayer()
    }

    pub const fn next_time(&'d self) -> &DateTime<Tz> {
        self.next.time()
    }
}

impl<Tz: TimeZone> Times<Tz> {
    #[must_use]
    pub fn new(date: &DateTime<Tz>, coordinates: &Coordinates, parameters: &Parameters) -> Self {
        let tomorrow = date.tomorrow();
        let yesterday = date.yesterday();

        let solar_time_yesterday = SolarTime::new(&yesterday, coordinates);
        let solar_time = SolarTime::new(date, coordinates);
        let solar_time_tomorrow = SolarTime::new(&tomorrow, coordinates);

        let night = calculate_night(&solar_time_tomorrow, &solar_time);

        let fajr = calculate_fajr(parameters, &solar_time, night, coordinates, date);
        let sunrise = calculate_sunrise(&solar_time, parameters);
        let dhuhr = calculate_dhuhr(&solar_time, parameters);
        let asr = calculate_asr(&solar_time, parameters);
        let maghrib = calculate_maghrib(&solar_time, parameters);
        let maghrib_yesterday = calculate_maghrib(&solar_time_yesterday, parameters);
        let isha = calculate_isha(parameters, &solar_time, night, coordinates, date);

        // Calculate the middle of the night and qiyam times
        let (midnight, qiyam, fajr_tomorrow) =
            calculate_qiyam(&maghrib, parameters, &solar_time_tomorrow, coordinates, &tomorrow);

        let (midnight_yesterday, qiyam_yesterday, _) =
            calculate_qiyam(&maghrib_yesterday, parameters, &solar_time, coordinates, date);

        Self {
            midnight_yesterday,
            qiyam_yesterday,
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

    pub fn display(&self, current_time: &DateTime<Tz>) -> String
    where
        Tz::Offset: std::fmt::Display,
    {
        let (hours, minutes) = self.time_remaining(current_time);

        let current_date = current_time.date_naive();

        let dhuhr_name = if current_date.weekday() == chrono::Weekday::Fri {
            Event::Prayer(Prayer::Dhuhr).friday_name()
        } else {
            Event::Prayer(Prayer::Dhuhr).name()
        };

        let current_name = if current_date.weekday() == chrono::Weekday::Fri {
            self.current(current_time).0.friday_name()
        } else {
            self.current(current_time).0.name()
        };

        let next_name = if current_date.weekday() == chrono::Weekday::Fri {
            self.next(current_time).0.friday_name()
        } else {
            self.next(current_time).0.name()
        };

        let prayer_table = tabled::col![
            current_time.format("%A, %-d %B, %C%y %H:%M:%S"),
            tabled::row![
                tabled::col!["Fajr", dhuhr_name, "Asr", "Maghrib", "Isha"],
                tabled::col![
                    self.fajr.time().format("%H:%M"),
                    self.dhuhr.time().format("%H:%M"),
                    self.asr.time().format("%H:%M"),
                    self.maghrib.time().format("%H:%M"),
                    self.isha.time().format("%H:%M"),
                ],
                tabled::col!["Current Prayer", "Next Prayer", "Time Left", "Midnight", "Qiyam"],
                tabled::col![
                    current_name,
                    next_name,
                    format!("{hours}h {minutes}m"),
                    self.midnight.time().format("%H:%M"),
                    self.qiyam.time().format("%H:%M")
                ]
            ]
        ];

        prayer_table.to_string()
    }

    #[must_use]
    pub const fn fajr(&self) -> &DateTime<Tz> {
        &self.fajr
    }

    #[must_use]
    pub const fn sunrise(&self) -> &DateTime<Tz> {
        &self.sunrise
    }

    #[must_use]
    pub const fn dhuhr(&self) -> &DateTime<Tz> {
        &self.dhuhr
    }

    #[must_use]
    pub const fn asr(&self) -> &DateTime<Tz> {
        &self.asr
    }

    #[must_use]
    pub const fn maghrib(&self) -> &DateTime<Tz> {
        &self.maghrib
    }

    #[must_use]
    pub const fn isha(&self) -> &DateTime<Tz> {
        &self.isha
    }

    #[must_use]
    pub const fn midnight(&self) -> &DateTime<Tz> {
        &self.midnight
    }

    #[must_use]
    pub const fn qiyam(&self) -> &DateTime<Tz> {
        &self.qiyam
    }

    pub fn expected(&self, time: &DateTime<Tz>) -> ExpectedPrayers<Tz> {
        let (p1, d1) = self.current(time);
        let (p2, d2) = self.next(time);

        ExpectedPrayers {
            current: EventInfo { name: p1, time: d1 },
            next: EventInfo { name: p2, time: d2 },
        }
    }

    fn current(&self, time: &DateTime<Tz>) -> (Event, &DateTime<Tz>) {
        if self.fajr_tomorrow.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Prayer(Prayer::Fajr), &self.fajr_tomorrow)
        } else if self.qiyam.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Qiyam, &self.qiyam)
        } else if self.midnight.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Restricted(Reason::AfterMidnight), &self.midnight)
        } else if self.isha.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Prayer(Prayer::Isha), &self.isha)
        } else if self.maghrib.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Prayer(Prayer::Maghrib), &self.maghrib)
        } else if (0..=20).contains(&self.maghrib.clone().signed_duration_since(time).num_minutes()) {
            (Event::Restricted(Reason::DuringSunset), &self.asr)
        } else if self.asr.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Prayer(Prayer::Asr), &self.asr)
        } else if self.dhuhr.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Prayer(Prayer::Dhuhr), &self.dhuhr)
        } else if self.sunrise.clone().signed_duration_since(time).num_minutes() <= -20 {
            (Event::Sunrise, &self.sunrise)
        } else if (-20..=0).contains(&self.sunrise.clone().signed_duration_since(time).num_minutes()) {
            (Event::Restricted(Reason::DuringSunrise), &self.sunrise)
        } else if self.fajr.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Prayer(Prayer::Fajr), &self.fajr)
        } else if self.qiyam_yesterday.clone().signed_duration_since(time).num_seconds() <= 0 {
            (Event::Qiyam, &self.qiyam_yesterday)
        } else {
            (Event::Restricted(Reason::AfterMidnight), &self.midnight_yesterday)
        }
    }

    #[must_use]
    fn next(&self, time: &DateTime<Tz>) -> (Event, &DateTime<Tz>) {
        match self.current(time).0 {
            Event::Prayer(Prayer::Fajr) => (Event::Sunrise, &self.sunrise),
            Event::Sunrise => {
                // There's roughly a 20 minute window during sunrise where it's
                // forbidden to give Fajr prayer
                if (-20..0).contains(&self.sunrise.clone().signed_duration_since(time).num_minutes()) {
                    (Event::Restricted(Reason::DuringSunrise), &self.dhuhr)
                } else {
                    (Event::Prayer(Prayer::Dhuhr), &self.dhuhr)
                }
            }
            Event::Restricted(Reason::DuringSunrise) => (Event::Prayer(Prayer::Dhuhr), &self.dhuhr),
            Event::Prayer(Prayer::Dhuhr) => (Event::Prayer(Prayer::Asr), &self.asr),
            Event::Prayer(Prayer::Asr) => {
                // Similarly, there's a 20 minute window during sunset where
                // it's forbidden to give Asr prayer
                if (0..=20).contains(&self.maghrib.clone().signed_duration_since(time).num_minutes()) {
                    (Event::Restricted(Reason::DuringSunset), &self.maghrib)
                } else {
                    (Event::Prayer(Prayer::Maghrib), &self.maghrib)
                }
            }
            Event::Restricted(Reason::DuringSunset) => (Event::Prayer(Prayer::Maghrib), &self.maghrib),
            Event::Prayer(Prayer::Maghrib) => (Event::Prayer(Prayer::Isha), &self.isha),
            // It is forbidden to pray past Islamic Midnight
            // and before the period of Qiyam
            Event::Prayer(Prayer::Isha) => (
                Event::Restricted(Reason::AfterMidnight),
                if time.date_naive() == self.isha.date_naive() {
                    &self.midnight
                } else {
                    &self.midnight_yesterday
                },
            ),
            Event::Restricted(Reason::AfterMidnight) => (
                Event::Qiyam,
                if time.date_naive() == self.midnight.date_naive() {
                    &self.qiyam
                } else {
                    &self.qiyam_yesterday
                },
            ),
            Event::Qiyam => (Event::Prayer(Prayer::Fajr), &self.fajr),
        }
    }

    #[must_use]
    pub fn time_remaining(&self, now: &DateTime<Tz>) -> (i64, i64) {
        let (_, current_time) = self.current(now);
        let (_, next_time) = self.next(current_time);

        let now_to_next = next_time.clone().signed_duration_since(now);
        let hours = now_to_next.num_hours();
        let minutes = now_to_next.num_minutes() % 60;

        (hours, minutes)
    }
}

fn calculate_night<Tz: TimeZone>(solar_time_tomorrow: &SolarTime<Tz>, solar_time: &SolarTime<Tz>) -> chrono::TimeDelta {
    solar_time_tomorrow
        .clone()
        .sunrise
        .signed_duration_since(&solar_time.sunset)
}

fn calculate_maghrib<Tz: TimeZone>(solar_time: &SolarTime<Tz>, parameters: &Parameters) -> DateTime<Tz> {
    ops::adjust_time(
        &solar_time.sunset,
        parameters.time_adjustments(Event::Prayer(Prayer::Maghrib)),
    )
    .rounded_minute(parameters.rounding)
}

fn calculate_asr<Tz: TimeZone>(solar_time: &SolarTime<Tz>, parameters: &Parameters) -> DateTime<Tz> {
    solar_time
        .afternoon(parameters.madhab.shadow().into())
        .adjust_time(parameters.time_adjustments(Event::Prayer(Prayer::Asr)))
        .rounded_minute(parameters.rounding)
}

fn calculate_dhuhr<Tz: TimeZone>(solar_time: &SolarTime<Tz>, parameters: &Parameters) -> DateTime<Tz> {
    solar_time
        .transit
        .adjust_time(parameters.time_adjustments(Event::Prayer(Prayer::Dhuhr)))
        .rounded_minute(parameters.rounding)
}

fn calculate_fajr<Tz: TimeZone>(
    parameters: &Parameters,
    solar_time: &SolarTime<Tz>,
    night: Duration,
    coordinates: &Coordinates,
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

    fajr.adjust_time(parameters.time_adjustments(Event::Prayer(Prayer::Fajr)))
        .rounded_minute(parameters.rounding)
}

fn calculate_isha<Tz: TimeZone>(
    parameters: &Parameters,
    solar_time: &SolarTime<Tz>,
    night: Duration,
    coordinates: &Coordinates,
    prayer_date: &DateTime<Tz>,
) -> DateTime<Tz> {
    if parameters.isha_interval > 0 {
        solar_time
            .clone()
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
                .clone()
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
    .adjust_time(parameters.time_adjustments(Event::Prayer(Prayer::Isha)))
    .rounded_minute(parameters.rounding)
}

fn calculate_qiyam<Tz: TimeZone>(
    current_maghrib: &DateTime<Tz>,
    parameters: &Parameters,
    solar_time: &SolarTime<Tz>,
    coordinates: &Coordinates,
    prayer_date: &DateTime<Tz>,
) -> (DateTime<Tz>, DateTime<Tz>, DateTime<Tz>) {
    let tomorrow = prayer_date.tomorrow();
    let solar_time_tomorrow = SolarTime::new(&tomorrow, coordinates);
    let night = solar_time_tomorrow.sunrise.signed_duration_since(&solar_time.sunset);

    let tomorrow_fajr = calculate_fajr(parameters, solar_time, night, coordinates, prayer_date);
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
        .rounded_minute(parameters.rounding);
    let last_third_of_night = current_maghrib
        .clone()
        .checked_add_signed(Duration::try_seconds(last_third_portion).unwrap())
        .unwrap()
        .rounded_minute(parameters.rounding);

    (middle_of_night, last_third_of_night, tomorrow_fajr)
}

fn calculate_sunrise<Tz: TimeZone>(solar_time: &SolarTime<Tz>, parameters: &Parameters) -> DateTime<Tz> {
    solar_time
        .sunrise
        .adjust_time(parameters.time_adjustments(Event::Sunrise))
        .rounded_minute(parameters.rounding)
}

/// A builder for the [`PrayerTimes`](struct.PrayerTimes.html) struct.
pub struct Schedule<Tz: TimeZone> {
    date: Option<DateTime<Tz>>,
    coordinates: Option<Coordinates>,
    params: Option<Parameters>,
}

impl<Tz: TimeZone> Default for Schedule<Tz> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Tz: TimeZone> Schedule<Tz> {
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

    pub fn build(&self) -> Result<Times<Tz>, String> {
        match (&self.date, &self.coordinates, &self.params) {
            (Some(date), Some(coordinates), Some(params)) => Ok(Times::new(date, coordinates, params)),
            (x, y, z) => Err(format!(
                "Required information is needed in order to calculate the prayer times.\n{x:?}\n{y:?}\n{z:?}",
            )),
        }
    }
}

impl Schedule<Local> {
    #[must_use]
    pub fn now() -> Self {
        Self {
            date: Some(Local::now()),
            coordinates: None,
            params: None,
        }
    }
}

impl Schedule<Utc> {
    #[must_use]
    pub fn now() -> Self {
        Self {
            date: Some(Utc::now()),
            coordinates: None,
            params: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use rstest::{fixture, rstest};

    use super::*;
    use crate::models::madhab::Madhab;

    #[fixture]
    #[once]
    fn position() -> Coordinates {
        Coordinates::new(35.7750, -78.6336)
    }

    #[fixture]
    #[once]
    fn parameters() -> Parameters {
        Parameters::from_method(Method::NorthAmerica).with_madhab(Madhab::Hanafi)
    }

    #[rstest]
    #[case::should_be_fajr(
        Utc.with_ymd_and_hms(2015, 7, 12, 9, 0, 0).unwrap(),
        None,
        Event::Prayer(Prayer::Fajr)
    )]
    #[case::should_be_sunrise(
        Utc.with_ymd_and_hms(2015, 7, 12, 11, 0, 0).unwrap(),
        None,
        Event::Sunrise
    )]
    #[case::should_be_dhuhr(
        Utc.with_ymd_and_hms(2015, 7, 12, 19, 0, 0).unwrap(),
        None,
        Event::Prayer(Prayer::Dhuhr)
    )]
    #[case::should_be_asr(
        Utc.with_ymd_and_hms(2015, 7, 12, 22, 26, 0).unwrap(),
        None,
        Event::Prayer(Prayer::Asr)
    )]
    #[case::should_be_maghrib(
        Utc.with_ymd_and_hms(2015, 7, 12, 0, 0, 0).unwrap(),
        Some(Utc.with_ymd_and_hms(2015, 7, 13, 1, 0, 0).unwrap()),
        Event::Prayer(Prayer::Maghrib)
    )]
    #[case::should_be_isha(
        Utc.with_ymd_and_hms(2015, 7, 12, 0, 0, 0).unwrap(),
        Some(Utc.with_ymd_and_hms(2015, 7, 13, 2, 0, 0).unwrap()),
        Event::Prayer(Prayer::Isha)
    )]
    #[case::should_be_qiyam(
        Utc.with_ymd_and_hms(2015, 7, 12, 8, 0, 0).unwrap(),
        None,
        Event::Qiyam
    )]
    fn test_current_prayer(
        position: &Coordinates,
        parameters: &Parameters,
        #[case] first_timestamp: DateTime<Utc>,
        #[case] second_timestamp: Option<DateTime<Utc>>,
        #[case] expected_prayer: Event,
    ) {
        // Given the above DateTime, the Fajr prayer is at 2015-07-12T08:42:00Z
        let times = Times::new(&first_timestamp, position, parameters);
        let current_prayer_time = second_timestamp.map_or_else(
            || first_timestamp.with_timezone(&Utc),
            |second_timestamp| second_timestamp,
        );

        assert_eq!(times.current(&current_prayer_time).0, expected_prayer);
    }

    #[test]
    fn calculate_times_for_moonsighting_method() {
        let date = Utc.with_ymd_and_hms(2016, 1, 31, 0, 0, 0).unwrap();
        let params = Parameters::from_method(Method::MoonsightingCommittee).with_madhab(Madhab::Shafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let result = Schedule::new()
            .with_date(&date)
            .with_coordinates(coordinates)
            .with_parameters(params)
            .build();

        if let Ok(schedule) = result {
            // fajr    = 2016-01-31 10:48:00 UTC
            // sunrise = 2016-01-31 12:16:00 UTC
            // dhuhr   = 2016-01-31 17:33:00 UTC
            // asr     = 2016-01-31 20:20:00 UTC
            // maghrib = 2016-01-31 22:43:00 UTC
            // isha    = 2016-02-01 00:05:00 UTC
            assert_eq!(schedule.fajr.format("%-l:%M %p").to_string(), "10:48 AM");
            assert_eq!(schedule.sunrise.format("%-l:%M %p").to_string(), "12:16 PM");
            assert_eq!(schedule.dhuhr.format("%-l:%M %p").to_string(), "5:33 PM");
            assert_eq!(schedule.asr.format("%-l:%M %p").to_string(), "8:20 PM");
            assert_eq!(schedule.maghrib.format("%-l:%M %p").to_string(), "10:43 PM");
            assert_eq!(schedule.isha.format("%-l:%M %p").to_string(), "12:05 AM");
        } else {
            unreachable!()
        }
    }

    #[test]
    fn calculate_times_for_moonsighting_method_with_high_latitude() {
        let date = Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap();
        let params = Parameters::from_method(Method::MoonsightingCommittee).with_madhab(Madhab::Hanafi);
        let coordinates = Coordinates::new(59.9094, 10.7349);
        let result = Schedule::new()
            .with_date(&date)
            .with_coordinates(coordinates)
            .with_parameters(params)
            .build();

        if let Ok(schedule) = result {
            // fajr    = 2016-01-01 06:34:00 UTC
            // sunrise = 2016-01-01 08:19:00 UTC
            // dhuhr   = 2016-01-01 11:25:00 UTC
            // asr     = 2016-01-01 12:36:00 UTC
            // maghrib = 2016-01-01 14:25:00 UTC
            // isha    = 2016-01-01 16:02:00 UTC
            assert_eq!(schedule.fajr.format("%-l:%M %p").to_string(), "6:34 AM");
            assert_eq!(schedule.sunrise.format("%-l:%M %p").to_string(), "8:19 AM");
            assert_eq!(schedule.dhuhr.format("%-l:%M %p").to_string(), "11:25 AM");
            assert_eq!(schedule.asr.format("%-l:%M %p").to_string(), "12:36 PM");
            assert_eq!(schedule.maghrib.format("%-l:%M %p").to_string(), "2:25 PM");
            assert_eq!(schedule.isha.format("%-l:%M %p").to_string(), "4:02 PM");
        } else {
            unreachable!()
        }
    }
}
