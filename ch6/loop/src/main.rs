use std::io;
use std::cmp::Ordering;

fn loop_example() {
    let mut count = 0u32;

    // infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");
        
            // skip rest of iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("done");

            // exit loop
            break;
        }
    }
}

fn main() {
    loop_example();
}
