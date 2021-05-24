//! A rust library for parsing date/time strings from various formats
//! and normalising to a standard fixed offset format (rfc3339).
//! Parsed date will be returned `DateTime<FixedOffset>`
//!

use chrono::{
    DateTime, Datelike, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, ParseError,
    TimeZone,
};

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
        .or_else(|_| from_datetime_with_tz(&date_time))
        .or_else(|_| from_datetime_without_tz(&date_time))
        .or_else(|_| from_date_without_tz(&date_time))
        .or_else(|_| from_time_without_tz(&date_time))
        .or_else(|_| from_time_with_tz(&date_time))
        .or_else(|_| try_yms_hms_tz(&date_time))
        .or_else(|_| try_dmony_hms_tz(&date_time))
        .or_else(|_| try_others(&date_time))
}

/// Convert a `datetime` string to `DateTime<FixedOffset>`
fn from_datetime_with_tz(s: &str) -> Result<DateTime<FixedOffset>, ParseError> {
    DateTime::parse_from_rfc3339(s)
        .or_else(|_| DateTime::parse_from_rfc2822(s))
        .or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%dT%T%.f%z"))
        .or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%d %T%#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%d %T.%f%#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%B %d, %Y; %T %#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%B %d, %Y; %T.%f%#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%B %d %Y %T %#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%B %d %Y %T.%f%#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%B, %d %Y %T %#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%B, %d %Y %T.%f%#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%A, %d %B %Y %T.%f%#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%A %d %B %Y %T.%f%#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%A, %d %B %Y %T %#z"))
        .or_else(|_| DateTime::parse_from_str(s, "%A %d %B %Y %T %#z"))
}

