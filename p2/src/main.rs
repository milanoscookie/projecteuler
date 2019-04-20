pub fn main() {
    let limit = 4_000_000;
    let mut a = 0;
    let mut b = 2;
    let mut s = 0;

    while b<=limit{
        a = b;
        b = a + 4 * b;
        s = s + b;
    }

    print!("{:?}", s);
}
