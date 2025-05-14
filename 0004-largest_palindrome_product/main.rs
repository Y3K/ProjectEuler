fn is_palindrome(number: i32) -> bool {
    let mut first = number;
    let mut second = 0;
    
    while first > 0 {
       let digit = (first % 10) as i32; 
       second = second * 10 + digit;
       first = (first / 10) as i32;
    }
    
    number == second
}

fn main() {
    let digits = 3;
    let limit = 10_i32.pow(digits);
    let mut largest = 0;
    
    for i in (1..limit).rev() {
        for j in (1..i).rev() {
            let product = i * j;
            
            if product > largest && is_palindrome(product) {
                largest = product;
            }
        }
    }
    
    println!("{}", largest);
}