/// Convert a `datetime` string, that which mostly does not have a timezone info
/// to Datetime fixed offset with local timezone
fn from_datetime_without_tz(s: &str) -> Result<DateTime<FixedOffset>, ParseError> {
    Local
        .datetime_from_str(s, "%Y-%m-%dT%T")
        .or_else(|_| Local.datetime_from_str(s, "%c"))
        .or_else(|_| Local.datetime_from_str(s, "%Y-%m-%dT%T.%f"))
        .or_else(|_| Local.datetime_from_str(s, "%Y-%m-%d %T"))
        .or_else(|_| Local.datetime_from_str(s, "%Y-%m-%d %T.%f"))
        .or_else(|_| Local.datetime_from_str(s, "%B %d %Y %T"))
        .or_else(|_| Local.datetime_from_str(s, "%B %d %Y %T.%f"))
        .or_else(|_| Local.datetime_from_str(s, "%B %d, %Y %T"))
        .or_else(|_| Local.datetime_from_str(s, "%B %d, %Y %T.%f"))
        .or_else(|_| Local.datetime_from_str(s, "%Y-%m-%d %T"))
        .or_else(|_| Local.datetime_from_str(s, "%Y-%m-%d %T.%f"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %B %Y %T.%f"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %B %Y %T.%f"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %B %Y %T"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %B %Y %T"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %B %Y %I:%M%P"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %B %Y %I:%M %P"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %B %Y %I:%M:%S%P"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %B %Y %I:%M:%S %P"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %B %Y %I:%M%P"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %B %Y %I:%M %P"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %B %Y %I:%M:%S%P"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %B %Y %I:%M:%S %P"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %m %Y %I:%M%P"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %m %Y %I:%M %P"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %m %Y %I:%M:%S%P"))
        .or_else(|_| Local.datetime_from_str(s, "%A %d %m %Y %I:%M:%S %P"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %m %Y %I:%M%P"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %m %Y %I:%M %P"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %m %Y %I:%M:%S%P"))
        .or_else(|_| Local.datetime_from_str(s, "%A, %d %m %Y %I:%M:%S %P"))
        .or_else(|_| Local.datetime_from_str(s, "%d %B %Y %I:%M%P"))
        .or_else(|_| Local.datetime_from_str(s, "%d %B %Y %I:%M %P"))
        .or_else(|_| Local.datetime_from_str(s, "%d %B %Y %I:%M:%S%P"))
        .or_else(|_| Local.datetime_from_str(s, "%d %B %Y %I:%M:%S %P"))
        .or_else(|_| Local.datetime_from_str(s, "%d %m %Y %I:%M%P"))
        .or_else(|_| Local.datetime_from_str(s, "%d %m %Y %I:%M %P"))
        .or_else(|_| Local.datetime_from_str(s, "%d %m %Y %I:%M:%S%P"))
        .or_else(|_| Local.datetime_from_str(s, "%d %m %Y %I:%M:%S %P"))
        .map(|x| x.with_timezone(x.offset()))
}

/// Convert just `date` string without time or timezone information
/// to Datetime fixed offset with local timezone
fn from_date_without_tz(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(s, "%m-%d-%y"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%D"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%F"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%v"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%B %d %Y"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%B, %d %Y"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%d %B %Y"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%d %B, %Y"))
        .map(|x| x.and_hms(0, 0, 0))
        .map(|x| Local.from_local_datetime(&x))
        .map_err(|e| e.to_string())
        .map(|x| x.unwrap().with_timezone(x.unwrap().offset()))
}

/// Convert just `time` string without date or timezone information
/// to Datetime fixed offset with local timezone & current date
fn from_time_without_tz(s: &str) -> Result<DateTime<FixedOffset>, ParseError> {
    NaiveTime::parse_from_str(s, "%T")
        .or_else(|_| NaiveTime::parse_from_str(s, "%I:%M%P"))
        .or_else(|_| NaiveTime::parse_from_str(s, "%I:%M %P"))
        .map(|x| Local::now().date().and_time(x).unwrap())
        .map(|x| x.with_timezone(x.offset()))
}

/// Convert just `time` string without date but timezone information
/// to Datetime fixed offset with local timezone & current date
fn from_time_with_tz(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    if let Some((dt, tz)) = is_tz_alpha(s) {
        let date = format!("{} {}", Local::today().format("%Y-%m-%d").to_string(), dt);
        to_rfc2822(&date, tz)
    } else {
        Err("custom parsing failed".to_string())
    }
}

/// Try to parse the following types of dates
/// 1970-12-25 16:16:16 PST
/// 1970-12-25 16:16 PST
fn try_yms_hms_tz(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    if let Some((dt, tz)) = is_tz_alpha(s) {
        to_rfc2822(dt, &tz)
    } else {
        Err("custom parsing failed".to_string())
    }
}

/// Try to parse the following types of dates
/// 1 Jan 1970 22:00:00 PDT
/// 1 Jan, 1970 22:00:00.000 PDT
/// 1 Jan, 1970; 22:00:00 PDT
fn try_dmony_hms_tz(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    if let Some((dt, tz)) = is_tz_alpha(s) {
        to_rfc2822(dt, &tz)
    } else {
        Err("custom parsing failed".to_string())
    }
}

/// Try to parse the following types of dates
/// Feb 12 12:12:12 or Feb 12, 12:12
/// Feb 12 or 12 Feb
fn try_others(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    let date = s.split_whitespace().collect::<Vec<_>>();
    let year = Local::now().year();
    if date.len().eq(&2) && date[0].replace(',', "").chars().all(char::is_alphabetic) {
        // trying Feb 12
        NaiveDate::parse_from_str(&format!("{} {}", s, year), "%B %d %Y")
            .map(|x| x.and_hms(0, 0, 0))
            .map(|x| Local.from_local_datetime(&x))
            .map_err(|e| e.to_string())
            .map(|x| x.unwrap().with_timezone(x.unwrap().offset()))
    } else if date.len().eq(&2) && date[1].replace(',', "").chars().all(char::is_alphabetic) {
        // trying 12 Feb
        NaiveDate::parse_from_str(&format!("{} {}", s, year), "%d %B %Y")
            .map(|x| x.and_hms(0, 0, 0))
            .map(|x| Local.from_local_datetime(&x))
            .map_err(|e| e.to_string())
            .map(|x| x.unwrap().with_timezone(x.unwrap().offset()))
    } else if date.len().eq(&3) && date[0].replace(',', "").chars().all(char::is_alphabetic) {
        // trying Feb 12 14:00:01 or Feb 12, 14:00:01 or Feb 12 14:00
        Local
            .datetime_from_str(
                &format!("{} {} {} {}", date[0], date[1], year, date[2]).replace(',', ""),
                "%B %d %Y %H:%M",
            )
            .or_else(|_| {
                Local.datetime_from_str(
                    &format!("{} {} {} {}", date[0], date[1], year, date[2]).replace(',', ""),
                    "%B %d %Y %T",
                )
            })
            .or_else(|_| {
                Local.datetime_from_str(
                    &format!("{} {} {} {}", date[0], date[1], year, date[2]).replace(',', ""),
                    "%B %d %Y %I:%M%P",
                )
            })
            .map(|x| x.with_timezone(x.offset()))
            .map_err(|e| e.to_string())
    } else if date.len().eq(&3) && date[1].replace(',', "").chars().all(char::is_alphabetic) {
        // trying 12 Feb 14:00:01 or 12 Feb, 14:00:01 or 12 Feb 14:00
        Local
            .datetime_from_str(
                &format!("{} {} {} {}", date[0], date[1], year, date[2]).replace(',', ""),
                "%d %B %Y %H:%M",
            )
            .or_else(|_| {
                Local.datetime_from_str(
                    &format!("{} {} {} {}", date[0], date[1], year, date[2]).replace(',', ""),
                    "%d %B %Y %T",
                )
            })
            .or_else(|_| {
                Local.datetime_from_str(
                    &format!("{} {} {} {}", date[0], date[1], year, date[2]).replace(',', ""),
                    "%d %B %Y %I:%M%P",
                )
            })
            .map(|x| x.with_timezone(x.offset()))
            .map_err(|e| e.to_string())
    } else if date.len().eq(&4) && date[0].replace(',', "").chars().all(char::is_alphabetic) {
        // trying Feb 12 3:33 pm
        Local
            .datetime_from_str(
                &format!("{} {} {} {} {}", date[0], date[1], year, date[2], date[3])
                    .replace(',', ""),
                "%B %d %Y %I:%M %P",
            )
            .map(|x| x.with_timezone(x.offset()))
            .map_err(|e| e.to_string())
    } else if date.len().eq(&4) && date[1].replace(',', "").chars().all(char::is_alphabetic) {
        // trying 12 Feb 3:33 pm
        Local
            .datetime_from_str(
                &format!("{} {} {} {} {}", date[0], date[1], year, date[2], date[3])
                    .replace(',', ""),
                "%d %B %Y %I:%M %P",
            )
            .map(|x| x.with_timezone(x.offset()))
            .map_err(|e| e.to_string())
    } else {
        Err("failed brute force parsing".to_string())
    }
}

/// Checks if the last characters are alphabet and assumes it to be TimeZone
/// and returns the tuple of (date_part, timezone_part)
fn is_tz_alpha(s: &str) -> Option<(&str, &str)> {
    let mut dtz = s.trim().rsplitn(2, ' ');
    let tz = dtz.next().unwrap_or_default();
    let dt = dtz.next().unwrap_or_default();
    if tz.chars().all(char::is_alphabetic) {
        Some((dt, tz))
    } else {
        None
    }
}

/// Convert the given date/time and timezone information into RFC 2822 format
fn to_rfc2822(s: &str, tz: &str) -> Result<DateTime<FixedOffset>, Error> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %I:%M%P"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %I:%M %P"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%d %B, %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%d %B, %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%d %B %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%d %B %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d, %Y %H:%M"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d %Y %H:%M"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d, %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d, %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d %Y; %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d, %Y; %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d, %Y; %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%B %d %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A, %B %d %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A, %B %d %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A, %d %B %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A, %d %B %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A %B %d %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A %B %d %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A %d %B %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A %d %B %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A, %d %m %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A, %d %m %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A, %d %m %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A, %d %m %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A %d %m %Y %T.%f"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A %d %m %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A %d %m %Y %T"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%A %d %m %Y %T.%f"))
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
    .replace(" UTC", " GMT")
    .replace(" UT", " GMT")
}
