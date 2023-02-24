/*
If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
*/

fn main() {
    let mut count = 0;
    for num in 1..=1_000 {
        if (num % 100 > 20) | (num % 100 < 10) {
            match num % 10 {
                0 => println!("{num}"),
                1 => {count += 3;}, // "one"
                2 => {count += 3;}, // "two"
                3 => {count += 5;}, // "three"
                4 => {count += 4;}, // "four"
                5 => {count += 4;}, // "five"
                6 => {count += 3;}, // "six"
                7 => {count += 5;}, // "seven"
                8 => {count += 5;}, // "eight"
                9 => {count += 4;}, // "nine"
                _ => unreachable!()
            }
        } else if num % 100 == 10 {
            count += 3; // "ten"
        } else if num % 100 == 20 {
            count += 6; // "twenty"
        } else {
            match num % 10 {
                1 => {count += 6;} // "eleven"
                2 => {count += 6;} // "twelve"
                3 => {count += 8;} // "thirteen"
                4 => {count += 8;} // "fourteen"
                5 => {count += 7;} // "fifteen"
                6 => {count += 7;} // "sixteen"
                7 => {count += 9;} // "seventeen"
                8 => {count += 8;} // "eighteen"
                9 => {count += 8;} // "nineteen"
                _ => unreachable!()
            }
        }

        if num % 100 > 20 {
            match (num % 100) / 10 {
                2 => {count += 6;}, // "twenty"
                3 => {count += 6;}, // "thirty"
                4 => {count += 5;}, // "forty"
                5 => {count += 5;}, // "fifty"
                6 => {count += 5;}, // "sixty"
                7 => {count += 7;}, // "seventy"
                8 => {count += 6;}, // "eighty"
                9 => {count += 6;}, // "ninety"
                _ => unreachable!()
            }
        }

        if (num >= 100) && (num < 1_000) {
            count += 7; // "hundred"
            if num % 100 != 0 {
                count += 3; // "and"
            }
            match num / 100 {
                1 => {count += 3;}, // "one"
                2 => {count += 3;}, // "two"
                3 => {count += 5;}, // "three"
                4 => {count += 4;}, // "four"
                5 => {count += 4;}, // "five"
                6 => {count += 3;}, // "six"
                7 => {count += 5;}, // "seven"
                8 => {count += 5;}, // "eight"
                9 => {count += 4;}, // "nine"
                _ => unreachable!()
            }
        } else if num == 1_000 {
            count += 11; // "one thousand"
        }
    }

    println!("{count}");
}
