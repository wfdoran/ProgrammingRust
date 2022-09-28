use std::str::FromStr;
use std::env;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        let val = u64::from_str(&arg).expect("error parsing argument");
        numbers.push(val);
    }

    if numbers.len() == 0 {
        eprintln!("Useage: gcd NUMBER NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for val in &numbers[1..] {
        d = gcd(d, *val);
    }

    println!("gcd of {:?} is {}", numbers, d);
}
