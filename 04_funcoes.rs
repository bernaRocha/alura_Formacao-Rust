fn soma(a:i32, b:i32) -> i32
{
    println!("Soma: {} + {} = {}", a, b, a + b);
    a + b
}

fn main() {
    soma(5, 8);
    soma(9, 9);
}