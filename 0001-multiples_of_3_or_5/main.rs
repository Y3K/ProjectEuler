fn main() {
    let limit: i32 = 1000;
    let result: i32 = (0..limit)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum();

    println!("{}", result);
}
