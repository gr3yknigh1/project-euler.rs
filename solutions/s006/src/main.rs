// The sum of the squares of the first ten natural numbers is,
// 
// 1 ^ 2 + 2 ^ 2 + ... + 10 ^ 2 = 385
//
// The square of the sum of the first ten natural numbers is,
//
// (1 + 2 + ... + 10) ^ 2 = 55 ^ 2 = 3025
//
// Hence the difference between the sum of the squares of the first ten 
// natural numbers and the square of the sum is 3025 - 385 = 2640.
//
// Find the difference between the sum of the squares of the first one 
// hundred natural numbers and the square of the sum.


fn main() {

    let mut sum_of_seq = 0_i32;
    let mut sum_of_sqrt = 0_i32;

    for i in 1..100+1 {
        sum_of_seq += i;
        sum_of_sqrt += i * i;
    }

    let total = sum_of_seq * sum_of_seq - sum_of_sqrt;

    println!("{}", total);

}
