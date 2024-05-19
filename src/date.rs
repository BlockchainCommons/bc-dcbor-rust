import_stdlib!();

use ops::{Add, Sub};

use chrono::{DateTime, Utc, TimeZone, SecondsFormat, NaiveDate, NaiveDateTime, Timelike};

use anyhow::{bail, Error, Result};

use crate::{CBORTaggedEncodable, Tag, CBOR, CBORTaggedDecodable, CBORTagged};

/// A CBOR-friendly representation of a date and time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date(DateTime<Utc>);

impl Date {
    /// Creates a new `Date` from the given chrono `DateTime`.
    pub fn from_datetime(date_time: DateTime<Utc>) -> Self {
        Date(date_time)
    }

    /// Creates a new `Date` from seconds since (or before) the Unix epoch.
    pub fn from_timestamp(seconds_since_unix_epoch: f64) -> Self {
        let whole_seconds_since_unix_epoch = seconds_since_unix_epoch.trunc() as i64;
        let nsecs = (seconds_since_unix_epoch.fract() * 1_000_000_000.0) as u32;
        Self::from_datetime(Utc.timestamp_opt(whole_seconds_since_unix_epoch, nsecs).unwrap())
    }

    /// Creates a new `Date` from a string containing an ISO-8601 (RFC-3339) date (with or without time).
    pub fn from_string(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        // try parsing as DateTime
        if let Ok(dt) = DateTime::parse_from_rfc3339(&value) {
            return Ok(Self::from_datetime(dt.with_timezone(&Utc)));
        }

        // try parsing as just a date (with assumed zero time)
        if let Ok(d) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
            let dt = NaiveDateTime::new(d, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return Ok(Self::from_datetime(DateTime::from_naive_utc_and_offset(dt, Utc)));
        }

        bail!("Invalid date string")
    }

    /// Creates a new `Date` containing the current date and time.
    pub fn now() -> Self {
        Self::from_datetime(Utc::now())
    }

    /// Creates a new `Date` containing the current date and time plus the given duration.
    pub fn with_duration_from_now(duration: Duration) -> Self {
        Self::now() + duration
    }

    /// Returns the underlying chrono `DateTime` struct.
    pub fn datetime(&self) -> DateTime<Utc> {
        self.0
    }

    /// Returns the `Date` as the number of seconds since the Unix epoch.
    pub fn timestamp(&self) -> f64 {
        let d = self.datetime();
        let whole_seconds_since_unix_epoch = d.timestamp();
        let nsecs = d.nanosecond();
        (whole_seconds_since_unix_epoch as f64) + ((nsecs as f64) / 1_000_000_000.0)
    }
}

// Support adding seconds as f64
impl Add<f64> for Date {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self::from_timestamp(self.timestamp() + rhs)
    }
}

// Support subtracting seconds as f64
impl Sub<f64> for Date {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::from_timestamp(self.timestamp() - rhs)
    }
}

// Support adding a duration
impl Add<Duration> for Date {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self::from_timestamp(self.timestamp() + rhs.as_secs_f64())
    }
}

// Support subtracting a duration
impl Sub<Duration> for Date {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self::from_timestamp(self.timestamp() - rhs.as_secs_f64())
    }
}

// Support subtracting another date and returning the number of seconds as f64
impl Sub for Date {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.timestamp() - rhs.timestamp()
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::now()
    }
}

impl TryFrom<&str> for Date {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value)
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(value: DateTime<Utc>) -> Self {
        Self::from_datetime(value)
    }
}

impl From<Date> for CBOR {
    fn from(value: Date) -> Self {
        value.tagged_cbor()
    }
}

impl AsRef<Date> for Date {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl TryFrom<CBOR> for Date {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        Self::from_tagged_cbor(cbor)
    }
}

impl CBORTagged for Date {
    fn cbor_tags() -> Vec<Tag> {
        vec![Tag::new(1)]
    }
}

impl CBORTaggedEncodable for Date {
    fn untagged_cbor(&self) -> CBOR {
        self.timestamp().into()
    }
}

impl CBORTaggedDecodable for Date {
    fn from_untagged_cbor(cbor: CBOR) -> Result<Self> {
        let n = cbor.clone().try_into()?;
        Ok(Date::from_timestamp(n))
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.datetime().to_rfc3339_opts(SecondsFormat::Secs, true).as_str())
    }
}
