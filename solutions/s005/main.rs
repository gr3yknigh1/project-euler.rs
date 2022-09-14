// https://projecteuler.net/problem=5
// 2520 is the smallest number that can be divided by each of the numbers from 
// 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of 
// the numbers from 1 to 20?


fn main() {

    let mut base_num: u32 = 2;

    loop {
        let mut is_target = true;
        let num = base_num * 10;

        for div in 1..21 {
            if num % div != 0 {
                is_target = false;
                break;
            }
        }

        if is_target {
            break;
        }

        base_num += 2;
    }

    println!("{}", base_num * 10);

}

