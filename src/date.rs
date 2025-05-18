import_stdlib!();

#[cfg(feature = "std")]
use std::ops::{ Add, Sub };

#[cfg(not(feature = "std"))]
use core::ops::{ Add, Sub };

use chrono::{ DateTime, Utc, TimeZone, SecondsFormat, NaiveDate, NaiveDateTime, Timelike };

use crate::{ tags_for_values, CBORTagged, CBORTaggedDecodable, CBORTaggedEncodable, Error, Result, Tag, CBOR, TAG_DATE };

/// A CBOR-friendly representation of a date and time.
///
/// The `Date` type provides a wrapper around `chrono::DateTime<Utc>` that supports
/// encoding and decoding to/from CBOR with tag 1, following the CBOR date/time
/// standard specified in [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html#name-date-and-time-tag-1-and-co).
///
/// When encoded to CBOR, dates are represented as tag 1 followed by a numeric value
/// representing the number of seconds since (or before) the Unix epoch (1970-01-01T00:00:00Z).
/// The numeric value can be a positive or negative integer, or a floating-point value
/// for dates with fractional seconds.
///
/// # Features
///
/// - Supports UTC dates with optional fractional seconds
/// - Provides convenient constructors for common date creation patterns
/// - Implements the [`CBORTagged`], [`CBORTaggedEncodable`], and [`CBORTaggedDecodable`] traits
/// - Supports arithmetic operations with durations and between dates
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
/// use dcbor::Date;
///
/// // Create a date from a timestamp (seconds since Unix epoch)
/// let date = Date::from_timestamp(1675854714.0);
///
/// // Create a date from year, month, day
/// let date = Date::from_ymd(2023, 2, 8);
///
/// // Convert to CBOR
/// let cbor = CBOR::from(date);
///
/// // Decode from CBOR
/// let decoded_date: Date = cbor.try_into().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date(DateTime<Utc>);

impl Date {
    /// Creates a new `Date` from the given chrono `DateTime`.
    ///
    /// This method creates a new `Date` instance by wrapping a `chrono::DateTime<Utc>`.
    ///
    /// # Arguments
    ///
    /// * `date_time` - A `DateTime<Utc>` instance to wrap
    ///
    /// # Returns
    ///
    /// A new `Date` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    /// use chrono::{DateTime, Utc};
    ///
    /// let datetime = Utc::now();
    /// let date = Date::from_datetime(datetime);
    /// ```
    pub fn from_datetime(date_time: DateTime<Utc>) -> Self {
        Date(date_time)
    }

