/*
You are given the following information, but you may prefer to do some research for yourself.

    1 Jan 1900 was a Monday.
    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
    A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
*/

fn main() {
    let weekdays = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let mut day_number = 1; // 1 Jan 1900 = day 1, 2 Jan 1900 = day 2 etc.
    let mut count = 0;
    loop {
        let weekday = weekdays[day_number % 7];
        let (yr, mo, day) = date(day_number);
        //println!("{yr}-{mo}-{day}: {weekday}");
        if yr > 2000 {
            break;
        }
        if day == 1 && weekday == "Sunday" && day_number > 365 {
            count +=1;
            println!("{yr}-{mo}-{day}: {weekday}");
        }
        day_number += 1;
    }
    let result = count;
    println!("{result:?}");

}

fn date(day_number: usize) -> (usize, usize, usize) {
    let mut num = day_number;
    let mut count = 0;
    let mut condition = {num > 365};
    while condition {
        if count == 0 {
            num -= 365;
            condition = num > 365;
        } else if count % 4 == 0 {
            num -= 366;
            condition = num > 365;
        } else {
            num -= 365;
            if count % 4 == 3 {
                condition = num > 366;
            } else {
                condition = num > 365;
            }
        }
        count += 1;
    }
    let mut leap_day = 0;
    if count % 4 == 0 {
        leap_day += 1;
    }
    let month_ranges: [usize; 12] = [31, 59+leap_day, 90+leap_day, 120+leap_day, 151+leap_day, 181+leap_day, 
                                    212+leap_day, 243+leap_day, 273+leap_day, 304+leap_day, 334+leap_day, 365+leap_day,];
    let mut month = 1;
    if num > month_ranges[10] {
        month = 12;
        num -= month_ranges[10];
    } else if num > month_ranges[9] {
        month = 11;
        num -= month_ranges[9]; 
    } else if num > month_ranges[8] {
        month = 10;
        num -= month_ranges[8]; 
    } else if num > month_ranges[7] {
        month = 9;
        num -= month_ranges[7]; 
    } else if num > month_ranges[6] {
        month = 8;
        num -= month_ranges[6]; 
    } else if num > month_ranges[5] {
        month = 7;
        num -= month_ranges[5]; 
    } else if num > month_ranges[4] {
        month = 6;
        num -= month_ranges[4]; 
    } else if num > month_ranges[3] {
        month = 5;
        num -= month_ranges[3]; 
    } else if num > month_ranges[2] {
        month = 4;
        num -= month_ranges[2]; 
    } else if num > month_ranges[1] {
        month = 3;
        num -= month_ranges[1]; 
    } else if num > month_ranges[0] {
        month = 2;
        num -= month_ranges[0]; 
    }
    (count + 1900, month, num)
}