use datetime_parse::DateTimeFixedOffset;

fn main() {
    let dates = include_str!("dates.txt").trim();
    for date in dates.lines() {
        match date.parse::<DateTimeFixedOffset>() {
            Ok(parsed) => println!("{:_<35}: {:?}", date, parsed.0),
            Err(e) => println!("{:_<35}: error: {}", date, e),
        }
    }
}