    /// Creates a new `Date` from year, month, and day components.
    ///
    /// This method creates a new `Date` with the time set to 00:00:00 UTC.
    ///
    /// # Arguments
    ///
    /// * `year` - The year component (e.g., 2023)
    /// * `month` - The month component (1-12)
    /// * `day` - The day component (1-31)
    ///
    /// # Returns
    ///
    /// A new `Date` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    ///
    /// // Create February 8, 2023
    /// let date = Date::from_ymd(2023, 2, 8);
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the provided components do not form a valid date.
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
        Self::from_datetime(dt)
    }

    /// Creates a new `Date` from year, month, day, hour, minute, and second components.
    ///
    /// # Arguments
    ///
    /// * `year` - The year component (e.g., 2023)
    /// * `month` - The month component (1-12)
    /// * `day` - The day component (1-31)
    /// * `hour` - The hour component (0-23)
    /// * `minute` - The minute component (0-59)
    /// * `second` - The second component (0-59)
    ///
    /// # Returns
    ///
    /// A new `Date` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    ///
    /// // Create February 8, 2023, 15:30:45 UTC
    /// let date = Date::from_ymd_hms(2023, 2, 8, 15, 30, 45);
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the provided components do not form a valid date and time.
    pub fn from_ymd_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32
    ) -> Self {
        let dt = Utc.with_ymd_and_hms(year, month, day, hour, minute, second).unwrap();
        Self::from_datetime(dt)
    }

    /// Creates a new `Date` from seconds since (or before) the Unix epoch.
    ///
    /// This method creates a new `Date` representing the specified number of seconds
    /// since the Unix epoch (1970-01-01T00:00:00Z). Negative values represent times
    /// before the epoch.
    ///
    /// # Arguments
    ///
    /// * `seconds_since_unix_epoch` - Seconds from the Unix epoch (positive or negative),
    ///   which can include a fractional part for sub-second precision
    ///
    /// # Returns
    ///
    /// A new `Date` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    ///
    /// // Create a date from a timestamp
    /// let date = Date::from_timestamp(1675854714.0);
    ///
    /// // Create a date one second before the Unix epoch
    /// let before_epoch = Date::from_timestamp(-1.0);
    ///
    /// // Create a date with fractional seconds
    /// let with_fraction = Date::from_timestamp(1675854714.5);
    /// ```
    pub fn from_timestamp(seconds_since_unix_epoch: f64) -> Self {
        let whole_seconds_since_unix_epoch = seconds_since_unix_epoch.trunc() as i64;
        let nsecs = (seconds_since_unix_epoch.fract() * 1_000_000_000.0) as u32;
        Self::from_datetime(Utc.timestamp_opt(whole_seconds_since_unix_epoch, nsecs).unwrap())
    }

    /// Creates a new `Date` from a string containing an ISO-8601 (RFC-3339) date (with or without time).
    ///
    /// This method parses a string representation of a date or date-time in ISO-8601/RFC-3339 format
    /// and creates a new `Date` instance. It supports both full date-time strings (e.g.,
    /// "2023-02-08T15:30:45Z") and date-only strings (e.g., "2023-02-08").
    ///
    /// # Arguments
    ///
    /// * `value` - A string containing a date or date-time in ISO-8601/RFC-3339 format
    ///
    /// # Returns
    ///
    /// * `Ok(Date)` - A new `Date` instance if parsing succeeds
    /// * `Err` - If the string cannot be parsed as a valid date or date-time
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    ///
    /// // Parse a date-time string
    /// let date = Date::from_string("2023-02-08T15:30:45Z").unwrap();
    ///
    /// // Parse a date-only string (time will be set to 00:00:00)
    /// let date = Date::from_string("2023-02-08").unwrap();
    /// ```
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

        return Err(Error::InvalidDate("Invalid date string".into()));
    }

    /// Creates a new `Date` containing the current date and time.
    ///
    /// # Returns
    ///
    /// A new `Date` instance representing the current UTC date and time
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    ///
    /// let now = Date::now();
    /// ```
    pub fn now() -> Self {
        Self::from_datetime(Utc::now())
    }

    /// Creates a new `Date` containing the current date and time plus the given duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - The duration to add to the current time
    ///
    /// # Returns
    ///
    /// A new `Date` instance representing the current UTC date and time plus the duration
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    /// use std::time::Duration;
    ///
    /// // Get a date 1 hour from now
    /// let one_hour_later = Date::with_duration_from_now(Duration::from_secs(3600));
    /// ```
    pub fn with_duration_from_now(duration: Duration) -> Self {
        Self::now() + duration
    }

    /// Returns the underlying chrono `DateTime` struct.
    ///
    /// This method provides access to the wrapped `chrono::DateTime<Utc>` instance.
    ///
    /// # Returns
    ///
    /// The wrapped `DateTime<Utc>` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    /// use chrono::Datelike;
    ///
    /// let date = Date::now();
    /// let datetime = date.datetime();
    /// let year = datetime.year();
    /// ```
    pub fn datetime(&self) -> DateTime<Utc> {
        self.0
    }

    /// Returns the `Date` as the number of seconds since the Unix epoch.
    ///
    /// This method converts the date to a floating-point number representing the number
    /// of seconds since the Unix epoch (1970-01-01T00:00:00Z). Negative values represent
    /// times before the epoch. The fractional part represents sub-second precision.
    ///
    /// # Returns
    ///
    /// Seconds since the Unix epoch as a `f64`
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    ///
    /// let date = Date::from_ymd(2023, 2, 8);
    /// let timestamp = date.timestamp();
    /// ```
    pub fn timestamp(&self) -> f64 {
        let d = self.datetime();
        let whole_seconds_since_unix_epoch = d.timestamp();
        let nsecs = d.nanosecond();
        (whole_seconds_since_unix_epoch as f64) + (nsecs as f64) / 1_000_000_000.0
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

/// Implementation of the `CBORTagged` trait for `Date`.
///
/// This implementation specifies that `Date` values are tagged with CBOR tag 1,
/// which is the standard CBOR tag for date/time values represented as seconds
/// since the Unix epoch per RFC 8949.
impl CBORTagged for Date {
    /// Returns the CBOR tags associated with the `Date` type.
    ///
    /// For dates, this is always tag 1, which is the standard CBOR tag for
    /// date/time values represented as seconds since the Unix epoch.
    ///
    /// # Returns
    ///
    /// A vector containing tag 1
    fn cbor_tags() -> Vec<Tag> {
        tags_for_values(&[TAG_DATE])
    }
}

/// Implementation of the `CBORTaggedEncodable` trait for `Date`.
///
/// This implementation converts a `Date` to an untagged CBOR value
/// representing the number of seconds since the Unix epoch.
impl CBORTaggedEncodable for Date {
    /// Converts this `Date` to an untagged CBOR value.
    ///
    /// The date is converted to a numeric value representing the number of
    /// seconds since the Unix epoch. This value may be an integer or a floating-point
    /// number, depending on whether the date has fractional seconds.
    ///
    /// # Returns
    ///
    /// A CBOR value representing the timestamp
    fn untagged_cbor(&self) -> CBOR {
        self.timestamp().into()
    }
}

/// Implementation of the `CBORTaggedDecodable` trait for `Date`.
///
/// This implementation creates a `Date` from an untagged CBOR value
/// representing seconds since the Unix epoch.
impl CBORTaggedDecodable for Date {
    /// Creates a `Date` from an untagged CBOR value.
    ///
    /// The CBOR value must be a numeric value (integer or floating-point) representing
    /// the number of seconds since the Unix epoch.
    ///
    /// # Arguments
    ///
    /// * `cbor` - The untagged CBOR value
    ///
    /// # Returns
    ///
    /// * `Ok(Date)` - A new `Date` instance if decoding succeeds
    /// * `Err` - If the CBOR value is not a valid timestamp
    fn from_untagged_cbor(cbor: CBOR) -> Result<Self> {
        let n = cbor.clone().try_into()?;
        Ok(Date::from_timestamp(n))
    }
}

/// Implementation of the `Display` trait for `Date`.
///
/// This implementation provides a string representation of a `Date` in ISO-8601 format.
/// For dates with time exactly at midnight (00:00:00), only the date part is shown.
/// For other times, a full date-time string is shown.
impl fmt::Display for Date {
    /// Formats the `Date` as a string in ISO-8601 format.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    /// use dcbor::Date;
    ///
    /// // A date at midnight will display as just the date
    /// let date = Date::from_ymd(2023, 2, 8);
    /// assert_eq!(date.to_string(), "2023-02-08");
    ///
    /// // A date with time will display as date and time
    /// let date = Date::from_ymd_hms(2023, 2, 8, 15, 30, 45);
    /// assert_eq!(date.to_string(), "2023-02-08T15:30:45Z");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dt = self.datetime();
        if dt.hour() == 0 && dt.minute() == 0 && dt.second() == 0 {
            f.write_str(dt.date_naive().to_string().as_str())
        } else {
            f.write_str(dt.to_rfc3339_opts(SecondsFormat::Secs, true).as_str())
        }
    }
}
