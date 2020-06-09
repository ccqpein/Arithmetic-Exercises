use std::ops::Add;

#[derive(Debug)]
struct Year {
    num: i32,
    leap: bool,
}

impl Year {
    fn new(y: i32) -> Self {
        Year {
            num: y,
            leap: Self::judge_leap(y),
        }
    }

    fn judge_leap(y: i32) -> bool {
        if (y % 4) != 0 {
            return false;
        } else if (y % 100) != 0 {
            return true;
        } else if (y % 400) != 0 {
            return false;
        } else {
            true
        }
    }
}

#[derive(Debug)]
struct Month(u8);

impl Month {
    fn new(m: u8) -> Result<Self, String> {
        if m < 1 || m > 12 {
            return Err(String::from("Are you kidding me"));
        }

        Ok(Month(m))
    }
}

#[derive(Debug)]
struct Days(i32);

impl Days {
    fn new(d:i32) -> 
}

#[derive(Debug)]
struct Date {
    Y: Year,
    M: Month,
    D: Days,
}

impl Date {
    fn new(y: i32, m: u8, d: i32) -> Result<Self, String> {
        Ok(Date {
            Y: Year::new(y),
            M: Month::new(m)?,
            D: Days::new(d),
        })
    }
}

impl Add<u32> for Date {
    type Output = Self;
    fn add(self, other: u32) -> Self {
        Date::new()
    }
}

fn date_calculation(d: Date, days: i32) -> Date {
    Date::new()
}

fn main() {}
