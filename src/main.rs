/*
Create a multi-threaded program that generates prime numbers up to a given N.
Use channels to communicate between threads and share the computed prime numbers.
*/
use std::thread;
use std::sync::mpsc;
use clap::Parser;

/// Use cargo run -- --upper-limit N to input a max limit for prime number generation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value="1000",help="Upper limit for prime number generation")]
    upper_limit: u32,
}

/// returns whether given number *n* is prime or not as a boolean.
/// # Arguments
/// * 'n' - Number to evaluate as prime/not prime
/// # Examples
/// is_prime(5)
fn is_prime(n: u32) -> bool {
    // prime numbers can only be numbers greater than 1
    if n <= 1 {
        return false;
    }
    // 2 and 3 are always prime
    if n <= 3 {
        return true;
    }
    // if a number is divisible by 2 or 3, it is immediately not prime
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut k = 1;
    // check whether n is divisible by numbers of the form `6k + 1` or `6k - 1` where *k* is a positive integer
    // it starts with `k = 1` and iterates while the square of *k* is less than *n*
    // the number *n* is not prime if it is divisible by either `6k + 1` or `6k - 1`.
    while (6 * k - 1) * (6 * k - 1) <= n {
        if n % (6 * k - 1) == 0 || n % (6 * k + 1) == 0 {
            return false;
        }
        k += 1;
    }
    true
}

fn main(){
    let args = Args::parse();
    let n = args.upper_limit;
    let (sender, receiver) = mpsc::channel();
    for i in 2..=n {
        let sender = sender.clone();
        thread::spawn(move || {
            if is_prime(i) {
                sender.send(i).unwrap();
            }
        });
    }
    drop(sender);
    let mut primes = Vec::new();

    for p in receiver {
        primes.push(p);
    }

    primes.sort();

    println!("Prime numbers upto {}: {:?}", n, primes);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn identifies_prime_number() {
        let test_n = 59;
        assert_eq!(is_prime(test_n), true);
    }

    #[test]
    fn identifies_non_prime_number() {
        let test_n = 100;
        assert_eq!(is_prime(test_n), false);
    }
}
