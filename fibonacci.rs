
fn fibonacci(n: i32)->i32 {
        // F(n) = F(n-1) + F(n-2) where F(0) = 0 and F(1) = 1
        if n <= 1 {
            1
        } else {
            fibonacci(n-1) + fibonacci(n-2)
        }
}

fn main() {
    let value =	fibonacci(10);
    println!("The value is {}", value);
}
