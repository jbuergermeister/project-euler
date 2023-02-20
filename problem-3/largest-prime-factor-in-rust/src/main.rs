/*
Problem 3

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

fn main() {
    let empty_factors: Vec<usize> = vec![];
    let factors = factorize(600851475143, empty_factors);
    let result = factors.iter().max().unwrap();
    println!("{result}");
}

fn factorize(value: usize, vec: Vec<usize>) -> Vec<usize> {
    if value > 1 {
        let mut factors = vec;
        let factor = divides(value, 2);
        factors.push(factor);
        let new_value = value / factor;
        factorize(new_value, factors)
    } else {
        vec
    }
}

fn divides(value: usize, divisor: usize) -> usize {
    if value % divisor == 0 {
        divisor
    } else {
        divides(value, divisor + 1)
    }
}