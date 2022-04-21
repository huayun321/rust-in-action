use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1,0); //Creates a Duration that represents 1 second
    let start = Instant::now(); //Accesses time from the system’s clock

    while(Instant::now() - start) < time_limit { //An Instant minus an Instant returns a Duration.
        count += 1;
    }

    println!("{}", count);
}