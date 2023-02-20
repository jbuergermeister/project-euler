/*
Problem 4

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn main() {
    let mut palindromes = vec![];
    for i in 100..=999 {
        for j in 100..=i {
            let num = i*j;
            if palindrome_check(num) {
                palindromes.push(num);
            }
        }
    }

    let result = palindromes.iter().max().unwrap();
    println!("{result:?}");
}

fn palindrome_check(number: usize) -> bool {
    let number_string = number.to_string();
    number_string.chars().collect::<Vec<_>>() == number_string.chars().rev().collect::<Vec<_>>()
}