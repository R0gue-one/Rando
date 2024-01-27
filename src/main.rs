use chrono::{Local, DateTime, Utc, Timelike};

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
     
    for a in 1..10{
        let now = Local::now();
        let nano_sec = u64::from(now.nanosecond());
        let sec = now.second();
        // print_type_of(&nano_sec);
        println!(" sec: {}, nanosecond: {}", sec, nano_sec);
        
        let random_no = rng(nano_sec, sec);
        println!("{}", random_no);
    }

    let random = rng(34102765,20);
    println!("{}",random);
}

fn rng(seed: u64, rounds: u32) -> u64{
    let mut r_no: u64 = seed;
    
    for x in 1..rounds {
        if r_no == 0{
            println!("0 mil gya in round: {}",x);
        }
        r_no = (r_no*r_no)%1_000_000;
    }
    // println!("Random no: {}", r_no);
    r_no

}
