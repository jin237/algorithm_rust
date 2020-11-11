fn fizz_buzz3() {
    for x in 1 ..= 100 {
        match x % 15 {
            0 => print!("FizzBuzz "),
            3 | 6 | 9 | 12 => print!("Fizz "),
            5 | 10 => print!("Buzz "),
            _ => print!("{} ", x), 
        }
    }
}

fn main() {
    fizz_buzz3();
}
