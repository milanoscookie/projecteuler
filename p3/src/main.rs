fn main() {
    println!("{:?}", largest_prime_factor(600_851_475_143))
}

fn is_prime(num: u64) -> bool {
    for i in 2..num/2 {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn largest_prime_factor(num: u64) -> u64 {
    
    let mut i = 2;
    loop {
        if num % i == 0 && is_prime(num / i) {
            return num / i
        }
        i += 1;
    }
}
