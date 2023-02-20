/*
Problem 10

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

fn main() {
    const UPPER_BOUND: usize = 2_000_000;
    let mut primes: Vec<usize> = vec![2];
    for num in (3..UPPER_BOUND).step_by(2) {
        if prime_check(num as usize, primes.clone()) {
            primes.push(num);
            println!("{num}");
        }
    }
    let sum: usize = primes.into_iter().sum();
    println!("{sum}");
}

fn prime_check(num: usize, primes: Vec<usize>) -> bool {
    for prime in primes.iter() {
        if num % *prime == 0 {
            return false;
        }
    }
    return true;
}
