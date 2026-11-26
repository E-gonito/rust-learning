fn main() {
    let mut x = 5;
    {
        x = x * 2;
        println!("The value of inner x is: {x}");
    }
    println!("The value of outer x is: {x}");
}