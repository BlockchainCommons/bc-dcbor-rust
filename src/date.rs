use chrono::{DateTime, Utc, TimeZone, SecondsFormat};

use crate::{CBORCodable, CBOREncodable, CBORTaggedEncodable, Tag, CBOR, CBORDecodable, decode_error::DecodeError, CBORTaggedDecodable, CBORTaggedCodable};

#[derive(Debug, Clone)]
pub struct Date(DateTime<Utc>);

impl Date {
    /// Creates a new `Date` from the given `DateTime`.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Date(dt)
    }

    /// Creates a new `Date` from seconds since (or before) the Unix epoch.
    pub fn from_timestamp(ts: i64) -> Self {
        Self::from_datetime(Utc.timestamp_opt(ts, 0).unwrap())
    }

    pub fn now() -> Self {
        Self::from_datetime(Utc::now())
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.datetime().timestamp()
    }

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
    fn tag() -> Tag {
        Tag::new_opt(1, None)
    }

    fn untagged_cbor(&self) -> CBOR {
        self.timestamp().cbor()
    }
}

impl CBORTaggedDecodable for Date {
    fn tag() -> Tag {
        Tag::new_opt(1, None)
    }

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
