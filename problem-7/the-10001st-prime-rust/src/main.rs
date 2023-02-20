fn main() {
    let mut counter = 0;
    let mut num = 1;
    while counter < 10001 {
        num += 1;
        if prime_check(num) {
            counter += 1;
        }
    }
    println!("{num}");
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