/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10_000.
*/

fn main() {
    let mut divisor_sums: Vec<usize> = vec![];
    for i in 0..=10_000 {
        let divisor_sum = divisors(i).into_iter().sum();
        //println!("d({i})={divisor_sum}");
        divisor_sums.push(divisor_sum);
    }
    let mut amicables: Vec<usize> = vec![];
    for b in 0..divisor_sums.len() {
        let a = divisor_sums[b];
        if (a != b) & (a < divisor_sums.len()) {
            if divisor_sums[a] == b {
                amicables.push(a);
            }
        }
    }
    println!("{amicables:?}");
    let result: usize = amicables.into_iter().sum();
    println!("{result:?}");
}

fn divisors(num: usize) -> Vec<usize> {
    let mut vec = vec![1];
    let upper_bound = ((num as f64).sqrt()) as usize + 1;
    for i in 2..=upper_bound {
        if num % i == 0 {
            vec.push(i);
            vec.push(num/i);
        }
    }
    vec
}