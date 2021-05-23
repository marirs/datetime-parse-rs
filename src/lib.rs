//! A rust library for parsing date/time strings in many used formats
//! and normalising to a standard fixed offset format.
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
        .or_else(|_| {
            Local
                .datetime_from_str(&date_time, "%Y-%m-%dT%T.%f")
                .or_else(|_| Local.datetime_from_str(&date_time, "%Y-%m-%dT%T"))
                .map(|x| x.with_timezone(x.offset()))
        })
        .or_else(|_| {
            Local
                .datetime_from_str(&date_time, "%b %d %Y %T")
                .or_else(|_| Local.datetime_from_str(&date_time, "%b %d, %Y %T"))
                .or_else(|_| Local.datetime_from_str(&date_time, "%B %d %Y %T"))
                .or_else(|_| Local.datetime_from_str(&date_time, "%B %d, %Y %T"))
                .map(|x| x.with_timezone(x.offset()))
        })
        .or_else(|_| {
            Local
                .datetime_from_str(&date_time, "%Y-%m-%d %T")
                .map(|x| x.with_timezone(x.offset()))
        })
        .or_else(|_| {
            Local
                .datetime_from_str(&date_time, "%Y-%m-%d %T.%f")
                .map(|x| x.with_timezone(x.offset()))
        })
        .or_else(|_| {
            NaiveDate::parse_from_str(&date_time, "%Y-%m-%d")
                .or_else(|_| NaiveDate::parse_from_str(&date_time, "%m-%d-%y"))
                .or_else(|_| NaiveDate::parse_from_str(&date_time, "%D"))
                .or_else(|_| NaiveDate::parse_from_str(&date_time, "%F"))
                .or_else(|_| NaiveDate::parse_from_str(&date_time, "%v"))
                .map(|x| x.and_hms(0, 0, 0))
                .map(|x| Local.from_local_datetime(&x))
                .map_err(|e| e.to_string())
                .map(|x| x.unwrap().with_timezone(x.unwrap().offset()))
        })
        .or_else(|_| {
            NaiveTime::parse_from_str(&date_time, "%T")
                .or_else(|_| NaiveTime::parse_from_str(&date_time, "%I:%M%P"))
                .or_else(|_| NaiveTime::parse_from_str(&date_time, "%I:%M %P"))
                .map(|x| Local::now().date().and_time(x).unwrap().naive_local())
                .map(|x| DateTime::from_utc(x, FixedOffset::east(0)))
                .map_err(|e| e.to_string())
        })
        .or_else(|_| try_yms_hms_tz(&date_time))
}

/// Try to parse the following types of dates
/// 1970-12-25 16:16:16 PST
/// 1970-12-25 16:16 PST
fn try_yms_hms_tz(date_time: &str) -> Result<DateTime<FixedOffset>, Error> {
    if let Some((tz, dt)) = is_tz_alpha(date_time) {
        // assume we found a timezone
        let tz = match tz.to_lowercase().as_str() {
            "ut" | "utc" => "gmt".to_uppercase(),
            _ => tz.to_uppercase(),
        };
        NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M"))
            .and_then(|x| {
                DateTime::parse_from_rfc2822(
                    (x.format("%a, %d %b %Y %H:%M:%S").to_string() + " " + &tz).as_str(),
                )
            })
            .map_err(|e| e.to_string())
    } else {
        Err("yms_hms_tz failed".to_string())
    }
}

/// Checks if the last characters are alphabet and assumes it to be TimeZone
/// and returns the tuple of (timezone_part, date_part)
fn is_tz_alpha(date_time: &str) -> Option<(&str, &str)> {
    let mut dtz = date_time.trim().rsplitn(2, ' ');
    let tz = dtz.next().unwrap_or_default();
    let dt = dtz.next().unwrap_or_default();
    if tz.chars().all(char::is_alphabetic) {
        Some((tz, dt))
    } else {
        None
    }
}

/// converts date/time string from having '.' or '/' to '-'
/// eg: 12/13/2000 to 12-13-2000 or 12/13/2000 12:12:12.14 to 12-13-2000 12:12:12.14
fn standardize_date(date_time: &str) -> String {
    if date_time.len() < 8 {
        date_time.to_string()
    } else {
        date_time
            .chars()
            .into_iter()
            .take(8)
            .map(|mut x| {
                if x.eq(&'.') || x.eq(&'/') {
                    x = '-'
                };
                x
            })
            .collect::<String>()
            + &date_time[8..]
    }
}
