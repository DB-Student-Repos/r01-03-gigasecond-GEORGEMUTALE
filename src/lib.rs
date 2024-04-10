use time::{PrimitiveDateTime as DateTime, Date, Time, Month};

pub fn after( start: DateTime) -> DateTime{

    let gigasecond_duration = time::Duration::seconds(1_000_000_000);

    let result = start + gigasecond_duration;

    result
}

fn main(){
    let start_date = Date::from_calendar_date(2015, Month::January, 24).expect("Invalid date");

    let start_time = Time::from_hms(22, 00, 00).expect("Invalid time");

    let start = DateTime::new(start_date, start_time);

    let result = after(start);

    println!("One gigasecond after {} is {}", start, result)

}