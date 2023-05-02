use chrono::{prelude::*, Duration};

fn main() {
    println!("{:?}", get_system_expiration_date());
}

fn get_system_expiration_date() -> Option<DateTime<Local>> {
    let time = unsafe { *(0x7ffe02c8 as *const u64) };

    if time == 0 {
        None
    } else {
        let epoch = NaiveDateTime::new(
            NaiveDate::from_yo_opt(1601, 1).unwrap(),
            NaiveTime::default(),
        );
        let offset = Duration::milliseconds((time / 10000).try_into().unwrap());

        Some(Local.from_utc_datetime(&(epoch + offset)))
    }
}
