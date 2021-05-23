/// tests
use crate::DateTimeFixedOffset;

#[test]
fn test_dotted_date() {
    let date = "1970.12.31";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test.unwrap().0.to_rfc3339().starts_with("1970-12-31T00:00:00"));
}

#[test]
fn test_slash_date() {
    let date = "1970/12/31";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test.unwrap().0.to_rfc3339().starts_with("1970-12-31T00:00:00"));
}
