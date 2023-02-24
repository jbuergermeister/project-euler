/*
Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, 
there are exactly 6 routes to the bottom right corner.

How many such routes are there through a 20×20 grid?
*/

fn main() {
    // number of steps: 40 -> 40! possibilities
    // fixed endpoint: 20 right steps, 20 down steps
    // path = sequence of right/down choices
    // degeneracy of paths is 20! for right/down respectively
    // result = 40!/(20! * 20!)
    let result: u128 = (21..=40).product::<u128>()/((1..=20).product::<u128>());
    println!("{result}");
}