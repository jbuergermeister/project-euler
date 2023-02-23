/*
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
*/

fn main() {
    const MILLION: usize = 1_000_000;
    let mut result = 0;
    let mut max_length = 0;
    for num in 1..MILLION {
        let length = collatz_step(num, 0);
        if length > max_length {
            max_length = length;
            result = num;
            println!("starting at {result}: sequence length {max_length}");
        }
    }
    println!("{result}");
}

fn collatz_step(num: usize, length: usize) -> usize {
    if num == 1 {
        length + 1
    } else if num % 2 == 0 {
        collatz_step(num / 2, length + 1)
    } else {
        collatz_step(3 * num + 1, length + 1)
    }
}