/// tests
use crate::DateTimeFixedOffset;

#[test]
fn test_dotted_date() {
    let date = "1970.12.31";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("1970-12-31T00:00:00"));
}

#[test]
fn test_slash_date() {
    let date = "1970/12/31";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("1970-12-31T00:00:00"));
}

#[test]
fn test_epoch_seconds() {
    let date = "1672903639";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-01-05T07:27:19"));
}

#[test]
fn test_epoch_milliseconds() {
    let date = "1672903639123";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-01-05T07:27:19.123"));
}

#[test]
fn test_epoch_microseconds() {
    let date = "1672903639123123";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-01-05T07:27:19.123123"));
}

#[test]
fn test_epoch_nanoseconds() {
    let date = "1672903639123123123";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-01-05T07:27:19.123123123"));
}
