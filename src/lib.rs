//! A rust library for parsing date/time strings from various formats
//! and normalising to a standard fixed offset format (rfc3339).
//! Parsed date will be returned `DateTime<FixedOffset>`
//!

use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

#[cfg(test)]
mod tests;

type Error = String;

/// DateTimeFixedOffset returns a str containing date time to a
/// standard datetime fixed offset RFC 3339 format.
///
/// ## Example usage:
/// ```
/// use datetime_parse::DateTimeFixedOffset;
///
/// let date_str = "Mon, 6 Jul 1970 15:30:00 PDT";
/// let result = date_str.parse::<DateTimeFixedOffset>();
/// assert!(result.is_ok());
/// match result {
///     Ok(parsed) => println!("{} => {:?}", date_str, parsed.0),
///     Err(e) => println!("Error: {}", e)
/// }
/// ```
#[derive(Debug)]
pub struct DateTimeFixedOffset(pub DateTime<FixedOffset>);

impl std::str::FromStr for DateTimeFixedOffset {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        parse_from(s).map(DateTimeFixedOffset)
    }
}

/// parse_from interprets the input date/time slice and returns a normalised parsed date/time
/// as DateTime<FixedOffset> or will return an Error
fn parse_from(date_time: &str) -> Result<DateTime<FixedOffset>, Error> {
    let date_time = standardize_date(date_time);
    DateTime::parse_from_str(&date_time, "%+")
        .or_else(|_| DateTime::parse_from_rfc3339(&date_time))
        .or_else(|_| DateTime::parse_from_rfc2822(&date_time))
        .or_else(|_| DateTime::parse_from_str(&date_time, "%Y-%m-%dT%T%.f%z"))
        .or_else(|_| DateTime::parse_from_str(&date_time, "%Y-%m-%d %T%#z"))
        .or_else(|_| DateTime::parse_from_str(&date_time, "%Y-%m-%d %T.%f%#z"))
        .or_else(|_| DateTime::parse_from_str(&date_time, "%Y-%m-%d %T.%f%#z"))
        .or_else(|_| from_datetime_without_tz(&date_time))
        .or_else(|_| from_date_without_tz(&date_time))
        .or_else(|_| from_time_without_tz(&date_time))
        .or_else(|_| try_yms_hms_tz(&date_time))
}

/// Convert a `datetime` string, that which mostly does not have a timezone info
/// to Datetime fixed offset with local timezone
fn from_datetime_without_tz(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    Local
        .datetime_from_str(s, "%Y-%m-%dT%T.%f")
        .or_else(|_|Local.datetime_from_str(s, "%b %d %Y %T"))
        .or_else(|_|Local.datetime_from_str(s, "%b %d, %Y %T"))
        .or_else(|_|Local.datetime_from_str(s, "%B %d %Y %T"))
        .or_else(|_|Local.datetime_from_str(s, "%B %d, %Y %T"))
        .or_else(|_|Local.datetime_from_str(s, "%Y-%m-%d %T"))
        .or_else(|_|Local.datetime_from_str(s, "%%Y-%m-%d %T.%f"))
        .map(|x| x.with_timezone(x.offset()))
        .map_err(|e| e.to_string())
}

/// Convert just `date` string without time or timezone information
/// to Datetime fixed offset with local timezone
fn from_date_without_tz(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(s, "%m-%d-%y"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%D"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%F"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%v"))
        .map(|x| x.and_hms(0, 0, 0))
        .map(|x| Local.from_local_datetime(&x))
        .map_err(|e| e.to_string())
        .map(|x| x.unwrap().with_timezone(x.unwrap().offset()))
}

/// Convert just `time` string without date or timezone information
/// to Datetime fixed offset with local timezone & current date
fn from_time_without_tz(s: &str) -> Result<DateTime<FixedOffset>, String> {
    NaiveTime::parse_from_str(s, "%T")
        .or_else(|_| NaiveTime::parse_from_str(s, "%I:%M%P"))
        .or_else(|_| NaiveTime::parse_from_str(s, "%I:%M %P"))
        .map(|x| Local::now().date().and_time(x).unwrap().naive_local())
        .map(|x| DateTime::from_utc(x, FixedOffset::east(0)))
        .map_err(|e| e.to_string())
}

/// Try to parse the following types of dates
/// 1970-12-25 16:16:16 PST
/// 1970-12-25 16:16 PST
fn try_yms_hms_tz(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    if let Some((tz, dt)) = is_tz_alpha(s) {
        // assume we found a timezone
        let tz = match tz.to_lowercase().as_str() {
            "ut" | "utc" => "gmt".to_uppercase(),
            _ => tz.to_uppercase(),
        };
        to_rfc2822(dt, &tz)
    } else {
        Err("yms_hms_tz failed".to_string())
    }
}

/// Checks if the last characters are alphabet and assumes it to be TimeZone
/// and returns the tuple of (timezone_part, date_part)
fn is_tz_alpha(s: &str) -> Option<(&str, &str)> {
    let mut dtz = s.trim().rsplitn(2, ' ');
    let tz = dtz.next().unwrap_or_default();
    let dt = dtz.next().unwrap_or_default();
    if tz.chars().all(char::is_alphabetic) {
        Some((tz, dt))
    } else {
        None
    }
}

/// Convert the given date/time and timezone information into RFC 2822 format
fn to_rfc2822(s: &str, tz: &str) -> Result<DateTime<FixedOffset>, Error> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M"))
        .and_then(|x| {
            DateTime::parse_from_rfc2822(
                (x.format("%a, %d %b %Y %H:%M:%S").to_string() + " " + tz).as_str(),
            )
        })
        .map_err(|e| e.to_string())
}

/// converts date/time string from having '.' or '/' to '-'
/// eg: 12/13/2000 to 12-13-2000 or 12/13/2000 12:12:12.14 to 12-13-2000 12:12:12.14
fn standardize_date(s: &str) -> String {
    if s.len() < 8 {
        s.to_string()
    } else {
        s.chars()
            .into_iter()
            .take(8)
            .map(|mut x| {
                if x.eq(&'.') || x.eq(&'/') {
                    x = '-'
                };
                x
            })
            .collect::<String>()
            + &s[8..]
    }
}
