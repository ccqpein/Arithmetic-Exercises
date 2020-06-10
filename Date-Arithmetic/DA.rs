use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
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

    // fn leap_year_months() -> [i8; 12] {
    //     [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    // }

    // fn normal_year_months() -> [i8; 12] {
    //     [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    // }
}

impl Add<i32> for Year {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        Self::new(self.num + other)
    }
}
impl Sub<i32> for Year {
    type Output = Self;
    fn sub(self, other: i32) -> Self {
        Self::new(self.num - other)
    }
}

#[derive(Debug, Clone, Copy)]
struct Month {
    num: i32,
}

impl Month {
    fn new_with_year(mut m: i32) -> (i32, Self) {
        let mut count = 0;
        while m > 12 {
            count += 1;
            m -= 12
        }
        (count, Month { num: m })
    }

    fn new(m: i32) -> Self {
        Month { num: m }
    }

    fn days_with_year(&self, year: Year) -> Result<u8, String> {
        if self.num < 1 || self.num > 12 {
            return Err("Wrong".to_string());
        }

        match self.num {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
            4 | 6 | 9 | 11 => Ok(30),
            2 => {
                if year.leap {
                    Ok(29)
                } else {
                    Ok(28)
                }
            }
            _ => Err("Wrong month".to_string()),
        }
    }

    // fn days(&self) -> u8 {
    //     match self.num {
    //         1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    //         4 | 6 | 9 | 11 => 30,
    //         2 => 28,
    //         _ => 0,
    //     }
    // }
}

impl Add<i32> for Month {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        let temp = (self.num + other) % 12;
        if temp != 0 {
            Self::new(temp)
        } else {
            Self::new(12)
        }
    }
}
impl Sub<i32> for Month {
    type Output = Self;
    fn sub(self, other: i32) -> Self {
        let mut temp = self.num - other;
        while temp < 1 {
            temp += 12
        }
        Self::new(temp)
    }
}

#[derive(Debug)]
struct Days(i32);

impl Days {
    fn new(d: i32) -> Self {
        Days(d)
    }
}

#[derive(Debug)]
struct Date {
    Y: Year,
    M: Month,
    D: Days,
}

impl Date {
    fn new(y: i32, m: i32, d: i32) -> Self {
        let (y_adjust, real_m) = Month::new_with_year(m);
        let mut init = Date {
            Y: Year::new(y) + y_adjust,
            M: real_m,
            D: Days::new(d),
        };

        //Self::days_adjust(&mut init); // same as below
        init.days_adjust();
        init
    }

    fn days_adjust(&mut self) {
        if self.D.0 <= 0 {
            // negative days
            while self.D.0 <= 0 {
                self.M = self.M - 1;
                // cross year
                if self.M.num == 12 {
                    self.Y = self.Y - 1;
                }
                let dd = self.M.days_with_year(self.Y).unwrap();

                self.D.0 += dd as i32;
            }
        } else {
            loop {
                let dd = self.M.days_with_year(self.Y).unwrap();
                if self.D.0 > dd as i32 {
                    self.M = self.M + 1;
                    if self.M.num == 1 {
                        self.Y = self.Y + 1;
                    }
                    self.D.0 -= dd as i32
                } else {
                    break;
                }
            }
        }
    }
}

fn date_calculation(mut d: Date, days: i32) -> Date {
    d.D.0 += days;
    d.days_adjust();
    d
}

fn tests_set_of_month() {
    let case0 = Month::new(2);
    assert!((case0 + 1).num == 3);

    let case1 = Month::new(12);
    assert!((case1 + 1).num == 1);

    assert!((case1 + 2).days_with_year(Year::new(2000)) == Ok(29));

    let case2 = Month::new_with_year(14);
    assert!(case2.0 == 1);
    assert!(case2.1.num == 2);

    let case3 = Month::new_with_year(24);
    assert!(case3.0 == 1);
    assert!(case3.1.num == 12);

    let case4 = Month::new(2);
    assert!((case4 - 14).num == 12);
}

fn tests_set_of_date() {
    let case0 = Date::new(2000, 3, 0);
    assert!(case0.M.num == 2);
    assert!(case0.D.0 == 29);

    let case1 = Date::new(2000, 1, -1);
    assert!(case1.Y.num == 1999);
    assert!(case1.M.num == 12);
    assert!(case1.D.0 == 30);

    let case2 = Date::new(2000, 1, 32);
    assert!(case2.M.num == 2);
    assert!(case2.D.0 == 1);

    let case3 = Date::new(1999, 12, 31 + 365);
    assert!(case3.M.num == 12);
    assert!(case3.D.0 == 30);
}

fn main() {
    tests_set_of_month();
    tests_set_of_date();

    let case0 = Date::new(2015, 2, 15);
    let result0 = date_calculation(case0, 7);
    assert!(result0.Y.num == 2015);
    assert!(result0.M.num == 2);
    assert!(result0.D.0 == 22);

    let case0 = Date::new(2015, 2, 15);
    let result0 = date_calculation(case0, -7);
    assert!(result0.Y.num == 2015);
    assert!(result0.M.num == 2);
    assert!(result0.D.0 == 8);

    let case0 = Date::new(2015, 2, 15);
    let result0 = date_calculation(case0, -16);
    assert!(result0.Y.num == 2015);
    assert!(result0.M.num == 1);
    assert!(result0.D.0 == 30);
}
