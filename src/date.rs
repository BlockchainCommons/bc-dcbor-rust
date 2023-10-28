
use chrono::{DateTime, Utc, TimeZone, SecondsFormat, NaiveDate, NaiveDateTime, Timelike};

use crate::{CBORCodable, CBOREncodable, CBORTaggedEncodable, Tag, CBOR, CBORDecodable, CBORTaggedDecodable, CBORTaggedCodable, CBORTagged};

use anyhow::bail;

/// A CBOR-friendly representation of a date and time.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub fn new_from_string(value: &str) -> anyhow::Result<Self> {
        // try parsing as DateTime
        if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
            return Ok(Self::from_datetime(dt.with_timezone(&Utc)));
        }

        // try parsing as just a date (with assumed zero time)
        if let Ok(d) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            let dt = NaiveDateTime::new(d, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return Ok(Self::from_datetime(DateTime::from_naive_utc_and_offset(dt, Utc)));
        }

        bail!("Invalid date string")
    }

    /// Creates a new `Date` containing the current date and time.
    pub fn now() -> Self {
        Self::from_datetime(Utc::now())
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

impl Default for Date {
    fn default() -> Self {
        Self::now()
    }
}

impl TryFrom<&str> for Date {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_from_string(value)
    }
}

impl AsRef<Date> for Date {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl CBOREncodable for Date {
    fn cbor(&self) -> CBOR {
        self.tagged_cbor()
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.tagged_cbor().cbor_data()
    }
}

impl CBORDecodable for Date {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        Self::from_tagged_cbor(cbor)
    }
}

impl CBORCodable for Date { }

impl CBORTagged for Date {
    const CBOR_TAG: Tag = Tag::new(1);
}

impl CBORTaggedEncodable for Date {
    fn untagged_cbor(&self) -> CBOR {
        self.timestamp().cbor()
    }
}

impl CBORTaggedDecodable for Date {
    fn from_untagged_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        let n = f64::from_cbor(cbor)?;
        Ok(Date::from_timestamp(n))
    }
}

impl CBORTaggedCodable for Date { }

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.datetime().to_rfc3339_opts(SecondsFormat::Secs, true).as_str())
    }
}
