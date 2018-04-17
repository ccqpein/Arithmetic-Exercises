use std::collections::HashMap;

//:= maybe I will finish this trait in future
/*
impl<'a> std::ops::Try for &'a str {
    type Ok = &'a str;
    type Error = Self;

    fn into_result(self) -> Result<&str, Self::Error> {
        Ok(self)
    }

    fn from_ok(v: &str) -> Self {
        Some(v)
    }

    fn from_error(_: NoneError) -> Self {
        None
    }
}*/

fn hours_parse<'a>(input: &'a str, num_table: &'a HashMap<i32, &str>) -> (&'a str, &'a str) {
    let num = input.parse::<i32>().unwrap();

    if num > 23 {
        panic!("hours wrong")
    };

    if num >= 12 {
        return (num_table.get(&(num - 12)).unwrap(), "pm");
    } else {
        return (num_table.get(&num).unwrap(), "am");
    }
}

fn minute_parse<'a>(input: &'a str, num_table: &'a HashMap<i32, &str>) -> String {
    let num = input.parse::<i32>().unwrap();

    if num == 0 {
        String::new()
    } else if num < 10 {
        return format!("oh {}", num_table.get(&num).unwrap());
    } else if num <= 20 {
        return format!("{}", num_table.get(&num).unwrap());
    } else if num < 60 {
        if num < 30 {
            return format!(
                "{} {}",
                num_table.get(&20).unwrap(),
                num_table.get(&(num - 20)).unwrap()
            );
        } else if num == 30 {
            return format!("{}", num_table.get(&30).unwrap());
        } else if num < 40 {
            return format!(
                "{} {}",
                num_table.get(&30).unwrap(),
                num_table.get(&(num - 30)).unwrap()
            );
        } else if num == 40 {
            return format!("{}", num_table.get(&40).unwrap());
        } else if num < 50 {
            return format!(
                "{} {}",
                num_table.get(&40).unwrap(),
                num_table.get(&(num - 40)).unwrap()
            );
        } else if num == 50 {
            return format!("{}", num_table.get(&50).unwrap());
        }
        return format!(
            "{} {}",
            num_table.get(&50).unwrap(),
            num_table.get(&(num - 50)).unwrap()
        );
    } else {
        panic!("minutes wrong");
    }
}

fn main() {
    let num_table: HashMap<i32, &str> = [
        (0, "twelve"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        // below code is for minutes
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "fourty"),
        (50, "fifty"),
    ].iter()
        .cloned()
        .collect();
    let test_case = ["00:00", "01:30", "12:05", "14:01", "20:29", "21:00"];

    for case in test_case.iter() {
        let split_string: Vec<&str> = case.split(":").collect();

        let (hour, noon) = hours_parse(split_string[0], &num_table);
        let minutes = minute_parse(split_string[1], &num_table);

        if minutes == "" {
            println!("it's {} {}", hour, noon);
        } else {
            println!("it's {} {} {}", hour, minutes, noon);
        }
    }

    //println!("{:?}", hours_parse(test0, &num_table));
    //println!("{:?}", minute_parse(test0, &num_table));
}
