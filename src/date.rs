use chrono::{DateTime, Utc, TimeZone, SecondsFormat};

use crate::{CBORCodable, CBOREncodable, CBORTaggedEncodable, Tag, CBOR, CBORDecodable, decode_error::DecodeError, CBORTaggedDecodable, CBORTaggedCodable};

/// A CBOR-friendly representation of a date and time.
#[derive(Debug, Clone)]
pub struct Date(DateTime<Utc>);

impl Date {
    /// Creates a new `Date` from the given chrono `DateTime`.
    pub fn from_datetime(date_time: DateTime<Utc>) -> Self {
        Date(date_time)
    }

    /// Creates a new `Date` from seconds since (or before) the Unix epoch.
    pub fn from_timestamp(seconds_since_unix_epoch: i64) -> Self {
        Self::from_datetime(Utc.timestamp_opt(seconds_since_unix_epoch, 0).unwrap())
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
    pub fn timestamp(&self) -> i64 {
        self.datetime().timestamp()
    }

    /// Returns a string with the ISO-8601 (RFC-3339) representation of the date.
    pub fn to_string(&self) -> String {
        self.datetime().to_rfc3339_opts(SecondsFormat::Secs, true)
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
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError> {
        Self::from_tagged_cbor(cbor)
    }
}

impl CBORCodable for Date { }

impl CBORTaggedEncodable for Date {
    const CBOR_TAG: Tag = Tag::new(1);

    fn untagged_cbor(&self) -> CBOR {
        self.timestamp().cbor()
    }
}

impl CBORTaggedDecodable for Date {
    const CBOR_TAG: Tag = Tag::new(1);

    fn from_untagged_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError> {
        let a = i64::from_cbor(cbor)?;
        Ok(Box::new(Date::from_timestamp(*a)))
    }
}

impl CBORTaggedCodable for Date { }

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
