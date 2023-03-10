/*
Problem 9

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a2 + b2 = c2

For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

fn main() {
    for a in 1..1000 {
        for b in 1..a {
            let c = 1_000 - a - b;
            let pythagorean = a*a + b*b;
            //println!("{pythagorean} {c}");
            if pythagorean == c*c {
                let result = a*b*c;
                println!("{result}");
            }
        }
    }
}
