use chrono::{Local, DateTime, Utc, Timelike};

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn count_digits(mut num: u64) -> usize {
    if num == 0 {
        return 1; // Special case for 0
    }

    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn main() {
     
    for a in 1..10{
        let now = Local::now();
        let nano_sec = u64::from(now.nanosecond());
        let sec = now.second();
        println!(" sec: {}, nanosecond: {}", sec, nano_sec);
        
        let random_no = rng(nano_sec, sec);
        println!();
        println!("Final: {}, digits: {}", random_no, count_digits(random_no));
        println!();

    }

}

fn rng(seed: u64, rounds: u32) -> u64{
    let mut r_no: u64 = seed;
    
    for x in 1..rounds {
        // if r_no == 0{
        //     println!("0 mil gya in round: {}",x);
        // }
        // println!("{}, {}",r_no*r_no/1000,r_no);
        r_no = (r_no*r_no/1000)%1_000_000_000;
        println!("{}",r_no);
    }
    // println!("Random no: {}", r_no);
    r_no

}
