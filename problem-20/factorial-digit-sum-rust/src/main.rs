/*
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
*/

fn main() {
    let result: usize = factorial(200).into_iter().sum();
    // calculate number of digits using sterlings formula
    // represent number as vector of digits
    println!("{result:?}");
}

fn factorial(num: usize) -> Vec<usize> {
    let size = num*num; // Estimate using log of Stirling's formula: 1/2 log(2pi n) + n log(n/e)
    let mut vec = vec![0;size];
    vec[0] += 1;
    if num < 2 {
        return vec;
    } else {
        for i in 2..=num {
            vec = num_times_vec(i, vec);
        }
        return vec;
    }
}
// implement multiplication of a number represented as vector of its digits by an ordinarily represented number
fn num_times_vec(num: usize, vec: Vec<usize>) -> Vec<usize> {
    let mut vec = vec;
    vec = vec.into_iter().map(|x| num * x).collect();
    for i in 0..vec.len() {
        let mut reduced = vec[i];
        vec[i] = reduced % 10;
        reduced /= 10;
        let mut count = 1;
        while reduced > 0 {
            vec[i+count] += reduced % 10;
            reduced /= 10;
            count += 1;
        }
    }
    vec
}
