/*
Problem 6

The sum of the squares of the first ten natural numbers is 1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

fn main() {
    let min = 1;
    let max = 100;
    let mut sum_of_squares = 0;
    let mut square_of_sum = 0;
    for i in min..=max {
        square_of_sum += i;
        sum_of_squares += square(i);
    }
    square_of_sum = square(square_of_sum);
    let result = square_of_sum - sum_of_squares;
    println!("{result}");
}

fn square(num: usize) -> usize {
    num * num
}