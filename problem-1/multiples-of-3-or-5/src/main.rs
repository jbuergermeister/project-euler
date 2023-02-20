fn main() {
    // create array of numbers of size max=1000
    // iterate over values
        // check if multiple, set to value if no
    // sum values in array
    const MAX_VALUE: usize = 1000;
    let numbers: Vec<usize> = (1..MAX_VALUE).filter(|x| x % 3 == 0 || x % 5 == 0).collect() ;
    println!("{numbers:?}");
    let result: usize = numbers.into_iter().sum();
    println!("{result:?}");
}
