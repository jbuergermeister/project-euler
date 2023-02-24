/*
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
*/

fn main() {
    let mut num: Vec<usize> = vec![1];
    for _i in 0..1_000 {
        num = multiply_by_two(num);
        num = carry(num);
    //println!("{num:?}");
    }
    let result: usize = num.into_iter().sum();
    println!("{result}");
}

fn multiply_by_two(vec: Vec<usize>) -> Vec<usize> {
    let mut result = vec;
    for i in 0..result.len() {
        result[i] *= 2;
    }
    result
}

fn carry(vec: Vec<usize>) -> Vec<usize> {
    let mut result = vec;
    for i in 0..result.len() {
        if result[i] >= 10 {
            result[i] -= 10;
            if let Some(digit) = result.get_mut(i+1) {
                *digit +=1;
            } else {
                result.push(1);
            }
        }
    }
    result
}