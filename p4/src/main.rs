fn main() {
    let mut acc: Vec<u64> = Vec::new();
    for i in 1..999 {
        for j in 1..999 {
            let product = i*j;
            if is_palindrome(product) {
                acc.push(i * j);
            }
        }
    }
    acc.sort();
    println!("{:?}", acc.last());
}

fn is_palindrome(x: u64) -> bool {
    x.to_string() == x.to_string().chars().rev().collect::<String>()
}
