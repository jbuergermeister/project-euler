/*
Problem 5

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn main() {
    let min = 1;
    let max = 20;
    let mut num = prime_product(min, max);
    let mut loop_check = true;
    while loop_check {
        if is_multiple(num, min, max) {
            loop_check = false;
        } else {
            num += 1;
        }
    }

    let result = num;
    println!("{result:?}");
}

fn is_multiple(num: usize, min: usize, max: usize) -> bool {
    for i in min..=max {
        if num % i != 0 {
            return false;
        }
    }
    return true;
}

fn prime_product(min: usize, max: usize) -> usize {
    let mut result: usize = 1;
    for i in min..=max {
        if prime_check(i) {
            result *= i;
        }
    }
    result
}

fn prime_check(num: usize) -> bool {
    if num == 2 {
        true
    } else if num % 2 ==0 {
        false
    } else {
        for i in 3..(num+1)/2 {
            if num % i ==0 {
                return false;
            }
        }
        return true;
    }
}